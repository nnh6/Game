use bevy::prelude::*;
use iyes_loopless::prelude::*;
//use std::convert::From;
//use std::time::Duration;

use crate::{
	//LEVEL_LEN,
	//WIN_W,
	WIN_H,
	TILE_SIZE,
	//ANIM_TIME,
	//ACCEL_RATE,
	//PLAYER_SPEED,
	//JUMP_TIME,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
	player::Health,
	//level::Background,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Deref, DerefMut)]
pub struct EnemySheet(Handle<TextureAtlas>);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_enemy_sheet)
			.add_enter_system(GameState::Playing, spawn_enemy);
	}
}

fn load_enemy_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let enemy_handle = asset_server.load("enemy.png");
	loading_assets.insert(
		enemy_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(enemy_handle.clone_untyped(), &asset_server),
	);

	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2::splat(50.), 1, 1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	
	commands.insert_resource(EnemySheet(enemy_atlas_handle));
}

fn spawn_enemy(
	mut commands: Commands,
	enemy_sheet: Res<EnemySheet>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: enemy_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(500., -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		})
		//.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
		//.insert(Velocity::new())
		//.insert(JumpTimer(Timer::from_seconds(JUMP_TIME, false)))
		.insert(Health::new())
		.insert(Enemy);
}