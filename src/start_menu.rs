use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App){

    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>){
    commands.spawn_bundle(UiCameraBundle::default());
    
}