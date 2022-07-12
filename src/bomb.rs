use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

use crate::{
	//for example bomb spawn
	//WIN_W,
	WIN_H,
	TILE_SIZE,
	ANIM_TIME,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	},
};

#[derive(Component)]
pub struct Bomb;

#[derive(Deref, DerefMut)]
pub struct BombSheet(Handle<TextureAtlas>);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedStep;

pub struct BombPlugin;
impl Plugin for BombPlugin {
	fn build (&self, app: &mut App) {
		let mut every_second = SystemStage::parallel();
		app.add_enter_system(GameState::Loading, load_bomb_sheet)
		.add_enter_system(GameState::Playing, spawn_bomb)
		.add_system_set(
			ConditionSet::new()
				.run_in_state(GameState::Playing)
				.with_system(animate_bomb)
				.into()
				)
		.add_stage_before(
				CoreStage::Update,
				FixedStep,
				FixedTimestepStage::new(Duration::from_secs(1))
					.with_stage(every_second)
			);
	}
}

fn load_bomb_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let bomb_handle = asset_server.load("bomb175.png");
	loading_assets.insert(
		bomb_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bomb_handle.clone_untyped(), &asset_server),
	);

	let bomb_atlas = TextureAtlas::from_grid(bomb_handle, Vec2::splat(35.), 5, 1);
	let bomb_atlas_handle = texture_atlases.add(bomb_atlas);
	
	commands.insert_resource(BombSheet(bomb_atlas_handle));
}

fn spawn_bomb(
	mut commands: Commands,
	bomb_sheet: Res<BombSheet>,
){
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: bomb_sheet.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform::from_xyz(200., -(WIN_H/2.) + (TILE_SIZE * 1.22), 900.),
			..default()
		})
		.insert(AnimationTimer(Timer::from_seconds(ANIM_TIME, true)));

}

fn animate_bomb( //not complete yet
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut bomb: Query<
		(
			Entity,
			&mut Bomb,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
			&mut AnimationTimer,
		),
		With<Bomb>
	>,
	mut commands: Commands,
){
	//let (entity, mut bomb, mut sprite, texture_atlas_handle, mut timer) = bomb.single_mut();
	for (entity, mut bomb, mut sprite, texture_atlas_handle, mut timer) in bomb.iter_mut() {
		
		info!("bomb"); //this doesn't go off
		timer.tick(time.delta());
		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1);
			if sprite.index >= texture_atlas.textures.len(){
				commands.entity(entity).despawn();
			}
		
		}
	}
}