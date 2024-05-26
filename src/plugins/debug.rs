use std::marker::PhantomData;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::view::VisibleEntities;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;

pub use bevy_inspector_egui::prelude::*;
pub use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
pub use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::*;

pub struct DebugPlugin;

#[derive(Default, Reflect, Resource, InspectorOptions)]
struct DebugStates {
    game_state: GameState,
    assets_state: GameAssetsState,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            // app.add_plugins(StateInspectorPlugin::<GameState>::default());
            app.init_resource::<DebugStates>()
                .add_plugins(ResourceInspectorPlugin::<DebugStates>::new());

            app.add_plugins(WorldInspectorPlugin::default())
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
