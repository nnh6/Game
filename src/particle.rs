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
	FRAME_TIME,
};

#[derive(Component)]
pub struct Particle{
	y_velocity: f32,
	x_velocity: f32,
	frames: f32,
	//grounded: bool,
}

#[derive(Deref, DerefMut)]
pub struct ParticleSheet(Handle<TextureAtlas>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedStep;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
	fn build (&self, app: &mut App) {
		let every_second = SystemStage::parallel();
		let mut every_frame = SystemStage::parallel();

		every_frame.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					//.with_system(animate_bomb)
					.into()
					);
		
		app.add_enter_system(GameState::Loading, load_particle_sheet)
		//.add_enter_system(GameState::Playing, spawn_particle)
		.add_stage_before(
			CoreStage::Update,
			FixedStep,
			FixedTimestepStage::from_stage(Duration::from_micros(16667), every_frame) // ~1 frame at 60 fps
		);
		//.add_stage_before(
		//	CoreStage::Update,
		//	FixedStep,
		//	FixedTimestepStage::new(Duration::from_secs(1))
		//		.with_stage(every_second)
		//);
	}
}

fn load_particle_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {}

fn spawn_bomb(
	mut commands: Commands,
	bomb_sheet: Res<BombSheet>,
){
}
