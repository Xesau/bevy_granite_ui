use bevy::{camera::visibility::RenderLayers, prelude::*};

mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::UiPlugin {
            editor_render_layer: 1,
        })
        // Setup window
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Circle::new(10.0)));
    let material = materials.add(StandardMaterial::default());
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    let mesh = meshes.add(Mesh::from(Rectangle::new(10.0, 10.0)));
    let material = materials.add(StandardMaterial::default());
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-10.0, 2.0, 10.0)).looking_at(Vec3::ZERO, Dir3::Y),
        RenderLayers::layer(0),
        Camera {
            order: 0,
            ..default()
        }
    ));
}
