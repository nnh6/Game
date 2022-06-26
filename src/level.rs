use std::{
    fs::File,
    io::{BufRead, BufReader},
}; //might have to ask to use these
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
	//LEVEL_LEN,
	WIN_W,
	WIN_H,
	TILE_SIZE,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
};



#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Door;

// Will need to access these with .0, not deriving Deref/DerefMut
pub struct BackgroundImage(Handle<Image>);
pub struct DoorImage(Handle<Image>);
pub struct BrickSheet(Handle<TextureAtlas>);

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_level)
			.add_enter_system(GameState::Playing, setup_level);
	}
}

fn load_level(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let bg_texture_handle = asset_server.load("small_bg.png");
	
	loading_assets.0.insert(
		bg_texture_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bg_texture_handle.clone_untyped(), &asset_server),
	);
	commands.insert_resource(BackgroundImage(bg_texture_handle));

	let brick_handle = asset_server.load("bricks.png");
	loading_assets.0.insert(
		brick_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(brick_handle.clone_untyped(), &asset_server),
	);

	let brick_atlas = TextureAtlas::from_grid(brick_handle, Vec2::splat(TILE_SIZE), 4, 1);
	let brick_atlas_handle = texture_atlases.add(brick_atlas);

	commands.insert_resource(BrickSheet(brick_atlas_handle));

	let door_handle = asset_server.load("door.png");
	loading_assets.0.insert(
		door_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(door_handle.clone_untyped(), &asset_server),
	);
	commands.insert_resource(DoorImage(door_handle));

}

fn setup_level(
	mut commands: Commands,
	texture_atlases: Res<Assets<TextureAtlas>>,	
	background_image: Res<BackgroundImage>,
	door_image: Res<DoorImage>,
	brick_sheet: Res<BrickSheet>,
) {
	commands
		.spawn_bundle(SpriteBundle {
			texture: background_image.0.clone(),
			transform: Transform {
				translation: Vec3::new(0., 0. , 100.0), 
				..default()
			},
			..default()
		})
		.insert(Background); //spawns background


	let file = File::open("assets/map.txt").expect("No map file found");
	let brick_atlas = texture_atlases.get(&brick_sheet.0);
	let brick_len = brick_atlas.unwrap().len();
	let mut i = 0;
	let mut t = Vec3::new(-WIN_W/2. + TILE_SIZE/2., WIN_H/2. - TILE_SIZE/2., 0.);
	for(y, line) in BufReader::new(file).lines().enumerate() { //read each line from file
		if let Ok(line) = line {
			for (x, char) in line.chars().enumerate() { //read each char from line
				match char { 
					'#'=> {
						commands
							.spawn_bundle(SpriteSheetBundle {
								texture_atlas: brick_sheet.0.clone(),
								sprite: TextureAtlasSprite {
									index: i % brick_len,
									..default()
								},
								transform: Transform {
									translation: t + Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0), // positions the bricks starting from the top-left (I hope)
									..default()
								},
								..default()
							})
							.insert(Brick);

						i += 1;
					},
					'D'=> {
						commands
							.spawn_bundle(SpriteBundle {
								texture: door_image.0.clone(),
								transform: Transform {
									translation: t + Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0), // positions the bricks starting from the top-left (I hope)
									..default()
								},
								..default()
							})
							.insert(Door);

						i += 1;
					}
					_=> {
						//default case
					}
				}
			}
		}
    }
}
