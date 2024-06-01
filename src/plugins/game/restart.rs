use bevy::prelude::*;

pub struct GameRestartPlugin;

use crate::plugins::entities::player::*;
use crate::plugins::entities::terrain::*;
use crate::GameState;

impl Plugin for GameRestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::restart
                .run_if(in_state(GameState::Resumed))
                .run_if(in_state(Being::Dead)),
        )
        .add_systems(
            OnTransition {
                from: Being::Dead,
                to: Being::Alive,
            },
            (
                BuildingsPlugin::setup,
                EnvironmentPlugin::setup,
                (PlayerPlugin::setup, PlayerPlugin::restart).chain(),
            )
                .chain(),
        );
    }
}

impl GameRestartPlugin {
    fn restart(
        mut commands: Commands,
        platforms_query: Query<Entity, With<Platform>>,
        player_query: Query<Entity, With<Player>>,
        mut next_being: ResMut<NextState<Being>>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        if input.just_pressed(KeyCode::KeyR) {
            for platform in platforms_query.iter() {
                commands.entity(platform).despawn_recursive();
            }

            if let Ok(player) = player_query.get_single() {
                commands.entity(player).despawn_recursive();
            }

            next_being.set(Being::Alive);
        }
    }
}
