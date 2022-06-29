use std::collections::HashMap;
use bevy::{
	asset::LoadState,	
	prelude::*,
	ui::FocusPolicy
};
use iyes_loopless::prelude::*;

use crate::{
	PROGRESS_LENGTH,
	PROGRESS_HEIGHT,
	PROGRESS_FRAME,
	GameState,
	MainCamera,
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
			//.add_enter_system(GameState::MainMenu, setup_menu)
			//.add_system(handle_start_button)
			.add_enter_system(GameState::Loading, setup_loading)
			.add_system(update_loading.run_in_state(GameState::Loading))
			.add_exit_system(GameState::Loading, despawn_with::<LoadingProgressFrame>)
			.add_exit_system(GameState::Loading, despawn_with::<LoadingProgress>);
	}
}


// impl Plugin for MainMenuPlugin{
// 	fn build(&self, app: &mut App){
// 		app.add_startup_system(setup_menu);
// 	}
// }
struct UiAssets{
	font: Handle<Font>,
	button: Handle<Image>,
	button_pressed: Handle<Image>
}
fn handle_start_button(
	mut commands: Commands,
	interaction_query: Query<(&Children, &Interaction), Changed<Interaction>>,
	mut image_query: Query<&mut UiImage>,
	ui_assets: Res<UiAssets>,
	//ascii: Rec<AsciiSheet>
){
	for(children, interaction) in interaction_query.iter(){
		let child = children.iter().next().unwrap();
		let mut image = image_query.get_mut(*child).unwrap();

		match interaction{
			Interaction:: Clicked => {
				image.0 = ui_assets.button_pressed.clone();
				commands.insert_resource(NextState(GameState::Loading));
			}
			Interaction::Hovered | Interaction:: None=>{
				image.0 = ui_assets.button.clone();
			}
		}
	}
}
fn setup_menu(mut commands: Commands, assets: Res<AssetServer>){
	let ui_assets = UiAssets{
		font: assets.load("quattrocentosans-bold.ttf"),
		button: assets.load("button.png"),
		button_pressed: assets.load("button_pressed.png")
	};

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle{
        style: Style{
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
		color: Color::NONE.into(),
        ..Default::default()
    })
	.with_children(|parent|{
		parent.spawn_bundle(ImageBundle{
			style:Style{
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			image: ui_assets.button.clone().into(),
			..Default::default()
		})
			.insert(FocusPolicy::Pass)
			.with_children(|parent|{
				parent.spawn_bundle(TextBundle{
					text: Text::with_section(
						"Start Game",
						TextStyle{
							font: ui_assets.font.clone(),
							font_size: 40.0,
							color: Color::rgb(0.9, 0.9, 0.9),
						},
						Default::default(),
					),
					focus_policy: FocusPolicy::Pass,
					..Default::default()
				});
			});
			
	}); 
	commands.insert_resource(ui_assets);
}

fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
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
