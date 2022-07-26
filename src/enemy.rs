use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use std::f32::consts::PI;

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
};

#[derive(Component)]
pub struct Enemy;

#[derive(Deref, DerefMut)]
pub struct EnemySheet(Handle<TextureAtlas>);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_enemy_sheet)
		.add_system(enemy_movement_system);
		
	}
}

fn load_enemy_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let enemy_handle = asset_server.load("bat.png");
	loading_assets.insert(
		enemy_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(enemy_handle.clone_untyped(), &asset_server),
	);

	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2::splat(70.), 25, 25);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	
	commands.insert_resource(EnemySheet(enemy_atlas_handle));
}

fn check_tile_collision(
	pos: Vec3,
	wall_collide: &Query<&Transform, (With<Collider>, Without<Enemy>)>
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

fn enemy_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Enemy>>){
	let now = time.seconds_since_startup() as f32;
	for mut transform in query.iter_mut(){
		//current position
		let (x_org, y_org) = (transform.translation.x, transform.translation.y);

		//max distance
		let max_distance = TIME_STEP * BASE_SPEED;

		//fixtures (hardcoded)
		let dir: f32 = -1.; //1 counter clockwise, -1 clockwise
		let (x_pivot, y_pivot) = (0., 0.);
		let (x_radius, y_radius) = (200., 130.);

		//compute path
		let angle = dir * BASE_SPEED * TIME_STEP * now % 360./ PI;

		// compute target x/y
		let x_dst = x_radius * angle.cos() + x_pivot;
		let y_dst = y_radius * angle.sin() + y_pivot;

		//compute distance
		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let distance = (dx*dx + dy*dy).sqrt();
		let distance_ratio = if distance != 0. {max_distance / distance} else {0.};

		//compute final
		let x = x_org - dx * distance_ratio;
		let x = if dx > 0. {x.max(x_dst)} else {x.min(x_dst)};
		let y = y_org - dy * distance_ratio;
		let y = if dy > 0. {y.max(x_dst)} else {y.min(x_dst)};

		let translation = &mut transform.translation;
		(translation.x, translation.y) = (x,y);

		
<<<<<<< HEAD
=======
		if x > 500.{
			println!("greater than 500 im going right");
			translation.x += dx * distance_ratio;
		}
		else if x <= -495. {
			//println!("less than -495 im going right");
			translation.x += BASE_SPEED * TIME_STEP/4. + 900.;
			//translation.x += BASE_SPEED * TIME_STEP/4.;
		}
		// else if x > 1000.{
		// 	translation.x += dx * distance_ratio;
		// }
		else{
			//println!("wtf");
			translation.x += dx * distance_ratio;
		}
		//translation.x += BASE_SPEED * TIME_STEP/4.;
		// println!("current x = {}", x);
		// println!("current dist_ratio = {}", distance_ratio);

		let target = transform.translation + Vec3::new(dx, 0., 0.);
		if check_tile_collision(target, &collision){
			//println!("HIT!");
			transform.translation = target;
			
		}
		//let x = if dx > 0. {x.max()} 
		//translation.y += BASE_SPEED * TIME_STEP/4.;
>>>>>>> d22debedcc9e9a77bce65d80c38dc3c78e53d527
	}
}