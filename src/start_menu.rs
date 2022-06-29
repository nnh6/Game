use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App){

    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>){
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
        ..Default::default()
    }); 
}