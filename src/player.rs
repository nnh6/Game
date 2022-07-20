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
	FRAME_TIME,
	INV_TIME,
	GameTextures,
	SPRITE_SCALE,
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

#[derive(Component, Deref, DerefMut)]
pub struct InvincibilityTimer(Timer);
#[derive(Component)]
pub struct Health{
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
#[derive(Component)]
pub struct Direction; // 0 = Left,  1 = Right?

#[derive(Deref, DerefMut)]
pub struct PlayerSheet(Handle<TextureAtlas>);




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedStep;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build (&self, app: &mut App) {
		let every_second = SystemStage::parallel();
		let mut every_frame = SystemStage::parallel();
    //every_second.add_system(check_enemy_collision.run_in_state(GameState::Playing)); //.add_system(update_health.run_in_state(GameState::Playing));
		//every_second.add_system(check_enemy_collision.run_in_state(GameState::Playing));

		every_frame.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					.with_system(move_player)
					.with_system(animate_player)
					.with_system(enter_door)
					.with_system(swing_axe)
					.with_system(update_health)
					.with_system(check_enemy_collision)
					//.with_system(my_fixed_update)  //This tests the frame times for this system, if that ever comes up
					.into()
					); //moving
		app.add_enter_system(GameState::Loading, load_player_sheet)
			.add_enter_system(GameState::Playing, spawn_player)
			.add_enter_system(GameState::Loading, load_health_sheet)
			.add_enter_system(GameState::Playing, spawn_health)
			//.add_system(player_fire_system)
			/*.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					.with_system(move_player)
					.with_system(animate_player)
					.with_system(enter_door)
					.with_system(swing_axe)
					.with_system(update_health) //health sprite
					.into()
			) */
			.add_stage_before(
				CoreStage::Update,
				"FixedStepFrame",
				FixedTimestepStage::from_stage(Duration::from_micros(16667), every_frame) // ~1 frame at 60 fps
					
				
			)
			.add_stage_before(
				CoreStage::Update,
				FixedStep,
				FixedTimestepStage::new(Duration::from_secs(1))
					.with_stage(every_second)
			);
	}
}

/*fn my_fixed_update(info: Res<FixedTimestepInfo>) { //testing timestep
    println!("Fixed timestep duration: {:?} ({} Hz).", info.timestep(), info.rate());
    println!("Overstepped by {:?} ({}%).", info.remaining(), info.overstep() * 100.0);
}**/

fn load_player_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let player_handle = asset_server.load("minerwalk-and-swing.png");
	loading_assets.insert(
		player_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(player_handle.clone_untyped(), &asset_server),
	);

	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(TILE_SIZE), 4, 3);
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
		.insert(InvincibilityTimer(Timer::from_seconds(INV_TIME, false)))
		.insert(Health::new())
		.insert(Player{
			grounded: false,
			y_velocity: -1.0,
			x_velocity: 0.,
		});
}

fn move_player(
	_time: Res<Time>,
	input: Res<Input<KeyCode>>,
	collision: Query<&Transform, (With<Collider>, Without<Player>)>,
	mut player: Query<(&mut Player, &mut Transform)>,
){
	for (mut player, mut transform) in player.iter_mut() {

		if player.grounded && input.pressed(KeyCode::Space) { //changed to "pressed" instead of "just_pressed" because sometimes the jump wasn't working. Now you can hold space to jump when you hit the ground, but this seems acceptable.
			player.y_velocity = JUMP_TIME * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
		}

		player.y_velocity += -25.0 * TILE_SIZE * FRAME_TIME;

		let deltay = player.y_velocity * FRAME_TIME;
		
		let mut deltax = 0.0;

		if input.pressed(KeyCode::A) {
			deltax -= 1. * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
		}

		if input.pressed(KeyCode::D) {
			deltax += 1. * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
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
	input: Res<Input<KeyCode>>,
	mut player: Query<
		(
			&mut Player,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
			&InvincibilityTimer,
			&Health,
			&mut Transform
		),
		With<Player>
	>,
){
	for (player, mut sprite, texture_atlas_handle, mut timer, invTimer,health,mut transform) in player.iter_mut() {
		if input.just_pressed(KeyCode::E){
				let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
				sprite.index = (sprite.index + 1) % (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3);
			}
		let velocity = Vec2::new(player.x_velocity, player.y_velocity);
		if velocity.cmpne(Vec2::ZERO).any() {
			timer.tick(time.delta());
			if !invTimer.finished() && timer.just_finished() && health.health != 100.0{
				let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
				sprite.index = ((sprite.index + 1) % (texture_atlas.textures.len()/3)) + (texture_atlas.textures.len()/3);
			}
			else if timer.just_finished() {
				let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
				sprite.index = (sprite.index + 1) % (texture_atlas.textures.len()/3);
			}

			

			if  player.x_velocity < 0.0 {
				transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
			} else {
				transform.rotation = Quat::default();
			}

		}
	}
}

fn enter_door(
	mut commands: Commands,
	player: Query<&Transform, With<Player>>,
	door: Query<&Transform, With<Door>>,
	input: Res<Input<KeyCode>>,
) {
	for player_transform in player.iter() {
		let door_transform = door.single();
		if input.just_pressed(KeyCode::W) && collide(player_transform.translation, Vec2::splat(50.), door_transform.translation, Vec2::splat(50.)).is_some() {
			info!("door open!");
			commands.insert_resource(NextState(GameState::Credits));
		}
	}
}

pub fn check_enemy_collision(
	mut commands: Commands,
	_enemy_sheet: Res<EnemySheet>,
	enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
	mut player_query: Query<
		(
			Entity, 
			&Transform, 
			&mut Health, 
			&mut InvincibilityTimer,
		),
		(With<Player>,
		Without<Enemy>)
		>,
	//mut health: Query<(&mut TextureAtlasSprite,&Handle<TextureAtlas>,),With<Health>>,
	//texture_atlases: Res<Assets<TextureAtlas>>,
	
) {
	let (player_entity, player_transform, mut player_health, mut inv_timer) = player_query.single_mut();
	//let (mut sprite, texture_atlas_handle) = health.single_mut();

	for enemy_transform in enemy_query.iter() {
		if collide(player_transform.translation, Vec2::splat(50.), enemy_transform.translation, Vec2::splat(50.)).is_some() && inv_timer.finished() {
  				inv_timer.reset(); //reset the invincibility
  				player_health.health -= 20.;
  				//call update health here for more efficiency 
  			
  				//let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
  				//let hs_len : usize = texture_atlas.textures.len() as usize;
  				//let c_health : usize = (player_health.health/10.).round() as usize;
  				//sprite.index = hs_len - c_health; //Use health to determine the index of the health sprite to show
  	
  				info!("{}", player_health.health);
  				if player_health.health <= 0. {
  					//player dies
  					commands.insert_resource(NextState(GameState::GameOver));
  					commands.entity(player_entity).despawn();
  				}
  			}
	}
	inv_timer.tick(Duration::from_secs_f32(FRAME_TIME)); //tick the invincibility timer after we're done checking collision
}

pub fn swing_axe(
	mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
	player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
	input: Res<Input<KeyCode>>,
	mut commands: Commands,
) {

	for player_transform in player_query.iter() {
		for (enemy_entity, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
			let collision = collide(player_transform.translation, Vec2::splat(150.), enemy_transform.translation, Vec2::splat(50.));
			if input.just_pressed(KeyCode::E) && collision.is_some() {
				match collision.unwrap() {
					Collision::Left => {
						enemy_health.health -= 20.;
						info!("{}", enemy_health.health);
						if enemy_health.health <= 0. {
							commands.entity(enemy_entity).despawn();
						}
					}
					Collision::Inside => {
						enemy_health.health -= 20.;
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

	let hp_atlas = TextureAtlas::from_grid(hp_handle, Vec2::new(300., 32.5), 2, 6);
	let hp_atlas_handle = texture_atlases.add(hp_atlas);

	commands.insert_resource(HealthAtlas(hp_atlas_handle));
	
}
 
fn spawn_health(
	mut commands: Commands,
	health_sheet: Res<HealthAtlas>, //healthsheet instead
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: health_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(-(WIN_W/2.) + (TILE_SIZE * 1.8)  , (WIN_H/2.) - (TILE_SIZE * 0.3), 900.),
			..default()
		})
		.insert(Health::new());

}

/* 
fn update_health(
	player: Query<(Entity, &Health, &Transform), With<Player>>, 
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut health: Query<
		(
			&Health, //&mut Health,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
		),
		With<Health>
	>,
){//not completed
	if(!player.is_empty()){
	let (player, p_health, transform) = player.single();

	for (health, mut sprite, texture_atlas_handle) in health.iter_mut() {
		let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			if sprite.index < texture_atlas.textures.len() as usize{
				let hs_len : f32 = 10.0;//texture_atlas.textures.len() as f32;
				let c_health : f32 = (p_health.health/10.);// % (texture_atlas.textures.len() as f32); //(player_health.health/10.).round() as f32;
				//info!("{}", (hs_len - c_health).round() as usize); //checking if index is correct
				
				sprite.index = (sprite.index + (hs_len - c_health).round() as usize) % texture_atlas.textures.len() as usize; //Use health to determine the index of the health sprite to show
			}
		}
	}
}*/


fn update_health(
	//texture_atlases: Res<Assets<TextureAtlas>>,
	mut health: Query<&mut TextureAtlasSprite, (With<Health>,Without<Player>,Without<Enemy>)>,
	mut player: Query<&Health, With<Player>>
){//not completed
	
	let mut sprite = health.single_mut();
	let player = player.single_mut();
	//let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
	//let hs_len : usize = texture_atlas.textures.len() as usize;
	sprite.index = if player.health != 100.0 {
		((100.0-player.health)/10.0).round() as usize
	}else{
		0_usize
	}
	//Use health to determine the index of the health sprite to show

} 

fn player_fire_system(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	game_textures: Res<GameTextures>,
	query: Query<&Transform, With<Player>>,
){
	if let Ok(player_tf) = query.get_single(){
		if kb.just_pressed(KeyCode::F){
			let (x,y) = (player_tf.translation.x, player_tf.translation.y);
			commands.spawn_bundle(SpriteBundle{
				texture: game_textures.player_bolt.clone(),
				transform: Transform{
					translation:Vec3::new(x,y,0.),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			});
		}
	}
}