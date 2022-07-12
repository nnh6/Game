use bevy::{
	window::PresentMode,
	prelude::*,	
};
use std::time::Duration;
use iyes_loopless::prelude::*;

mod loading;
mod player;
mod level;
mod music;

mod enemy;
mod start_menu;

use loading::LoadingPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use start_menu::MainMenuPlugin;

const TITLE: &str = "Miner Pitfall!";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SPEED: f32 = 6.;
const ANIM_TIME: f32 = 0.2;
const JUMP_TIME: f32 = 120.;
const INV_TIME: f32 = 1.;
const FRAME_TIME: f32 = 0.016667;
const TILE_SIZE: f32 = 80.;
const PROGRESS_LENGTH: f32 = 120.;
const PROGRESS_HEIGHT: f32 = 20.;
const PROGRESS_FRAME: f32 = 5.;

#[derive(Component)]
struct Slide;

struct PopupTimer {
	timer: Timer,
	z: f32,
	names: [&'static str; 8]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
	MainMenu,
	Loading,
	Playing,
	Credits,
	GameOver,
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
		.insert_resource(PopupTimer{
			timer: Timer::new(Duration::from_secs(3), true),
			z: 0.,
			names: ["best_monkey.png", "justinCredits.png", "NaraEndCredit.png",
			"yinuo-credit r.png", "lrm88-credit-slide_LI.png",
			 "landin-credits.png", "Grant-Credit.png", "trezza-credit.png"]
		})
		.add_plugins(DefaultPlugins)
		// Set initial state
		.add_loopless_state(GameState::Loading)
		// Add general systems
		.add_startup_system(setup_camera)
		.add_system(
			display_slides
				.run_in_state(GameState::Credits)
			)
		.add_system(display_game_over
						.run_in_state(GameState::GameOver)
					)
		.add_enter_system(GameState::Credits, despawn_all)
		.add_system(log_state_change)
		// Add all subsystems
		.add_plugin(LoadingPlugin)
		.add_plugin(PlayerPlugin)
		.add_plugin(LevelPlugin)
		.add_plugin(EnemyPlugin)
		.add_plugin(MainMenuPlugin)
		.run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
	let camera = OrthographicCameraBundle::new_2d();
	commands.spawn_bundle(camera);
}

fn log_state_change(state: Res<CurrentState<GameState>>) {
	if state.is_changed() {
		info!("Detected state change to {:?}!", state);
	}
}

fn display_slides(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	mut p_timer: ResMut<PopupTimer>,
) {
	p_timer.timer.tick(time.delta());
	if p_timer.timer.just_finished() && p_timer.z < 8. {
		let name = p_timer.names[p_timer.z as usize];
		p_timer.z += 1.;
		
		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load(name),
			transform: Transform::from_xyz(0., 0., p_timer.z),
			..default()
		})
		.insert(Slide);
	}
}

fn despawn_all (
    mut commands: Commands,
    query: Query<Entity, With<Transform>>,
)
{
    query.for_each(|entity| {
        commands.entity(entity).despawn();
	});
	setup_camera(commands);
}

fn display_game_over (
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("gameover.png"),
			transform: Transform::from_xyz(0., 0., 900.),
			..default()
		})
		.insert(Slide);
}