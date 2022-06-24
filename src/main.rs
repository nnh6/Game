use bevy::{
	window::PresentMode,
	prelude::*,	
};
use iyes_loopless::prelude::*;
use bevy_kira_audio::AudioPlugin;

mod loading;
mod player;
mod level;
mod music;

use loading::LoadingPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use music::BackgroundMusicPlugin;

const TITLE: &str = "Miner Pitfall!";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;
const ANIM_TIME: f32 = 0.2;
const JUMP_TIME: f32 = 0.1;
const TILE_SIZE: f32 = 100.;

//const LEVEL_LEN: f32 = 1280.;

const PROGRESS_LENGTH: f32 = 120.;
const PROGRESS_HEIGHT: f32 = 20.;
const PROGRESS_FRAME: f32 = 5.;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
	Loading,
	Playing,
}

fn main() {
	App::new()
		// Setup Bevy and game window
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::DARK_GRAY))
		.add_plugins(DefaultPlugins)
		// Set initial state
		.add_loopless_state(GameState::Loading)
		// Add general systems
		.add_startup_system(setup_camera)
		.add_startup_system(credit_setup)
		.add_system(log_state_change)
		// Add all subsystems
		//############### currently greyed out everything but player ######
		//.add_plugin(AudioPlugin)
		.add_plugin(LoadingPlugin)
		//.add_plugin(BackgroundMusicPlugin)
		.add_plugin(PlayerPlugin)
		.add_plugin(LevelPlugin)
		// Run the game
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn log_state_change(state: Res<CurrentState<GameState>>) {
	if state.is_changed() {
		info!("Detected state change to {:?}!", state);
	}
}

fn credit_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	//timings: Justin 3, Nara 6, Yinuo 9, Lucas 12, Landin 15, Grant 18, Matt 21
	//TODO: write functions for this repetetive code
	//array of filenames, loop through displaying them
	//commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("best_monkey.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		}); 
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("justinCredits.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(3., false)));
	info!("Hello Justin!");
	//Nara
	commands
	.spawn_bundle(SpriteBundle {
		texture: asset_server.load("NaraEndCredit.png"),
		transform: Transform::from_xyz(0., 0., -1.),
		..default()
	})
	.insert(PopupTimer(Timer::from_seconds(6., false)));
	info!("Hello Nara!");
	commands
	.spawn_bundle(SpriteBundle {
		texture: asset_server.load("yinuo-credit r.png"),
		transform: Transform::from_xyz(0., 0., -1.),
		..default()
	})
	.insert(PopupTimer(Timer::from_seconds(9., false)));
	info!("Hello Yinuo!");
	//Landin
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("lrm88-credit-slide_LI.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(12., false)));
	info!("Hello Lucas!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("landin-credits.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(15., false)));
	info!("Hello Landin!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Grant-Credit.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(18., false)));
	info!("Hello Grant!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("trezza-credit.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(21., false)));
	info!("Hello Matt!");
	

}

fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.z = timer.duration().as_secs() as f32;
			info!("End Credits!");
		}
	}
}
