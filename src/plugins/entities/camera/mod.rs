use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::setup);
    }
}

impl GameCameraPlugin {
    fn setup(mut commands: Commands) {
        commands
            .spawn(Camera2dBundle::default())
            .insert(MainCamera)
            .insert(Name::new("Main Camera"));
    }
}
