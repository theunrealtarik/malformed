use bevy::prelude::*;
use discord_rich_presence::{
    activity::{self, Assets, Button},
    DiscordIpc, DiscordIpcClient,
};
use glib::{DISCORD_APP_ID, DISCORD_LARGE_IMAGE, DISCORD_STATE};

#[derive(Resource)]
pub struct DiscordRPC {
    client: DiscordIpcClient,
}

impl Default for DiscordRPC {
    fn default() -> Self {
        Self {
            client: DiscordIpcClient::new(DISCORD_APP_ID).unwrap(),
        }
    }
}

pub struct RPCPlugin;

impl Plugin for RPCPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiscordRPC>()
            .add_systems(PreStartup, Self::setup);
    }
}

impl RPCPlugin {
    fn setup(mut rpc: ResMut<DiscordRPC>) {
        if rpc.client.connect().is_ok() {
            let payload = activity::Activity::new()
            .details("Komboter is not dying, not today.")
                .assets(
                    Assets::new()
                        .large_text(DISCORD_STATE)
                        .large_image(DISCORD_LARGE_IMAGE),
                )
                .buttons(vec![
                    Button::new("Play", env!("CARGO_PKG_HOMEPAGE")),
                    Button::new("BSoD", "https://www.youtube.com/watch?v=njFw1NOAu3s"),
                ]);

            rpc.client.set_activity(payload).unwrap();
        }
    }
}
