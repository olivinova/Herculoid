use bevy::{DefaultPlugins, prelude::App};
use acceleration::{};
use physics_manager::{};
use collision::{};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(acceleration)
    .add_plugin(physics_manager)
    .add_plugin(collision)
    .run();
}
