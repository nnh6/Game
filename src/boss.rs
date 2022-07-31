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
    player::Health
};

#[derive(Component)]
pub struct Boss;

#[derive(Deref, DerefMut)]
pub struct BossSheet(Handle<TextureAtlas>);

pub struct BossPlugin;
impl Plugin for BossPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_boss_sheet)
		.add_system(boss_movement_system);
		
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



fn boss_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Boss>>){
	let now = time.seconds_since_startup() as f32;
	for mut transform in query.iter_mut(){
		//run at player if in LOS
        //else hide
	}
}

fn boss_animate(
    texture_atlases: Res<Assets<TextureAtlas>>,
	mut boss: Query<
		(
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&Health,
			&mut Transform
		),
		With<Boss>
	>,
){
    for (mut sprite, texture_atlas_handle, health, mut transform) in boss.iter_mut() {
        //if health >= 50.0{
        //   sprite.index = 1;
        //}
        //else player is near retreat into shell
    }
}