use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_kira_audio::{
	Audio,
	AudioSource,
};

use crate::{
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
};

#[derive(Deref, DerefMut)]
pub struct BackgroundMusic(Handle<AudioSource>);

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_background_music)
			.add_enter_system(GameState::Playing, start_background_music);
	}
}

fn load_background_music(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let bg_music_handle = asset_server.load("bg_music.wav");
	loading_assets.insert(
		bg_music_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bg_music_handle.clone_untyped(), &asset_server),
	);
	commands.insert_resource(BackgroundMusic(bg_music_handle));
}

fn start_background_music(
	background_music: Res<BackgroundMusic>,
	audio: Res<Audio>,
) {
	audio.play_looped(background_music.clone());
}
