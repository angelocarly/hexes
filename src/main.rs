use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shape = Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6)));

    // Distribute colors evenly across the rainbow.
    let color = Color::hsl(190., 0.95, 0.7);

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}