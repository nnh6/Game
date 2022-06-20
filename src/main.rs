use bevy::{
	prelude::*,
	window::PresentMode,
};
use std::time::Duration;

#[derive(Component)]
struct Slide;

struct PopupTimer {
	timer: Timer,
	z: f32,
	names: [&'static str; 8]
}

fn main() {
    App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Miner Pitfall!"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(PopupTimer{
			timer: Timer::new(Duration::from_secs(3), true),
			z: 0.,
			names: ["best_monkey.png", "justinCredits.png", "NaraEndCredit.png",
			"yinuo-credit r.png", "lrm88-credit-slide_LI.png",
			 "landin-credits.png", "Grant-Credit.png", "trezza-credit.png"]
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(display_slides)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	
}

fn display_slides(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	mut p_timer: ResMut<PopupTimer>
) {
	p_timer.timer.tick(time.delta());
	if p_timer.timer.just_finished() && p_timer.z < 8. {
		let name = p_timer.names[p_timer.z as usize];
		p_timer.z = p_timer.z + 1.;
		info!("{}", name);
		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load(name),
			transform: Transform::from_xyz(0., 0., p_timer.z),
			..default()
		})
		.insert(Slide);
	}
}