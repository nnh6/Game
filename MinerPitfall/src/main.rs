use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

fn main() {
    App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Miner Pitfall!"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(show_popup)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	//timings: Justin 3, Nara 6, Yinuo 9, Lucas 12, Landin 15, Grant 18, Matt 21
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("monke.png"),
			..default()
		}); 
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("justinCredits.png"),
			transform: Transform::from_xyz(0., 0., -7.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(3., false)));
	info!("Hello Justin!");
	//Nara
	commands
	.spawn_bundle(SpriteBundle {
		texture: asset_server.load("NaraEndCredit.png"),
		transform: Transform::from_xyz(0., 0., -6.),
		..default()
	})
	.insert(PopupTimer(Timer::from_seconds(6., false)));
	info!("Hello Nara!");
	commands
	.spawn_bundle(SpriteBundle {
		texture: asset_server.load("yinuo-credit r.png"),
		transform: Transform::from_xyz(0., 0., -6.),
		..default()
	})
	.insert(PopupTimer(Timer::from_seconds(9., false)));
	info!("Hello Yinuo!");
	//Landin
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("lrm88-credit-slide_LI.png"),
			transform: Transform::from_xyz(0., 0., -3.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(12., false)));
	info!("Hello Lucas!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("landin-credits.png"),
			transform: Transform::from_xyz(0., 0., -3.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(15., false)));
	info!("Hello Landin!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Grant-Credit.png"),
			transform: Transform::from_xyz(0., 0., -3.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(18., false)));
	info!("Hello Grant!");
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("trezza-credit.png"),
			transform: Transform::from_xyz(0., 0., -3.),
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
			transform.translation.z = 7.;
			info!("End Credits!");
		}
	}
}
