pub mod assets;
pub mod menu;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
    winit::WinitSettings,
};

// setup
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        use glib::{APP_WINDOW_NAME, WORLD_BACKGROUND_COLOR};

        app.insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
            .insert_resource(WinitSettings::desktop_app())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some(APP_WINDOW_NAME.into()),
                    title: APP_WINDOW_NAME.into(),
                    resolution: WindowResolution::new(600.0, 400.0),
                    present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::BorderlessFullscreen,
                    ..Default::default()
                }),
                ..Default::default()
            }))
            .add_systems(Startup, Self::setup_camera)
            .add_systems(Update, Self::toggle_fullscreen);
    }
}

impl GameSetupPlugin {
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }

    fn toggle_fullscreen(mut window: Query<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
        let mut window = window.single_mut();
        if input.just_pressed(KeyCode::F11) {
            match window.mode {
                WindowMode::BorderlessFullscreen => window.mode = WindowMode::Windowed,
                _ => window.mode = WindowMode::BorderlessFullscreen,
            }
        }
    }
}
