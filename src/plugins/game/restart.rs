use std::time::Duration;

use bevy::prelude::*;

pub struct GameRestartPlugin;

use crate::plugins::entities::player::*;
use crate::plugins::entities::terrain::*;
use crate::GameState;

#[derive(Component)]
struct RestartTimer {
    timer: Timer,
}

impl Default for RestartTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
        }
    }
}

impl Plugin for GameRestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::restart)
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
        )
        .add_systems(OnEnter(Being::Dead), Self::setup_restart_timer);
    }
}

impl GameRestartPlugin {
    fn restart(
        mut commands: Commands,
        mut next_being: ResMut<NextState<Being>>,
        platforms_query: Query<Entity, With<Platform>>,
        player_query: Query<Entity, With<Player>>,
        mut restart_timer: Query<(Entity, &mut RestartTimer)>,
        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
    ) {
        let Ok((restart_timer_entity, mut restart_timer)) = restart_timer.get_single_mut() else {
            return;
        };

        let tick = restart_timer.timer.tick(time.delta());

        if tick.finished() && input.just_pressed(KeyCode::KeyR) {
            for platform in platforms_query.iter() {
                commands.entity(platform).despawn_recursive();
            }

            if let Ok(player) = player_query.get_single() {
                commands.entity(player).despawn_recursive();
            }

            next_being.set(Being::Alive);
            commands.entity(restart_timer_entity).despawn_recursive();
        }
    }

    fn setup_restart_timer(mut commands: Commands) {
        commands
            .spawn(RestartTimer::default())
            .insert(Name::new("Restart Timer"));
    }
}
