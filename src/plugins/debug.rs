use std::marker::PhantomData;

use crate::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;

pub use bevy_inspector_egui::prelude::*;
pub use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
pub use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins(StateInspectorPlugin::<GameState>::default())
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

pub struct EntityInspector<E> {
    marker: PhantomData<E>,
}

impl<E> Default for EntityInspector<E> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<E: 'static> Plugin for EntityInspector<E>
where
    E: Component,
{
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins(FilterQueryInspectorPlugin::<With<E>>::default());
        }
    }
}
