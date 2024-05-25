use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::setup);
    }
}

impl CameraPlugin {
    fn setup(mut commands: Commands) {
        commands
            .spawn(Camera2dBundle::default())
            .insert(MainCamera)
            .insert(Name::new("Main Camera"));
    }
}
