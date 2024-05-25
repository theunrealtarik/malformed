use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins(StateInspectorPlugin::<GameState>::default())
                .register_type::<Acceleration>()
                .register_type::<Velocity>()
                .add_plugins(WorldInspectorPlugin::default())
                .add_plugins(FrameTimeDiagnosticsPlugin)
                .add_systems(Update, Self::inspector_ui);
        }
    }
}

impl DebugPlugin {
    fn inspector_ui(world: &mut World) {
        use bevy_egui::egui::*;

        let mut egui_context = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .single(world)
            .clone();

        Window::new("Performance").show(egui_context.get_mut(), |ui| {
            ScrollArea::both().show(ui, |ui| {
                if let Some(diagnostics) = world.get_resource::<DiagnosticsStore>() {
                    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            ui.label(format!("FPS: {}", value));
                        }
                    }
                }
            });
        });
    }
}
