use std::collections::HashMap;
use bevy::{
	asset::LoadState,	
	prelude::*,
};
use iyes_loopless::prelude::*;

use crate::{
	PROGRESS_LENGTH,
	PROGRESS_HEIGHT,
	PROGRESS_FRAME,
	GameState,
};

#[derive(Component)]
struct LoadingProgressFrame;

#[derive(Component)]
struct LoadingProgress(f32);

#[derive(Deref, DerefMut)]
pub struct LoadingAssets(pub HashMap<HandleUntyped, LoadingAssetInfo>);

pub struct LoadingAssetInfo {
	pub handle: HandleUntyped,
	pub state: LoadState,
	pub path: String,
}
impl LoadingAssetInfo {
	pub fn for_handle(handle: HandleUntyped, asset_server: &Res<AssetServer>) -> Self {
		let state = asset_server.get_load_state(&handle);
		// `get_handle_path` returns an Option<T>, see `map_or` def in Option docs
		let path = asset_server.get_handle_path(&handle)
			.map_or(String::from("???"), |h| format!("{:?}", h.path()));

		Self { handle, state, path }
	}

	pub fn update_state(&mut self, asset_server: &Res<AssetServer>) {
		let new_state = asset_server.get_load_state(&self.handle);
		if new_state != self.state {
			match new_state {
				LoadState::Failed => warn!("{:?}: {}", new_state, self.path),
				_ => info!("{:?}: {}", new_state, self.path),
			}
			self.state = new_state;
		}
	}
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
	fn build (&self, app: &mut App) {
		app.insert_resource(LoadingAssets(HashMap::new()))
			.add_enter_system(GameState::Loading, setup_loading)
			.add_system(update_loading.run_in_state(GameState::Loading))
			.add_exit_system(GameState::Loading, despawn_with::<LoadingProgressFrame>)
			.add_exit_system(GameState::Loading, despawn_with::<LoadingProgress>);
	}
}

fn setup_loading(mut commands: Commands) {
	commands
		.spawn()
		.insert(LoadingProgressFrame)
		.insert_bundle(SpriteBundle {
			transform: Transform {
				scale: Vec3::new(
					PROGRESS_LENGTH + PROGRESS_FRAME,
					PROGRESS_HEIGHT + PROGRESS_FRAME,
					0.,
				),
				..default()
			},
			sprite: Sprite {
				color: Color::BLACK,
				..default()
			},
			..default()
		});

	commands
		.spawn()
		.insert(LoadingProgress(0.))
		.insert_bundle(SpriteBundle {
			transform: Transform {
				scale: Vec3::new(0., PROGRESS_HEIGHT, 0.),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		});	
}

fn update_loading(
	mut commands: Commands,
	asset_server: Res<AssetServer>,	
	mut loading_assets: ResMut<LoadingAssets>,
	mut loading_progress: Query<&mut Transform, With<LoadingProgress>>,
) {
	let mut progress_transform = loading_progress.single_mut();

	for info in loading_assets.values_mut() {
		info.update_state(&asset_server);
	}

	let loaded: usize = loading_assets.values()
		.map(|i| match i.state {
			LoadState::Loaded => 1,
			_ => 0,
		})
		.sum();
	let percent = (loaded as f32) / (loading_assets.len() as f32);

	progress_transform.scale.x = PROGRESS_LENGTH * percent;

	// Check if all assets are loaded
	if asset_server.get_group_load_state(loading_assets.keys().map(|h| h.id))
		== LoadState::Loaded
	{
		commands.insert_resource(NextState(GameState::Playing));
	}
}

fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
	for e in q.iter() {
		commands.entity(e).despawn_recursive();
	}
}
