use std::{f32::consts::PI, iter::Enumerate, time::Duration};

use bevy::{
    app::{Plugin, Startup},
    color::palettes::css::{LIME, RED, YELLOW},
    prelude::*,
};

const TRAFFIC_POLE_PATH: &str = "models/Traffic Light.glb";

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pole_positions = [
        (Vec2::new(-100.0, 100.0), Quat::from_rotation_y(dtr(270.0))),
        (Vec2::new(-100.0, -100.0), Quat::from_rotation_y(dtr(180.0))),
        (Vec2::new(100.0, -100.0), Quat::from_rotation_y(dtr(90.0))),
        (Vec2::new(100.0, 100.0), Quat::from_rotation_y(dtr(0.0))),
    ];
    let light_positions = [LIME, YELLOW, RED];

    for (pole_index, (pole_position, rotation)) in pole_positions.iter().enumerate() {
        commands
            .spawn((
                Name::new(format!("Pole {pole_index}")),
                SceneBundle {
                    scene: asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset(TRAFFIC_POLE_PATH)),
                    transform: Transform::from_translation(Vec3::new(
                        pole_position.x,
                        0.0,
                        pole_position.y,
                    ))
                    .with_rotation(*rotation)
                    .with_scale(Vec3::new(20.0, 20.0, 20.0)),
                    ..default()
                },
            ))
            .with_children(|builder| {
                for (light_index, light_color) in light_positions.iter().enumerate() {
                    let color: Color = light_color.to_owned().into();
                    builder
                        .spawn((
                            Name::new(format!("Light {light_index}")),
                            SpotLightBundle {
                                visibility: Visibility::Hidden,
                                transform: Transform::from_xyz(
                                    -3.77,
                                    3.5 + (light_index as f32) * 0.3,
                                    0.05,
                                )
                                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
                                spot_light: SpotLight {
                                    intensity: 100_000_000.0,
                                    range: 100_000_000.0,
                                    color: color,
                                    shadows_enabled: false,
                                    inner_angle: 0.1,
                                    outer_angle: 0.1,
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn(PbrBundle {
                                transform: Transform::from_rotation(Quat::from_rotation_y(PI)),
                                mesh: meshes.add(Cylinder::new(0.1, 0.1)),
                                material: materials.add(StandardMaterial {
                                    base_color: color,
                                    emissive: match *light_color {
                                        LIME => LinearRgba::new(0.0, 4.0, 0.0, 1.0),
                                        RED => LinearRgba::new(4.0, 0.0, 0.0, 1.0),
                                        YELLOW => LinearRgba::new(4.0, 4.0, 0.0, 1.0),
                                        _ => LinearRgba::new(0.0, 0.0, 0.0, 1.0),
                                    },
                                    ..default()
                                }),
                                ..default()
                            });
                        });
                }
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
