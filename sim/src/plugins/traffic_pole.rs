use std::time::Duration;

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

const TRAFFIC_POLE_PATH: &str = "Three way traffic light.glb";

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

pub struct TrafficPolePlugin;

impl Plugin for TrafficPolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(TRAFFIC_POLE_PATH)),
        ..default()
    });

    // let mut graph = AnimationGraph::new();
    // let animations = graph
    //     .add_clips(
    //         [
    //             GltfAssetLabel::Animation(1).from_asset(TRAFFIC_POLE_PATH),
    //             GltfAssetLabel::Animation(0).from_asset(TRAFFIC_POLE_PATH),
    //         ]
    //         .into_iter()
    //         .map(|path| asset_server.load(path)),
    //         1.0,
    //         graph.root,
    //     )
    //     .collect();

    // // Insert a resource with the current scene information
    // let graph = graphs.add(graph);
    // commands.insert_resource(Animations {
    //     animations,
    //     graph: graph.clone(),
    // });
}

fn animate(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}
