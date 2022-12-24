use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn spawn_point(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    radius: f32,
    transform: Transform,
    component: impl Component
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: transform.with_scale(radius * Vec3::ONE),
        ..default()
    }, component));
}