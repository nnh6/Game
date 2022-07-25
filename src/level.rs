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
	MAP_WIDTH,
	MAP_HEIGHT,
	ROOM_WIDTH,
	ROOM_HEIGHT,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
	player::*,
	enemy::*,
};

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Door;


#[derive(Component,Copy,Clone)]
pub struct Room
{
	room_coords:[[char;ROOM_WIDTH]; ROOM_HEIGHT] //array of tiles in the room
}
impl Room
{
	pub fn new() -> Self {
		Self{room_coords: [['-'; ROOM_WIDTH]; ROOM_HEIGHT] }
	}
}
#[derive(Component,Copy,Clone)]
pub struct Map
{
	map_coords:[[Room;MAP_WIDTH]; MAP_HEIGHT], //array of rooms on the map
	x_coords: usize,
	y_coords: usize, //coordinates for location of the current room
}
impl Map
{
	pub fn new() -> Self {
		Self{map_coords: [[Room::new(); MAP_WIDTH]; MAP_HEIGHT], x_coords: 0, y_coords: 0 }
	}
}

// Will need to access these with .0, not deriving Deref/DerefMut
pub struct BackgroundImage(Handle<Image>);
pub struct DoorImage(Handle<Image>);
pub struct BrickSheet(Handle<TextureAtlas>);

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_level)
			.add_enter_system(GameState::Loading, read_map)
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

	let brick_handle = asset_server.load("tiles.png");
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
	mut map_query: Query<(&mut Map)>,
	texture_atlases: Res<Assets<TextureAtlas>>,	
	background_image: Res<BackgroundImage>,
	door_image: Res<DoorImage>,
	brick_sheet: Res<BrickSheet>,
	enemy_sheet: Res<EnemySheet>,
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


	//let file = File::open("assets/map.txt").expect("No map file found");
	let brick_atlas = texture_atlases.get(&brick_sheet.0);
	let brick_len = brick_atlas.unwrap().len();
	let map = map_query.single_mut();
	let current_room = map.map_coords[map.x_coords][map.y_coords];
	let mut i = 0;
	let t = Vec3::new(-WIN_W/2. + TILE_SIZE/2., WIN_H/2. - TILE_SIZE/2., 0.);
	for(y, line) in current_room.room_coords.iter().enumerate() { //read each line from map
		for (x, char) in line.iter().enumerate() { //read each char from line
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
						.insert(Brick)
						.insert(Collider);
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
				'E'=> {
					commands
						.spawn_bundle(SpriteSheetBundle {
							texture_atlas: enemy_sheet.clone(),
							sprite: TextureAtlasSprite {
								index: 0,
								..default()
							},
							transform: Transform {
								translation: t + Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
								..default()
							},
							..default()
						})
						.insert(Health::new())
						.insert(Enemy);
					i += 1;
				}
				_=> {
					
					//default case
				}
			}
		}
    }
}

fn read_map(
	mut commands: Commands,
	
	//texture_atlases: Res<Assets<TextureAtlas>>,	
	//background_image: Res<BackgroundImage>,
	//door_image: Res<DoorImage>,
	//brick_sheet: Res<BrickSheet>,
	//enemy_sheet: Res<EnemySheet>,
) {
	let file = File::open("assets/map.txt").expect("No map file found");
	//let brick_atlas = texture_atlases.get(&brick_sheet.0);
	//let brick_len = brick_atlas.unwrap().len();
	let mut map = Map::new();
	let mut z1 = 0;
	let mut z2 = 0; //Current Map location is [z1],[z2]
	let mut current_room = map.map_coords[z1][z2];
	let t = Vec3::new(-WIN_W/2. + TILE_SIZE/2., WIN_H/2. - TILE_SIZE/2., 0.);
	for(x, line) in BufReader::new(file).lines().enumerate() { //read each line from file
		if let Ok(line) = line {
			for (y, char) in line.chars().enumerate() { //read each char from line
				match char { 
					
					'!'=> { //End of Room
						map.map_coords[z1][z2] = current_room;
						if z2<MAP_WIDTH-1 { //counting starts at 0
						z2+= 1;
						}
						else if z1<MAP_HEIGHT-1 {
							z2 = 0;
							z1 +=1;
						}
						current_room = map.map_coords[z1][z2];
					}
					
					//needs case for directional exit markers
					
					_=> {
						info!("{}", char);
						current_room.room_coords[x%9][y%16] = char;
						//default case
					}
				}
			}
		}
    }
	map.map_coords[z1][z2] = current_room; //need to add the last room since the loop never does
	//might add a line here to set the current room to the middle of the map
	commands.spawn().insert(map);
} 

/*
Divide Map File into "Cells" (9:16 Rooms)
Load file into 2D Array of Rooms
Each Room is a 2D Array of Characters. 2D Array of 2D Arrays
When loading a room, do it similar to the current method, but reading from the Room at the index in the array instead of directly from the file.
Potentially include 0/1/2/3 as indicators of Exits to a room.
When reading in a room from file, it should have a seperate array of it's exits, so we can check it without reading the entire room every time.
(This means each room is a struct containing a 2D array of its contents and a 1 Dimension array of its exits)
This trait will be important for generation, so we can make sure that adjacent rooms' exits actually connect to each other.

*/
