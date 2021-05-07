//! This crate provides a [Bevy](https://bevyengine.org/) plugin for integrating with
//! the Discord Game SDK.
//!
//! ## Installation
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bevy-discord-game-sdk = "0.1.0"
//! ```
//!
//! Ensure that your computer has all the needed
//! [requirements](https://rust-lang.github.io/rust-bindgen/requirements.html) to use
//! [bindgen](https://github.com/rust-lang/rust-bindgen).
//!
//! Download and install the [Discord Game SDK](https://discord.com/developers/docs/game-sdk/sdk-starter-guide)
//! and set the environment variable `DISCORD_GAME_SDK_LOCATION` to point to it.
//!
//! ## Usage
//!
//! To add the plugin to your game, simply add the `DiscordPlugin` to your
//! `AppBuilder`.
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_discord_game_sdk::DiscordPlugin;
//!
//! fn main() {
//!   App::build()
//!       .add_plugins(DefaultPlugins)
//!       .add_plugin(DiscordPlugin)
//!       .run()
//! }
//! ```
//!
//! The plugin adds `discord_game_sdk::Discord` as a Bevy ECS non-send resource,
//! The Discord Game SDK is not threadsafe at all, and all operations using the
//! client will run on the main thread.
//!
//! The plugin will automatically call `Discord::run_callbacks` on the Bevy
//! main thread every frame, so there is no need to run it manually.
//!
//! **NOTE**: If the plugin fails to initialize (i.e. `Discord::new()` fails and
//! returns an error, an error wil lbe logged (via `bevy_log`), but it will not
//! panic. In this case, it may be necessary to use `Option<NonSend<Discord>>` instead.
//!
//! ```rust
//! use bevy_discord_game_sdk::{Client, FriendFlags};
//!
//! fn discord_system(client: NonSend<Client>) {
//!   for friend in client.friends().get_friends(FriendFlags::IMMEDIATE) {
//!     println!("Friend: {:?} - {}({:?})", friend.id(), friend.name(), friend.state());
//!   }
//! }
//!
//! fn main() {
//!   App::build()
//!       .add_plugins(DefaultPlugins)
//!       .add_plugin(DiscordPlugin)
//!       .add_startup_system(discord_system.system())
//!       .run()
//! }
//! ```

use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::system::{IntoSystem, NonSend};
use bevy_log::error;
pub use discord_game_sdk::*;

fn run_discord_callbacks(client: NonSend<Discord>) {
    client.run_callbacks();
}

pub struct DiscordPlugin(ClientID);

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut AppBuilder) {
        match Discord::new(self.0) {
            Err(err) => error!("Failed to initialize Discord client: {}", err),
            Ok(client) => {
                app.insert_non_send_resource(client)
                    .add_system(run_discord_callbacks.system());
            }
        }
    }
}
