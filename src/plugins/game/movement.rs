use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::{GameAssetsState, GameState};

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn value(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Acceleration {
    pub fn value(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    velocity: Velocity,
    acceleration: Acceleration,
}

impl From<Vec3> for Velocity {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vec3> for Acceleration {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::update_velocity, Self::update_position)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GameAssetsState::Loaded)),
        )
        .register_type::<Acceleration>()
        .register_type::<Velocity>();
    }
}

impl MovementPlugin {
    pub fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
        for (mut v, a) in query.iter_mut() {
            v.x += a.x * time.delta_seconds();
            v.y += a.y * time.delta_seconds();
            v.z += a.z * time.delta_seconds();
        }
    }

    pub fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
        for (mut t, v) in query.iter_mut() {
            t.translation.x += v.x * time.delta_seconds();
            t.translation.y += v.y * time.delta_seconds();
            t.translation.z += v.z * time.delta_seconds();
        }
    }
}
