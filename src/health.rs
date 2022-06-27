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
	level::Background, HEALTH,
};

#[derive(Component)]
pub struct Health; //

//#[derive(Deref, DerefMut)]
//pub struct PlayerSheet(Handle<TextureAtlas>); //
#[derive(Deref, DerefMut)]
pub struct HealthAtlas(Handle<TextureAtlas>);

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
	fn build (&self, app: &mut App) {
		app.add_enter_system(GameState::Loading, load_health_sheet);
			//.add_enter_system(GameState::Playing, spawn_health);
	}
}

fn load_health_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
){
	
	let hp_handle = asset_server.load("Health_Hearts_Large.png");
	loading_assets.insert(
		hp_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(hp_handle.clone_untyped(), &asset_server),
	);

	let hp_atlas = TextureAtlas::from_grid(hp_handle, Vec2::splat(TILE_SIZE), 2, 6);
	let hp_atlas_handle = texture_atlases.add(hp_atlas);

	commands.insert_resource(HealthAtlas(hp_atlas_handle));
	
}
	//let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(100.), 2, 6);
	//let player_atlas_handle = texture_atlases.add(player_atlas);
	
	//commands.insert_resource(PlayerSheet(player_atlas_handle));

	////
/*
fn load_health_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	
	let player_handle = asset_server.load("Health_Hearts_Small.png");
	loading_assets.insert(
		player_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(player_handle.clone_untyped(), &asset_server),
	);

	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::splat(100.), 2, 6);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	
	commands.insert_resource(PlayerSheet(player_atlas_handle));
}*/

/* 
fn spawn_health(
	mut commands: Commands,
	health_sheet: Res<HealthAtlas>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: health_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(-(WIN_W/2.), -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		});
	
}*/

fn update_health(){//not completed
	//let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
	//sprite.index = texture_atlas.textures.len() - (HEALTH/10.).round(); //Use health to determine the index of the health sprite to show
}

/*
fn spawn_health(
	mut commands: Commands,
	//health_image: Res<HealthImage>,
){ 
	commands
		.spawn_bundle(SpriteBundle{ //SpriteSheetBundle {
			//texture: health_image.0.clone(),
			//sprite: TextureAtlasSprite {
			//	index: 0,
			//	..default()
			//},
			transform: Transform::from_xyz(-(WIN_W/2.), -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		})
		.insert(Health);
}*/

/*
fn animate_player(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut player: Query<
		(
			&Velocity,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
		),
		With<Player>
	>,
){
	let (velocity, mut sprite, texture_atlas_handle, mut timer) = player.single_mut();
	if velocity.cmpne(Vec2::ZERO).any() {
		timer.tick(time.delta());

		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}*/

/* 
fn load_health_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let health_handle = asset_server.load("Health_Hearts_Small.png");
	loading_assets.insert(
		health_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(health_handle.clone_untyped(), &asset_server),
	);

	let health_atlas = TextureAtlas::from_grid(health_handle, Vec2::splat(1209.), 2, 6);
	let health_atlas_handle = texture_atlases.add(health_atlas);
	
	commands.insert_resource(HealthSheet(health_atlas_handle));
}

fn spawn_health(
	mut commands: Commands,
	health_sheet: Res<HealthSheet>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: health_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(-(WIN_W/2.), -(WIN_H/2.) + (TILE_SIZE * 1.5), 900.),
			..default()
		});
}*/