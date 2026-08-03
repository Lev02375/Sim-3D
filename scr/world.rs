use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_ground, spawn_walls, spawn_targets));
    }
}

#[derive(Component)]
pub struct Target;

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    commands.spawn((
        PointLight {
            intensity: 200000.0,
            color: Color::srgb(0.8, 0.2, 0.9),
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.15),
            metallic: 0.1,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(25.0, 0.1, 25.0),
        RigidBody::Fixed,
    ));
}

fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.5, 0.8),
        emissive: LinearRgba::rgb(0.05, 0.1, 0.2),
        metallic: 0.6,
        perceptual_roughness: 0.3,
        ..default()
    });

    let walls = vec![
        (Vec3::new(-10.0, 1.5, -10.0), Vec3::new(1.0, 3.0, 8.0)),
        (Vec3::new(10.0, 1.5, -5.0), Vec3::new(1.0, 3.0, 10.0)),
        (Vec3::new(0.0, 1.5, -15.0), Vec3::new(12.0, 3.0, 1.0)),
        (Vec3::new(-5.0, 1.5, 5.0), Vec3::new(8.0, 3.0, 1.0)),
        (Vec3::new(8.0, 1.5, 8.0), Vec3::new(1.0, 3.0, 6.0)),
        (Vec3::new(-8.0, 1.5, 12.0), Vec3::new(6.0, 3.0, 1.0)),
    ];

    for (pos, size) in walls {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(wall_material.clone()),
            Transform::from_translation(pos),
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            RigidBody::Fixed,
        ));
    }
}

fn spawn_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();
    
    for _ in 0..8 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        let y = rng.gen_range(1.5..3.0);
        
        let color = Color::srgb(
            rng.gen_range(0.5..1.0),
            rng.gen_range(0.2..0.8),
            rng.gen_range(0.3..1.0),
        );

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::rgb(color.to_srgba().red * 0.5, color.to_srgba().green * 0.5, color.to_srgba().blue * 0.5),
                metallic: 0.8,
                perceptual_roughness: 0.2,
                ..default()
            })),
            Transform::from_xyz(x, y, z),
            Collider::ball(0.5),
            RigidBody::Fixed,
            Target,
        ));
    }
}
