use std::f32::consts::PI;

use bevy::app::Plugin;
use bevy::prelude::*;

pub struct CarPlugin;

#[derive(Component)]
struct Speed(f32);

impl Plugin for CarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_cars);
    }
}

fn spawn_cars(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("vehicles/suv.glb")),

        transform: Transform::from_translation(Vec3::new(150.0, 0.0, -20.0))
            .with_rotation(Quat::from_rotation_y(3.0 * PI / 2.0))
            .with_scale(Vec3::new(20.0, 20.0, 20.0)),
        ..default()
    });
}
