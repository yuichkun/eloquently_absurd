use bevy::prelude::*;

mod hello;
use hello::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
