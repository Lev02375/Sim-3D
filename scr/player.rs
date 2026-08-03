use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (player_move, player_look));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub sensitivity: f32,
}

#[derive(Component)]
pub struct CameraPivot;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            speed: 8.0,
            sensitivity: 0.003,
        },
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule_y(0.9, 0.4),
        Velocity::zero(),
        Damping {
            linear_damping: 2.0,
            angular_damping: 2.0,
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            CameraPivot,
            Transform::from_xyz(0.0, 0.8, 0.0),
        ))
        .with_children(|pivot| {
            pivot.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::NEG_Z, Vec3::Y),
            ));
        });
    });
}

fn player_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Velocity, &Transform)>,
) {
    let Ok((player, mut velocity, transform)) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction += transform.back().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction += transform.left().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += transform.right().as_vec3();
    }

    direction.y = 0.0;
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let current_vel = velocity.linvel;
    let target_vel = direction * player.speed;
    let new_vel = Vec3::new(target_vel.x, current_vel.y, target_vel.z);
    
    velocity.linvel = new_vel;
}

fn player_look(
    mut mouse_events: EventReader<bevy::input::mouse::MouseMotion>,
    mut pivot_query: Query<&mut Transform, With<CameraPivot>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<CameraPivot>)>,
) {
    let Ok(mut pivot) = pivot_query.get_single_mut() else { return };
    let Ok(mut player) = player_query.get_single_mut() else { return };

    let mut delta = Vec2::ZERO;
    for event in mouse_events.read() {
        delta += event.delta;
    }

    if delta.length_squared() > 0.0 {
        let sensitivity = 0.003;
        
        player.rotate_y(-delta.x * sensitivity);
        
        let (scale, rotation, translation) = pivot.to_scale_rotation_translation();
        let mut euler = rotation.to_euler(EulerRot::YXZ);
        euler.1 -= delta.y * sensitivity;
        euler.1 = euler.1.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        pivot.rotation = Quat::from_euler(EulerRot::YXZ, euler.0, euler.1, euler.2);
        pivot.translation = translation;
        pivot.scale = scale;
    }
  }
