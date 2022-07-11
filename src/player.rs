use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::convert::From;
use std::time::Duration;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use crate::{
	WIN_W,
	WIN_H,
	TILE_SIZE,
	ANIM_TIME,
	PLAYER_SPEED,
	JUMP_TIME,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
	level::Door,
	level::Collider,
	enemy::{
		Enemy,
		EnemySheet
	},
};

#[derive(Component)]
pub struct Player{
	y_velocity: f32,
	x_velocity: f32,
	grounded: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Health {
	health: f32,
}

impl Health {
	pub fn new() -> Self {
		Self {health: 100.}
	}
}

#[derive(Deref, DerefMut)]
pub struct HealthAtlas(Handle<TextureAtlas>);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity {
	velocity: Vec2,
}

impl Velocity {
	fn new() -> Self {
		Self {velocity: Vec2::splat(0.)}
	}
}

impl From<Vec2> for Velocity {
	fn from(velocity: Vec2) -> Self {
		Self {velocity}
	}
}

#[derive(Deref, DerefMut)]
pub struct PlayerSheet(Handle<TextureAtlas>);

#[derive(Component,Deref, DerefMut)]
pub struct JumpTimer(Timer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedStep;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build (&self, app: &mut App) {
		let mut every_second = SystemStage::parallel();
		every_second.add_system(check_enemy_collision.run_in_state(GameState::Playing));
		app.add_enter_system(GameState::Loading, load_player_sheet)
			.add_enter_system(GameState::Playing, spawn_player)
			.add_enter_system(GameState::Loading, load_health_sheet)
			.add_enter_system(GameState::Playing, spawn_health)
			.add_system(move_player.run_in_state(GameState::Playing).label("move_player"))
			.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					.after("move_player")
					.with_system(animate_player)
					.with_system(enter_door)
					.with_system(swing_axe)
					.into()
			)
			.add_stage_before(
				CoreStage::Update,
				FixedStep,
				FixedTimestepStage::new(Duration::from_secs(1))
					.with_stage(every_second)
			);
	}
}

fn load_player_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let player_handle = asset_server.load("walking-resize.png");
	loading_assets.insert(
		player_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(player_handle.clone_untyped(), &asset_server),
	);

	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(TILE_SIZE), 4, 1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	
	commands.insert_resource(PlayerSheet(player_atlas_handle));
}

fn spawn_player(
	mut commands: Commands,
	player_sheet: Res<PlayerSheet>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(-400., -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		.insert(Velocity::new())
		.insert(JumpTimer(Timer::from_seconds(JUMP_TIME, false)))
		.insert(Health::new())
		.insert(Player{
			grounded: false,
			y_velocity: -1.0,
			x_velocity: 0.,
		});
}

fn move_player(
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	collision: Query<&Transform, (With<Collider>, Without<Player>)>,
	mut player: Query<(&mut Player, &mut Transform)>,
){
	let (mut player, mut transform) = player.single_mut();

	if player.grounded && input.just_pressed(KeyCode::Space) { //starts jump timer
        player.y_velocity += JUMP_TIME * PLAYER_SPEED * TILE_SIZE * time.delta_seconds();
	}

	player.y_velocity += -24.0 * TILE_SIZE * time.delta_seconds();

	let deltay = player.y_velocity * time.delta_seconds();
	
	let mut deltax = 0.0;

	if input.pressed(KeyCode::A) {
		deltax -= 1. * PLAYER_SPEED * TILE_SIZE * time.delta_seconds();
	}

	if input.pressed(KeyCode::D) {
		deltax += 1. * PLAYER_SPEED * TILE_SIZE * time.delta_seconds();
	}
	player.x_velocity = deltax;
	let target = transform.translation + Vec3::new(deltax, 0., 0.);
	if check_tile_collision(target, &collision){
		transform.translation = target;
	}

	let target = transform.translation + Vec3::new(0., deltay, 0.);
	if check_tile_collision(target, &collision){
		transform.translation = target;
		player.grounded = false;
	}else{
		player.y_velocity = 0.0;
		player.grounded = true;
	}
}

fn check_tile_collision(
	pos: Vec3,
	wall_collide: &Query<&Transform, (With<Collider>, Without<Player>)>
) -> bool{
	for wall in wall_collide.iter(){
		let collision = collide(
			pos,
			Vec2::splat(TILE_SIZE * 0.9),
			wall.translation,
			Vec2::splat(TILE_SIZE)
		);
		if collision.is_some(){
			return false;
		}
	}	
	true
}

fn animate_player(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut player: Query<
		(
			&mut Player,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
		),
		With<Player>
	>,
){
	let (player, mut sprite, texture_atlas_handle, mut timer) = player.single_mut();
	let velocity = Vec2::new(player.x_velocity, player.y_velocity);
	if velocity.cmpne(Vec2::ZERO).any() {
		timer.tick(time.delta());

		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

fn enter_door(
	mut commands: Commands,
	player: Query<&Transform, With<Player>>,
	door: Query<&Transform, With<Door>>,
	input: Res<Input<KeyCode>>,
) {
	let player_transform = player.single();
	let door_transform = door.single();
	if input.just_pressed(KeyCode::W) && collide(player_transform.translation, Vec2::splat(50.), door_transform.translation, Vec2::splat(50.)).is_some() {
 			info!("door open!");
 			commands.insert_resource(NextState(GameState::Credits));
 		}
	
}

pub fn check_enemy_collision(
	_enemy_sheet: Res<EnemySheet>,
	enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
	mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>
) {
	let (player_transform, mut player_health) = player_query.single_mut();
	for enemy_transform in enemy_query.iter() {
		if collide(player_transform.translation, Vec2::splat(50.), enemy_transform.translation, Vec2::splat(50.)).is_some() {
			player_health.health = player_health.health - 20.;
			info!("{}", player_health.health);
		}
	}
}

pub fn swing_axe(
	mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
	player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
	input: Res<Input<KeyCode>>,
	mut commands: Commands,
) {
	let player_transform = player_query.single();
	for (enemy_entity, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
		let collision = collide(player_transform.translation, Vec2::splat(150.), enemy_transform.translation, Vec2::splat(50.));
		if input.just_pressed(KeyCode::E) && collision.is_some() {
			match collision.unwrap() {
				Collision::Left => {
					enemy_health.health = enemy_health.health - 20.;
					info!("{}", enemy_health.health);
					if enemy_health.health <= 0. {
						commands.entity(enemy_entity).despawn();
					}
				}
				Collision::Inside => {
					enemy_health.health = enemy_health.health - 20.;
					info!("{}", enemy_health.health);
					if enemy_health.health <= 0. {
						commands.entity(enemy_entity).despawn();
					}
				}
				_ => {
					//nothing
				}
			}
		}
	}
}

fn load_health_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
){
	
	let hp_handle = asset_server.load("Health_Hearts_Small.png");
	loading_assets.insert(
		hp_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(hp_handle.clone_untyped(), &asset_server),
	);

	let hp_atlas = TextureAtlas::from_grid(hp_handle, Vec2::new(300., 35.), 2, 6);
	let hp_atlas_handle = texture_atlases.add(hp_atlas);

	commands.insert_resource(HealthAtlas(hp_atlas_handle));
	
}
 
fn spawn_health(
	mut commands: Commands,
	health_sheet: Res<HealthAtlas>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: health_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(-(WIN_W/2.) + (TILE_SIZE * 1.55)  , (WIN_H/2.) - (TILE_SIZE * 0.3), 900.),
			..default()
		});

}

/*
fn update_health(
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut health: Query<
		(
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
		),
		With<Health>
	>,
){//not completed
	let (mut sprite, texture_atlas_handle) = health.single_mut();
	let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
	let hs_len : usize = texture_atlas.textures.len() as usize;
	let c_health : usize = (HEALTH/10.).round() as usize;
	sprite.index = hs_len - c_health; //Use health to determine the index of the health sprite to show
} */