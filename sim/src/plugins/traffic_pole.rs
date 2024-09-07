use std::{f32::consts::PI, time::Duration};

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

const TRAFFIC_POLE_PATH: &str = "Traffic Light.glb";

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

fn dtr(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let positions = [
        (Vec2::new(-100.0, 100.0), Quat::from_rotation_y(dtr(270.0))),
        (Vec2::new(-100.0, -100.0), Quat::from_rotation_y(dtr(180.0))),
        (Vec2::new(100.0, -100.0), Quat::from_rotation_y(dtr(90.0))),
        (Vec2::new(100.0, 100.0), Quat::from_rotation_y(dtr(0.0))),
    ];

    for (position, rotation) in positions {
        commands.spawn(SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(TRAFFIC_POLE_PATH)),
            transform: Transform::from_translation(Vec3::new(position.x, 0.0, position.y))
                .with_rotation(rotation)
                .with_scale(Vec3::new(20.0, 20.0, 20.0)),
            ..default()
        });
    }

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
