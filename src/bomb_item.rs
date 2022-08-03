use bevy::prelude::*;
use iyes_loopless::prelude::*;




use crate::{
	//for example bomb spawn
	//WIN_W,
	GameState,
	loading::{
		LoadingAssets,
		LoadingAssetInfo,
	}
};

#[derive(Component)]
pub struct Bomb{
	y_velocity: f32,
	x_velocity: f32,
	//grounded: bool,
}

#[derive(Deref, DerefMut)]
pub struct BombSheet(Handle<TextureAtlas>);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedStep;

pub struct BombPlugin;
impl Plugin for BombPlugin {
	fn build (&self, app: &mut App) {
		//let every_second = SystemStage::parallel();
		//let mut every_frame = SystemStage::parallel();
		/* 
		every_frame.add_system_set(
				ConditionSet::new()
					.run_in_state(GameState::Playing)
					.with_system(animate_bomb)
					.into()
					);
		 */
		app.add_enter_system(GameState::Loading, load_bomb_sheet);
		/* 
		.add_stage_before(
			CoreStage::Update,
			FixedStep,
			FixedTimestepStage::from_stage(Duration::from_micros(16667), every_frame) // ~1 frame at 60 fps
		*/
		//)
		//.add_stage_before(
		//	CoreStage::Update,
		//	FixedStep,
		//	FixedTimestepStage::new(Duration::from_secs(1))
		//		.with_stage(every_second)
	}
}

fn load_bomb_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut loading_assets: ResMut<LoadingAssets>,
) {
	let bomb_handle = asset_server.load("bomb_boom.png");
	loading_assets.insert(
		bomb_handle.clone_untyped(),
		LoadingAssetInfo::for_handle(bomb_handle.clone_untyped(), &asset_server),
	);

	let bomb_atlas = TextureAtlas::from_grid(bomb_handle, Vec2::splat(35.), 6, 1);
	let bomb_atlas_handle = texture_atlases.add(bomb_atlas);
	
	commands.insert_resource(BombSheet(bomb_atlas_handle));
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
	//info!("tick");
	//let (entity, mut bomb, mut sprite, texture_atlas_handle, mut timer) = bomb.single_mut();
	for (entity, _bomb, mut sprite, texture_atlas_handle, mut timer) in bomb.iter_mut() {
		
		//let ground = bomb.grounded;
		//info!("bomb");
		timer.tick(time.delta());

		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index += 1;// % texture_atlas.textures.len();

			if sprite.index >= texture_atlas.textures.len(){
				commands.entity(entity).despawn();
			}
		
		}
	}
}
