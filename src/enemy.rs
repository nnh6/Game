use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
	GameState,
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
		app.add_enter_system(GameState::Loading, load_enemy_sheet);
	}
}

fn load_enemy_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let enemy_handle = asset_server.load("groundhog.png");
	loading_assets.insert(
		enemy_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(enemy_handle.clone_untyped(), &asset_server),
	);

	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2::splat(80.), 1, 1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	
	commands.insert_resource(EnemySheet(enemy_atlas_handle));
}