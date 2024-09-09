use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod plugins;

use plugins::car::CarPlugin;
use plugins::scene::ScenePlugin;
use plugins::state::SimStatePlugin;
use plugins::traffic_pole::TrafficPolePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((SimStatePlugin, ScenePlugin, TrafficPolePlugin, CarPlugin))
        .run();
}
