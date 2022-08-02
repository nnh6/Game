use std::{
    fs::File,
    io::{BufRead, BufReader},
	fmt,
}; //might have to ask to use these


use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;
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
	boss::*,
	
};

const T: u32 = 5;	//CA threshold value
const N: usize = 55;	//number of seed walls
const P: u32 = 3;  //iterations of the CA to run

// room.exits indexes
const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;


#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Door;

#[derive(Component)]
pub struct Unbreakable;

#[derive(Component,Copy,Clone,Debug)]
pub struct Room
{
	seed_wall_locations: [usize;N],
	room_coords:[[char;ROOM_WIDTH]; ROOM_HEIGHT], //array of tiles in the room
	exits: [bool;4],
}

impl Room
{
	pub fn new(exits: [bool;4]) -> Self {
		Self {
			seed_wall_locations: gen_seed_wall_locations(),
			room_coords: [['-'; ROOM_WIDTH]; ROOM_HEIGHT],
			exits,
		}
	}
}

impl fmt::Display for Room {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let coords = &self.room_coords;

		for row in coords {
			write!(f, "\n{:?}", row)?;
		}
		write!(f, "")
	}
}

#[derive(Component,Clone)]
pub struct Map
{
	map_coords: Vec<[Room;MAP_WIDTH]>, //array of rooms on the map
	pub x_coords: usize,
	pub y_coords: usize, //coordinates for location of the current room
	pub player_spawn: Transform,
}

impl Map
{
	pub fn new() -> Self {
		Self{map_coords: vec![[Room::new([true, true, true, true]); MAP_WIDTH]; MAP_HEIGHT], x_coords: 0, y_coords: 0, player_spawn: Transform::from_xyz(-400., -(WIN_H/2.) + (TILE_SIZE * 2.5), 900.) }
	}
}
#[derive(Component)]
pub struct BombItem;

#[derive(Component)]
pub struct HealthItem;

// Will need to access these with .0, not deriving Deref/DerefMut
pub struct BackgroundImage(Handle<Image>);
pub struct DoorImage(Handle<Image>);
pub struct BrickSheet(Handle<TextureAtlas>);
pub struct BombItemSheet(Handle<TextureAtlas>);
pub struct HealthItemSheet(Handle<TextureAtlas>);

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_level)
			.add_enter_system(GameState::Loading, generate_map)
			.add_enter_system(GameState::Playing, setup_level)
			.add_enter_system(GameState::Traverse,despawn_all)
			;
	}
}

fn load_level(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
	mut loading_assets: ResMut<LoadingAssets>,
) {
	//Background
	let bg_texture_handle = asset_server.load("small_bg.png");
	
	loading_assets.0.insert(
		bg_texture_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bg_texture_handle.clone_untyped(), &asset_server),
	);
	commands.insert_resource(BackgroundImage(bg_texture_handle));

	//Brick
	let brick_handle = asset_server.load("tiles.png");
	loading_assets.0.insert(
		brick_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(brick_handle.clone_untyped(), &asset_server),
	);

	let brick_atlas = TextureAtlas::from_grid(brick_handle, Vec2::splat(TILE_SIZE), 4, 1);
	let brick_atlas_handle = texture_atlases.add(brick_atlas);

	commands.insert_resource(BrickSheet(brick_atlas_handle));

	//Door
	let door_handle = asset_server.load("door.png");
	loading_assets.0.insert(
		door_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(door_handle.clone_untyped(), &asset_server),
	);
	commands.insert_resource(DoorImage(door_handle));
	//info!("{}", generate_room([false, true, true, false]));

	//Bomb
	let bomb_handle = asset_server.load("bomb_boom.png");
	loading_assets.0.insert(
		bomb_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bomb_handle.clone_untyped(), &asset_server),
	);

	let bomb_atlas = TextureAtlas::from_grid(bomb_handle, Vec2::splat(35.), 6, 1);
	let bomb_atlas_handle = texture_atlases.add(bomb_atlas);

	commands.insert_resource(BombItemSheet(bomb_atlas_handle));

	//Health
	let hp_handle = asset_server.load("Health_Item.png");
	loading_assets.0.insert(
		hp_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(hp_handle.clone_untyped(), &asset_server),
	);

	let hp_atlas = TextureAtlas::from_grid(hp_handle, Vec2::new(45.,35.), 1, 1);
	let hp_atlas_handle = texture_atlases.add(hp_atlas);

	commands.insert_resource(HealthItemSheet(hp_atlas_handle));
}

fn setup_level(
	mut commands: Commands,
	mut map_query: Query<&mut Map>,
	texture_atlases: Res<Assets<TextureAtlas>>,	
	background_image: Res<BackgroundImage>,
	door_image: Res<DoorImage>,
	brick_sheet: Res<BrickSheet>,
	enemy_sheet: Res<EnemySheet>,
	boss_sheet: Res<BossSheet>,
	bomb_sheet: Res<BombItemSheet>,
	hp_sheet: Res<HealthItemSheet>
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
	
	//generate and store new room if OOB
	/* 
	let (x,y) = (map.x_coords,map.y_coords);

	if map.map_coords.len() >= x && map.map_coords[x].len() >= y{
		map.map_coords[x][y] = generate_room(map.cur_exits);
	}
	*/
	let current_room = map.map_coords[map.y_coords][map.x_coords];
	info!("{:?}", [map.x_coords, map.y_coords]);
	info!("{:?}", current_room.exits);
	
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
						.insert(Health::new())
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
				'T'=> {
					commands
						.spawn_bundle(SpriteSheetBundle {
							texture_atlas: boss_sheet.clone(),
							sprite: TextureAtlasSprite {
								index: 0,
								..default()
							},
							transform: Transform {
								translation: t + Vec3::new(x as f32 * TILE_SIZE, (-(y as f32) * TILE_SIZE)+37.0, 900.0),
								..default()
							},
							..default()
						})
						.insert(Health::new())
						.insert(Boss{health:100.0,y_velocity:0.0,y_accel:0.0,x_velocity:0.0,last_move: 0.0,turtled:false,path: Vec3::new(0.,0.,0.)});
					i += 1;
				}
				'U'=> {
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
						.insert(Collider)
						.insert(Unbreakable);
						i += 1;
				}
				'B'=> {
					commands
					.spawn_bundle(SpriteSheetBundle {
						texture_atlas: bomb_sheet.0.clone(),
						sprite: TextureAtlasSprite {
							index: 0,
							..default()
						},
						//transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
						transform: Transform {
								translation: t + Vec3::new(x as f32 * TILE_SIZE, (-(y as f32) * TILE_SIZE)-23.0, 900.0),
								..default()
							},
						..default()
					})
					//.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
					//.insert(Velocity::new())
					.insert(BombItem);
					//ENEMY CODE
					i += 1;
				}
				'H'=> {
					commands
					.spawn_bundle(SpriteSheetBundle {
						texture_atlas: hp_sheet.0.clone(),
						sprite: TextureAtlasSprite {
							index: 0,
							..default()
						},
						//transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
						transform: Transform {
								translation: t + Vec3::new(x as f32 * TILE_SIZE, (-(y as f32) * TILE_SIZE)-23.0, 900.0),
								..default()
							},
						..default()
					})
					//.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)))
					//.insert(Velocity::new())
					.insert(HealthItem);
					//ENEMY CODE
					i += 1;
				}
				_=> {
					
					//default case
				}
			}
		}
    }
}

fn generate_map(
	mut commands: Commands,
	//mut rooms_query: Query<(&mut GennedRooms)>,
	) {
		let mut new_map = Map::new(); 
		let mut rand_exits : [bool;4] = [true;4];
		let mut rng = rand::thread_rng();
		for i in 0..MAP_HEIGHT /*in new_map.map_coords.iter_mut().enumerate()*/ {
			for j in 0..MAP_WIDTH/* in row.iter_mut().enumerate()*/ {
				
				let r1 : u8 = rng.gen_range(0..=1);
				if r1 == 0 {
					rand_exits[BOTTOM] = false;
				}
				else {
					rand_exits[BOTTOM] = true;
				}
				let r2 : u8 = rng.gen_range(0..=1);
				if r2 == 0 {
					rand_exits[RIGHT] = false; 
				}
				else {
					rand_exits[RIGHT] = true;
				} //random assignments need to be at the top of the function in case we need to reassign them
				
				// we only randomly generate our bottom and right exits, since we know what the top and left have to be based on the previously generated rooms
				
				if i == (MAP_HEIGHT-1)/2 {
					if j == ((MAP_WIDTH-1)/2) - 1 {
						rand_exits[RIGHT] = true;
					 //connect to left exit of center room (we don't need to do the right exit because we check for that anyway)
					}
					else if j == ((MAP_WIDTH-1)/2) {
						//code to load in this room
						new_map.map_coords[i][j] = starting_room();
						continue; //SKIP GENERATING THIS ROOM so we can use a starting room that is not random
					}
				}
				if j == (MAP_WIDTH-1)/2 && i == ((MAP_HEIGHT-1)/2) - 1 {
    					rand_exits[BOTTOM] = true;
    					//connect to top exit of center room (we don't need to do the bottom because we check for that later anyway.)
    					}
				//above section ensures we can insert a room in the middle and leave it connected
				
				if i>0 && new_map.map_coords[i-1][j].exits[BOTTOM] { //is the above room connected to this one
					rand_exits[TOP] = true;
				}
				else {
					rand_exits[TOP] = false;			
				}
				if j>0 && new_map.map_coords[i][j-1].exits[RIGHT] { //is the left room connected to this one
					rand_exits[LEFT] = true;
				}
				else {
					rand_exits[LEFT] = false;
				}
				if i == MAP_HEIGHT-1 {
					rand_exits[BOTTOM] = false;
				}
				if j == MAP_WIDTH-1 {
					rand_exits[RIGHT] = false;
				}
				//info!("{}", i);
				//info!("{}", j);
				info!("{:?}", [i,j]);
				if i>0 { info!("{:?}", [rand_exits[TOP],new_map.map_coords[i-1][j].exits[BOTTOM]]);}
				new_map.map_coords[i][j] = generate_room(rand_exits);
			}
			
		}
		new_map.x_coords = (MAP_WIDTH-1)/2;
		new_map.y_coords = (MAP_HEIGHT-1)/2;
		commands.spawn().insert(new_map);
}

fn starting_room() -> Room {
	let file = File::open("assets/start_room.txt").expect("No map file found");
	let mut new_room = Room::new([true; 4]);
	for(x, line) in BufReader::new(file).lines().enumerate() { //read each line from file
		if let Ok(line) = line {
			for (y, char) in line.chars().enumerate() { //read each char from line
				new_room.room_coords[x%9][y%16] = char;
			}
		}
	}
	new_room
}


fn read_map( //No longer used
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
	let _t = Vec3::new(-WIN_W/2. + TILE_SIZE/2., WIN_H/2. - TILE_SIZE/2., 0.);
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
						//info!("{}", char);
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

fn generate_room(exits: [bool;4]) -> Room {
	let mut new_room = Room::new(exits);
	let mut cell_count = 0;
	let mut rng = thread_rng();
	let _door_here = rng.gen_range(0..100) == 50;

	for (i, row) in new_room.room_coords.iter_mut().enumerate() {
		for (j, character) in row.iter_mut().enumerate() {
			if (i == 0 && !exits[TOP])|| (j == 0 && !exits[LEFT]) || (i == ROOM_HEIGHT - 1 && !exits[BOTTOM]) || (j == ROOM_WIDTH - 1 && !exits[RIGHT]){
				//surround outside of room with walls
				*character = 'U';
			}

			if *character == '-' && rng.gen_range(0..35) == 5 {
				*character = 'E';
			}

			if *character == '-' && rng.gen_range(0..30) == 10 {
				*character = 'B';
			}

			if *character == '-' && rng.gen_range(0..100) == 3 {
				*character = 'D';
			}

			if *character == '-' && rng.gen_range(0..50) == 5 {
				*character = 'H';
			}

			//place seed walls
			cell_count += 1;
			for location in new_room.seed_wall_locations {
				if cell_count == location && *character != 'U'{
					*character = '#';
				}
			}
		}
	}
	


	for _p in 0..P {
		for i in 1..ROOM_HEIGHT-1 {
			for j in 1..ROOM_WIDTH-1 {
				let mut neighboring_walls = 0;
				if new_room.room_coords[i-1][j-1] == '#' || new_room.room_coords[i-1][j-1] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i-1][j] == '#' || new_room.room_coords[i-1][j] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i-1][j+1] == '#' || new_room.room_coords[i-1][j+1] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i][j-1] == '#' || 	new_room.room_coords[i][j-1] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i][j+1] == '#' || new_room.room_coords[i][j+1] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i+1][j-1] == '#' || new_room.room_coords[i+1][j-1] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i+1][j] == '#' || new_room.room_coords[i+1][j] == 'U' {neighboring_walls += 1;}
				if new_room.room_coords[i+1][j+1] == '#' || new_room.room_coords[i+1][j+1] == 'U' {neighboring_walls += 1;}
	
				if neighboring_walls >= T {
					new_room.room_coords[i][j] = '#';
				}
			}
		}
	}
	for (i, row) in new_room.room_coords.iter_mut().enumerate() {
		for (j, character) in row.iter_mut().enumerate() {
			
			if (i == 0 && exits[TOP])|| (j == 0 && exits[LEFT]) || (i == ROOM_HEIGHT - 1 && exits[BOTTOM]) || (j == ROOM_WIDTH - 1 && exits[RIGHT]){
				*character = '-';//place the exits at the end
			}
			if (i == 0 && !exits[TOP])|| (j == 0 && !exits[LEFT]) || (i == ROOM_HEIGHT - 1 && !exits[BOTTOM]) || (j == ROOM_WIDTH - 1 && !exits[RIGHT]){
				//surround outside of room with walls
				*character = 'U';
			}
		}
	}
	new_room
}

fn gen_seed_wall_locations() -> [usize;N] {
	let mut rng = thread_rng();
	let mut arr: [usize;N] = [0;N];	
	for num in arr.iter_mut() {
		*num = rng.gen_range(0..ROOM_WIDTH*ROOM_HEIGHT);
	} 
	//info!("{:?}", arr);
	arr
}

fn despawn_all(
	mut entity: Query<Entity,Without<Map>>,
	mut commands: Commands
){
	for e in entity.iter_mut() {
        commands.entity(e).despawn();
    }
	let camera = OrthographicCameraBundle::new_2d();
	commands.spawn_bundle(camera);

	commands.insert_resource(NextState(GameState::Playing));

}


