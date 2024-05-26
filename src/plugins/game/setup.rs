use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use bevy_rapier2d::prelude::*;

use super::prelude::Responsive;
use glib::*;

pub struct GameEssentialsPlugin;

impl Plugin for GameEssentialsPlugin {
    fn build(&self, app: &mut App) {
        use glib::{APP_WINDOW_NAME, WORLD_BACKGROUND_COLOR};

        app.insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
            .insert_resource(Msaa::Off)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            name: Some(APP_WINDOW_NAME.into()),
                            title: APP_WINDOW_NAME.into(),
                            resolution: WindowResolution::new(
                                APP_WINDOW_MIN_WIDTH,
                                APP_WINDOW_MIN_HEIGHT,
                            ),
                            present_mode: PresentMode::AutoVsync,
                            mode: WindowMode::BorderlessFullscreen,
                            resizable: false,
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_systems(PostStartup, Self::rescale_sprites);
    }
}

impl GameEssentialsPlugin {
    fn rescale_sprites(mut query: Query<&mut Transform, With<Responsive>>, window: Query<&Window>) {
        let window = window.single();

        for mut transform in query.iter_mut() {
            utils::rescale(&mut transform, window.width(), window.height());
        }
    }
}