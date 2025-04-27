//! Loads and renders a glTF file as a scene.

use bevy::{
    color::palettes::basic::SILVER,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use std::f32::consts::*;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -5.0, 12.0).looking_at(Vec3::new(0.0, -1.0, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.1, -FRAC_PI_4)),
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
    ));
    // Base plane
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d {
                    normal: Dir3::from_xyz_unchecked(0., 0., 1.),
                    half_size: Vec2::new(4.0, 6.0),
                }
                .mesh()
                .subdivisions(10),
            ),
        ),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
    // The ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius: 0.15 }.mesh().ico(5).unwrap())),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}
