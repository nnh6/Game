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
	level::Brick,
	level::Map,
	enemy::{
		Enemy,
		EnemySheet
	},
	level::BombItem,
	level::HealthItem,
	boss::Boss,
};

#[derive(Component)]
pub struct Player{
	y_velocity: f32,
	x_velocity: f32,
	grounded: bool,
	bombs: f32,
	swing: bool,
}

//BOMB
#[derive(Component)]
pub struct Bomb{
	y_velocity: f32,
	x_velocity: f32,
	//grounded: bool,
}

#[derive(Component)]
pub struct Fragment{
	y_velocity: f32,
	x_velocity: f32,
	index: i32,
}

#[derive(Deref, DerefMut)]
pub struct BombSheet(Handle<TextureAtlas>);
//BOMB^

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct SwingTimer(Timer);

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

#[derive(Deref, DerefMut)]
pub struct InventoryAtlas(Handle<TextureAtlas>);

#[derive(Deref, DerefMut)]
pub struct CountAtlas(Handle<TextureAtlas>);

#[derive(Component)]
pub struct InventoryCount{
	b_count: f32,
}

impl InventoryCount {
	pub fn new() -> Self {
		Self {b_count: 3.}
	}
}

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

#[derive(Deref, DerefMut)]
pub struct FragmentSheet(Handle<TextureAtlas>);



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
					.with_system(animate_swing)
					.with_system(update_health)
					.with_system(check_enemy_collision)
					//BOMB
					.with_system(check_player_bomb_pickup_collision)
					.with_system(check_player_health_pickup_collision)
					.with_system(animate_bomb)
					.with_system(bomb_throw)
					.with_system(enter_new_room)
					.with_system(damage_walls)
					.with_system(spawn_fragment)
					.with_system(fragment_movement)
					.with_system(update_count)
					//.with_system(my_fixed_update)  //This tests the frame times for this system, if that ever comes up
					.into()
					); //moving
		app.add_enter_system(GameState::Loading, load_player_sheet)
			.add_enter_system(GameState::Playing, spawn_player)
			.add_enter_system(GameState::Loading, load_health_sheet)
			.add_enter_system(GameState::Playing, spawn_health)
			//BOMB
			.add_enter_system(GameState::Loading, load_bomb_sheet)
			//.add_enter_system(GameState::Playing, spawn_bomb)
			.add_enter_system(GameState::Loading, load_fragment_sheet)
			.add_enter_system(GameState::Loading, load_inventory_sheet)
			.add_enter_system(GameState::Playing, spawn_inventory)
			.add_enter_system(GameState::Loading, load_count_sheet)
			.add_enter_system(GameState::Playing, spawn_count)
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
	mapq: Query<&Map>
){
	let map = mapq.single();
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: map.player_spawn,
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		.insert(SwingTimer(Timer::from_seconds(0.12, true)))
		.insert(Velocity::new())
		.insert(InvincibilityTimer(Timer::from_seconds(INV_TIME, false)))
		.insert(Health::new())
		.insert(Player{
			grounded: false,
			y_velocity: -1.0,
			x_velocity: 0.,
			bombs: 3., //starting with 3 bombs for testing
			swing: false,
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
		// if input.just_pressed(KeyCode::E){
		// 		let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
		// 		sprite.index = (sprite.index + 1) % (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3);
		// 	}
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
		//let door_transform = door.single();
		for door_transform in door.iter()
		{
		if input.just_pressed(KeyCode::W) && collide(player_transform.translation, Vec2::splat(50.), door_transform.translation, Vec2::splat(50.)).is_some() {
			info!("door open!");
			commands.insert_resource(NextState(GameState::Credits));
		}
		}
	}
}

pub fn check_enemy_collision(
	mut commands: Commands,
	_enemy_sheet: Res<EnemySheet>,
	enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
	boss_query: Query<&Transform, (With<Boss>, Without<Player>)>,
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

	for boss_transform in boss_query.iter() {
		if collide(player_transform.translation, Vec2::splat(50.), boss_transform.translation, Vec2::new(260.,100.)).is_some() && inv_timer.finished() {
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
						enemy_health.health -= 25.;
						info!("{}", enemy_health.health);
						if enemy_health.health <= 0. {
							commands.entity(enemy_entity).despawn();
						}
					}
					Collision::Inside => {
						enemy_health.health -= 25.;
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

fn animate_swing( //not complete yet
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	input: Res<Input<KeyCode>>,
	mut player: Query<
		(
			&mut Player,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut SwingTimer,
			&mut Transform
		),
		With<Player>
	>,
	mut commands: Commands,
){
	//info!("tick");
	//let (entity, mut bomb, mut sprite, texture_atlas_handle, mut timer) = bomb.single_mut();
	for (mut player, mut sprite, texture_atlas_handle, mut timer, mut transform) in player.iter_mut() {
		if player.x_velocity == 0.0 && (input.just_pressed(KeyCode::E) || player.swing){
			
			if !player.swing || sprite.index < 8{
				//info!("setting to 8");
				sprite.index = 8;
			}
			player.swing = true;
			timer.tick(time.delta());
			
			if timer.just_finished() {
				let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
				sprite.index = (sprite.index + 1);// % (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3)+ (texture_atlas.textures.len()/3);
				//info!("{:?}", sprite.index);
				if sprite.index == (texture_atlas.textures.len() - 1){
					player.swing = false;
					//sprite.index = 0;
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
	mut health: Query<&mut TextureAtlasSprite, (With<Health>,Without<Player>,Without<Enemy>,Without<Boss>,Without<Brick>)>,
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

/*

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
}*/

fn bomb_throw(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	//game_textures: Res<GameTextures>,
	query: Query<&Transform, With<Player>>,
	bomb_sheet: Res<BombSheet>,
	mut player: Query<&mut Player, With<Player>>,
){
	if let Ok(player_tf) = query.get_single(){
		if kb.just_pressed(KeyCode::F){
			let (x,y) = (player_tf.translation.x, player_tf.translation.y);
			let mut player = player.single_mut();
			//info!("Bomb dropped");
	if player.bombs > 0. {
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: bomb_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(x, (y- (TILE_SIZE * 0.25)), 900.), 
			//for throw, change the velocities for projectile/parabola trajectory and have spawn from player y (center of player sprite)
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		//.insert(Velocity::new())
		.insert(Bomb{
			//grounded: false,
			y_velocity: 0., //-1.0,
			x_velocity: 0.,
		});
		player.bombs = player.bombs - 1.;
		info!("bombs left: {}", player.bombs);
	}
		}
	}
}

//BOMB_WEAPON/////////////////
fn load_bomb_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let bomb_handle = asset_server.load("bomb_boom.png");
	loading_assets.insert(
		bomb_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bomb_handle.clone_untyped(), &asset_server),
	);

	let bomb_atlas = TextureAtlas::from_grid(bomb_handle, Vec2::splat(35.), 6, 1);
	let bomb_atlas_handle = texture_atlases.add(bomb_atlas);
	
	commands.insert_resource(BombSheet(bomb_atlas_handle));
}

fn spawn_bomb(
	mut commands: Commands,
	bomb_sheet: Res<BombSheet>,
){
	
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: bomb_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			//transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
			transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		//.insert(Velocity::new())
		.insert(Bomb{
			//grounded: false,
			y_velocity: 0., //-1.0,
			x_velocity: 0.,
		});

}

pub fn damage_walls(
	mut wall_query: Query<(Entity, &Transform, &mut Health), (With<Brick>, Without<Player>, Without<Enemy>)>,
	player_query: Query<&Transform, (With<Player>, Without<Enemy>, Without<Brick>)>,
	input: Res<Input<KeyCode>>,
	mut commands: Commands,
) {
	for player_transform in player_query.iter() {
		for (wall_entity, wall_transform, mut wall_health) in wall_query.iter_mut() {
			let collision = collide(player_transform.translation, Vec2::new(100., 60.), wall_transform.translation, Vec2::splat(80.));
			if input.just_pressed(KeyCode::E) && collision.is_some() {
				match collision.unwrap() {
					Collision::Left => {
						wall_health.health -= 20.;
						info!("{}", wall_health.health);
						//info!("Left");
						if wall_health.health <= 0. {
							commands.entity(wall_entity).despawn();
						}
					}
					Collision::Right => {
						wall_health.health -= 20.;
						info!("{}", wall_health.health);
						//info!("Right");
						if wall_health.health <= 0. {
							commands.entity(wall_entity).despawn();
						}
					}
					Collision::Top => {
						if (input.just_pressed(KeyCode::S)) {
							wall_health.health -= 20.;
							info!("{}", wall_health.health);
							//info!("Right");
							if wall_health.health <= 0. {
								commands.entity(wall_entity).despawn();
							}
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

fn animate_bomb( //not complete yet
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut bomb: Query<
		(
			Entity,
			&mut Bomb,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
			
		),
		With<Bomb>
	>,
	mut commands: Commands,
){
	//info!("tick");
	//let (entity, mut bomb, mut sprite, texture_atlas_handle, mut timer) = bomb.single_mut();
	for (entity, bomb, mut sprite, texture_atlas_handle, mut timer) in bomb.iter_mut() {
		
		//let ground = bomb.grounded;
		//info!("bomb");
		timer.tick(time.delta());

		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1);// % texture_atlas.textures.len();

			if sprite.index >= texture_atlas.textures.len(){
				commands.entity(entity).despawn();
			}
		
		}
	}
} 

fn check_player_bomb_pickup_collision(
	mut commands: Commands,
	mut player_query: Query<
		(
			&Transform, 
			&mut Player
		),
			(
				With<Player>, 
				Without<BombItem>
			)>,
	mut bomb_query: Query<
		(
			Entity, 
			&Transform,
		),
		(With<BombItem>,
		Without<Player>)
		>,
) {
	

	for (bomb_entity, bomb_transform)  in bomb_query.iter(){
		//info!("bp check"); 
		let (player_transform, mut player) = player_query.single_mut();
		if collide(player_transform.translation, Vec2::splat(50.), bomb_transform.translation, Vec2::splat(50.)).is_some() {
				info!("bomb picked up");
				player.bombs += 3.0;

				commands.entity(bomb_entity).despawn();
		}
	}
}//bomb collision if touch a neutral bomb, collect it

fn enter_new_room(
	mut player: Query<&mut Transform,With<Player>>,
	mut mapq: Query<&mut Map>,
	mut commands: Commands,
){
	//move player
	//update map coords
	//despawn?
	//enter loading state
	let mut map = mapq.single_mut();
	for mut player_transform in player.iter_mut() {
		if player_transform.translation.y >= WIN_H/2.0-TILE_SIZE/2.0 {
			player_transform.translation.y = -WIN_H/2.0+TILE_SIZE/2.0+TILE_SIZE;
			map.player_spawn = *player_transform;
			map.y_coords += 1 as usize;
			commands.insert_resource(NextState(GameState::Traverse));
			info!("newroom up");
		}
		else if player_transform.translation.x <= -WIN_W/2.0+TILE_SIZE/2.0{
			player_transform.translation.x = WIN_W/2.0+TILE_SIZE/2.0-TILE_SIZE;
			map.player_spawn = *player_transform;
			map.x_coords -= 1 as usize;
			commands.insert_resource(NextState(GameState::Traverse));
			info!("newroom left");
		}
		else if player_transform.translation.x >= WIN_W/2.0-TILE_SIZE/2.0 {
			player_transform.translation.x = -WIN_W/2.0+TILE_SIZE/2.0+TILE_SIZE;
			map.player_spawn = *player_transform;
			map.x_coords += 1 as usize;
			commands.insert_resource(NextState(GameState::Traverse));
			info!("newroom right");
		}
		else if player_transform.translation.y <= -WIN_H/2.0+TILE_SIZE/2.0 {
			player_transform.translation.y = WIN_H/2.0-TILE_SIZE/2.0-TILE_SIZE;
			map.player_spawn = *player_transform;
			map.y_coords -= 1 as usize;
			commands.insert_resource(NextState(GameState::Traverse));
			info!("newroom down");
		}
	}
}


fn check_player_health_pickup_collision(
	mut commands: Commands,
	mut player_query: Query<
		(
			&Transform, 
			&mut Player,
			&mut Health,
		),
			(
				With<Player>, 
				Without<BombItem>
			)>,
	mut hp_query: Query<
		(
			Entity, 
			&Transform,
		),
		(With<HealthItem>,
		Without<Player>)
		>,
) {
	

	for (hp_entity, health_transform)  in hp_query.iter(){
		//info!("bp check"); 
		let (player_transform, mut player, mut health) = player_query.single_mut();
		if collide(player_transform.translation, Vec2::splat(50.), health_transform.translation, Vec2::splat(50.)).is_some() {
				//info!("bomb picked up");
				health.health = 100.0;
				commands.entity(hp_entity).despawn();
		}
	}
}
//todo find valid ground

//bomb collision if touch a neutral bomb, collect it

fn load_fragment_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let fragment_handle = asset_server.load("fragment.png");
	loading_assets.insert(
		fragment_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(fragment_handle.clone_untyped(), &asset_server),
	);

	let fragment_atlas = TextureAtlas::from_grid(fragment_handle, Vec2::splat(20.), 1, 1);
	let fragment_atlas_handle = texture_atlases.add(fragment_atlas);
	
	commands.insert_resource(FragmentSheet(fragment_atlas_handle));
}

fn spawn_fragment(
	mut commands: Commands,
	input: Res<Input<KeyCode>>,
	fragment_sheet: Res<FragmentSheet>,
	query: Query<(Entity, &Transform), (With<Bomb>,Without<BombItem>, Without<Player>, Without<Enemy>, Without<Brick>,Without<Fragment>)>,
){	
	//if input.pressed(KeyCode::F){
		for (bomb_entity, bomb_tf) in query.iter() {
		
	//	for i in 0..8{
			//if input.just_pressed(KeyCode::F){
			
			
				//info!("found bomb");
			

				let (x,y) = (bomb_tf.translation.x, bomb_tf.translation.y);

				commands
					.spawn_bundle(SpriteSheetBundle {
						texture_atlas: fragment_sheet.clone(),
						sprite: TextureAtlasSprite {
							index: 0,
							..default()
						},
						//transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
						transform: Transform::from_xyz(x, y, -1.),
						..default()
					})
					.insert(AnimationTimer(Timer::from_seconds(1., true)))
					.insert(Velocity::new())
					.insert(Fragment{
						
						y_velocity: 0., //-1.0,
						x_velocity: 0.,
						index: 0,
					});
					//info!("{}", i);
			//}
		}
	//}
}

fn fragment_movement(
	mut commands: Commands,
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	collision: Query<&Transform, (With<Collider>, Without<Fragment>, Without<Bomb>,Without<BombItem>, Without<Player>, Without<Enemy>, Without<Brick>)>,
	mut fragment: Query<(Entity, &mut Fragment, &mut Transform, &mut AnimationTimer), (With<Fragment>, Without<Bomb>,Without<BombItem>, Without<Player>, Without<Enemy>, Without<Brick>)>,){

	let mut anim_complete = false;
	for (entity, mut fragment, mut transform, mut timer) in fragment.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() || anim_complete{
			anim_complete = true;
			let (x,y) = (transform.translation.x, transform.translation.y);
			transform.translation = Vec3::new(x, y, 900.);
			let mut deltax = 0.0;
			let mut deltay = 0.0;
			
			if fragment.index == 0{
				deltax = -0.5;
				deltay = 0.5;
			}

			if fragment.index == 1{
				deltay = 0.5;
			}

			if fragment.index == 2{
				deltax = 0.5;
				deltay = 0.5;
			}

			if fragment.index == 3{
				deltax = 0.5;
			}

			if fragment.index == 4{
				deltax = 0.5;
				deltay = -0.5;
			}

			if fragment.index == 5{
				deltay = -0.5;
			}

			if fragment.index == 6{
				deltax = -0.5;
				deltay = -0.5
			}

			if fragment.index == 7{
				deltax = -0.5;		
			}

			deltax = deltax * TILE_SIZE * FRAME_TIME * 100.;
			deltay = deltay  * TILE_SIZE * FRAME_TIME * 100.;

			fragment.x_velocity = deltax;
			let target = transform.translation + Vec3::new(deltax, 0., 0.);
			if check_tile_collision_frag(target, &collision){
				transform.translation = target;
			}else{
				commands.entity(entity).despawn();
			}
			fragment.y_velocity = deltay;
			let target = transform.translation + Vec3::new(0., deltay, 0.);
			if check_tile_collision_frag(target, &collision){
				transform.translation = target;
			}else{
				commands.entity(entity).despawn();
			}

		}
		//commands.entity(entity).despawn();
	}
}

fn check_tile_collision_frag(
	pos: Vec3,
	wall_collide: &Query<&Transform, (With<Collider>, Without<Fragment>, Without<Bomb>, Without<BombItem>, Without<Player>, Without<Enemy>, Without<Brick>)>
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

// if player.grounded && input.pressed(KeyCode::Space) { //changed to "pressed" instead of "just_pressed" because sometimes the jump wasn't working. Now you can hold space to jump when you hit the ground, but this seems acceptable.
// 			player.y_velocity = JUMP_TIME * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
// 		}

// 		player.y_velocity += -25.0 * TILE_SIZE * FRAME_TIME;

// 		let deltay = player.y_velocity * FRAME_TIME;
		
// 		let mut deltax = 0.0;

// 		if input.pressed(KeyCode::A) {
// 			deltax -= 1. * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
// 		}

// 		if input.pressed(KeyCode::D) {
// 			deltax += 1. * PLAYER_SPEED * TILE_SIZE * FRAME_TIME;
// 		}
// 		player.x_velocity = deltax;
// 		let target = transform.translation + Vec3::new(deltax, 0., 0.);
// 		if check_tile_collision(target, &collision){
// 			transform.translation = target;
// 		}

// 		let target = transform.translation + Vec3::new(0., deltay, 0.);
// 		if check_tile_collision(target, &collision){
// 			transform.translation = target;
// 			player.grounded = false;
// 		}else{
// 			player.y_velocity = 0.0;
// 			player.grounded = true;
// 		}

//BOMB INVENTORY
fn load_inventory_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
){
	let ib_handle = asset_server.load("inventory_bombs_small.png");
	
	loading_assets.insert(
		ib_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(ib_handle.clone_untyped(), &asset_server),
	);

	let ib_atlas = TextureAtlas::from_grid(ib_handle, Vec2::new(150., 32.5), 1, 1);
	let ib_atlas_handle = texture_atlases.add(ib_atlas);

	commands.insert_resource(InventoryAtlas(ib_atlas_handle));
}

fn spawn_inventory(
	mut commands: Commands,
	inventory_sheet: Res<InventoryAtlas>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: inventory_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz((WIN_W/2.) - (TILE_SIZE * 1.8) , (WIN_H/2.) - (TILE_SIZE * 0.3), 999.),
			..default()
		});
		//.insert();
}

fn load_count_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
){
	let c_handle = asset_server.load("count_99.png");
	
	loading_assets.insert(
		c_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(c_handle.clone_untyped(), &asset_server),
	);

	let c_atlas = TextureAtlas::from_grid(c_handle, Vec2::new(80., 80.), 10, 10);
	let c_atlas_handle = texture_atlases.add(c_atlas);

	commands.insert_resource(CountAtlas(c_atlas_handle));
}

fn spawn_count(
	mut commands: Commands,
	c_sheet: Res<CountAtlas>, //healthsheet instead
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: c_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz((WIN_W/2.) - (TILE_SIZE * 0.6) , (WIN_H/2.) - (TILE_SIZE * 0.35), 999.),
			..default()
		})
		.insert(InventoryCount::new());
}

fn update_count(
	//texture_atlases: Res<Assets<TextureAtlas>>,
	mut count: Query<&mut TextureAtlasSprite, (With<InventoryCount>, Without<Health>,Without<Player>,Without<Enemy>,Without<Boss>,Without<Brick>)>,
	mut player: Query<&InventoryCount, With<Player>>
){//not completed
	//let mut sprite = count.single_mut();
	//let player = player.single_mut();
	//let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
	//let hs_len : usize = texture_atlas.textures.len() as usize;
	//sprite.index = (player.b_count).round() as usize;
	//Use health to determine the index of the health sprite to show
}