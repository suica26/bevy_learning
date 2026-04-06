use crate::plugins::HelloPlugin;

mod components;
mod plugins;
mod resources;
mod systems;

fn main() {
    bevy::app::App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
