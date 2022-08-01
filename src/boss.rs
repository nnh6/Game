use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use std::f32::consts::PI;

use crate::enemy::Enemy;
use crate::{
	GameState,
	TIME_STEP,
	BASE_SPEED,
	TILE_SIZE,
	level::Collider,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
    player::{
		Player
	},
	FRAME_TIME
};

#[derive(Component)]
pub struct Boss{
	pub health: f32,
	pub y_velocity: f32,
	pub x_velocity: f32,
	pub y_accel: f32,
	pub last_move: f32,
	pub turtled: bool,
	pub path: Vec3
}

#[derive(Deref, DerefMut)]
pub struct BossSheet(Handle<TextureAtlas>);

pub struct BossPlugin;
impl Plugin for BossPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_boss_sheet)
		.add_system(boss_movement_system)
		.add_system(boss_animate);
		
	}
}

fn load_boss_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let boss_handle = asset_server.load("turtle.png");
	loading_assets.insert(
		boss_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(boss_handle.clone_untyped(), &asset_server),
	);

	let boss_atlas = TextureAtlas::from_grid(boss_handle, Vec2::splat(320.), 2, 2);
	let boss_atlas_handle = texture_atlases.add(boss_atlas);
	
	commands.insert_resource(BossSheet(boss_atlas_handle));
}



fn boss_movement_system(
	time: Res<Time>, 
	mut query: Query<(&mut Transform,&mut Boss), (With<Boss>,Without<Player>)>,
	mut player: Query<&mut Transform, With<Player>>,
	collision: Query<&Transform, (With<Collider>, Without<Player>,Without<Boss>)>,
){
	let now = time.seconds_since_startup() as f32;
	for mut player_transform in player.iter_mut() {
		for (mut transform,mut boss) in query.iter_mut(){
			//current position
			
			let (x_org, y_org) = (transform.translation.x, transform.translation.y);
			let (px_org,py_org) = (player_transform.translation.x, player_transform.translation.y);
			let path: Vec3 = Vec3::new(x_org- px_org,y_org-px_org,900.0);
			
			if check_tile_collision(path, &collision) && boss.last_move < (now + FRAME_TIME){ //if player in LOS run towards player
				boss.last_move = now;
				let deltax = {
					if path.x>0.0 {
						-10.0 * FRAME_TIME
					}else{
						10.0 * FRAME_TIME
					}
				};
				
				let target = transform.translation + Vec3::new(deltax, 0., 0.);
				if check_tile_collision(target, &collision){
					transform.translation = target;
					boss.x_velocity = deltax;
				}else{
					boss.y_accel = 1000.0;
				}
			
				
				boss.y_velocity += -25.0 * TILE_SIZE * FRAME_TIME;

				let deltay = boss.y_velocity * FRAME_TIME;
				let target = transform.translation + Vec3::new(0., deltay, 0.);
				if check_tile_collision(target, &collision){
					transform.translation = target;
					
				}else{
					boss.y_velocity = 0.0;
				}
				if path.y < transform.translation.y || boss.path == Vec3::new(0.,0.,0.){
					boss.path = path;
				}
				
			}
			else if boss.last_move < (now + FRAME_TIME) { //
				
				boss.last_move = now;
				let deltax = {
					if boss.path.x>0.0 {
						-10.0 * FRAME_TIME
					}else{
						10.0 * FRAME_TIME
					}
				};
				
				let target = transform.translation + Vec3::new(deltax, 0., 0.);
				if check_tile_collision(target, &collision){
					transform.translation = target;
					boss.x_velocity = deltax;
				}else{
					boss.y_accel = 1000.0;
				}
				
				boss.y_velocity = boss.y_accel* FRAME_TIME;
				boss.y_velocity += -25.0 * TILE_SIZE * FRAME_TIME;

				let deltay = boss.y_velocity * FRAME_TIME;
				let target = transform.translation + Vec3::new(0., deltay, 0.);
				if check_tile_collision(target, &collision){
					transform.translation = target;
					
				}else{
					boss.y_velocity = 0.0;
				}
			}
		}
	}
}

fn boss_animate(
    texture_atlases: Res<Assets<TextureAtlas>>,
	mut boss: Query<
		(
			&mut Boss,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut Transform
		),
		With<Boss>
	>,
){
    for (mut boss,mut sprite, texture_atlas_handle, mut transform) in boss.iter_mut() {
        if boss.health <= 50.0{
           sprite.index = 1;
        }
		//if bomb near (turtled = true; index =2)
		if  boss.x_velocity < 0.0 {
			transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
		} else {
			transform.rotation = Quat::default();
		}
    }
}

fn check_tile_collision(
	pos: Vec3,
	wall_collide: &Query<&Transform, (With<Collider>, Without<Player>,Without<Boss>)>
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