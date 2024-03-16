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

    // Distribute colors evenly across the rainbow.

    let size = 50.0;
    let width = 3.0_f32.sqrt() * size;
    let height = 3.0 / 2.0 * size;

    for w in 0..6 {
        for h in 0..6 {
            let x = width * (w as f32) + (h % 2) as f32 * width / 2.0;
            let y = height * (h as f32);

            let shape = Mesh2dHandle(meshes.add(RegularPolygon::new(size, 6)));
            let color = Color::hsl(44. * w as f32 + 9.0 * h as f32, 0.95, 0.7);

            commands.spawn(MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_xyz(
                    x,
                    y,
                    0.0,
                ),
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}