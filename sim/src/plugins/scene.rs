use std::f32::consts::PI;

use bevy::{
    app::{Plugin, Startup},
    input::mouse::MouseMotion,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
};

const SCENE_PATH: &str = "scene.glb";

#[derive(Component)]
pub struct WorldCamera;

#[derive(Component)]
pub struct Ground;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (move_viewer_camera, pan_camera));
    }
}

fn move_viewer_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<WorldCamera>>,
) {
    const SPEED: f32 = 100.0;

    if let Ok(mut camera_transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += camera_transform.forward().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += camera_transform.back().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += camera_transform.right().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += camera_transform.left().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            direction += camera_transform.up().as_vec3();
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            direction += camera_transform.down().as_vec3();
        }

        let movement = direction.normalize_or_zero() * SPEED * time.delta_seconds();
        camera_transform.translation += movement;
    }
}

fn pan_camera(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<WorldCamera>>,
) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        if mouse_button_input.pressed(MouseButton::Middle) {
            for event in mouse_motion_event.read() {
                let delta = event.delta;

                if delta != Vec2::ZERO {
                    let yaw = -delta.x * 0.003;
                    let pitch = -delta.y * 0.002;
                    camera_transform.rotate_y(yaw);
                    camera_transform.rotate_local_x(pitch);
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn((
        WorldCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(50.0f32, 50.0f32, 50.0f32)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(200., 200.)),
            material: materials.add(Color::BLACK.with_alpha(20.0)),
            ..default()
        },
        Ground,
    ));
}