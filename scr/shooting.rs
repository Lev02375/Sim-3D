use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{Target, Score};

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot);
    }
}

fn shoot(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Query<&RapierContext>,
    mut targets: Query<(Entity, &mut Transform), With<Target>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(rapier) = rapier_context.get_single() else { return };

    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    
    let Some(ray) = camera.viewport_to_world(camera_transform, center) else {
        return;
    };

    let filter = QueryFilter::default();
    let max_distance = 100.0;

    if let Some((entity, _toi)) = rapier.cast_ray(
        ray.origin,
        *ray.direction,
        max_distance,
        true,
        filter,
    ) {
        if let Ok((target_entity, mut transform)) = targets.get_mut(entity) {
            score.0 += 10;
            
            transform.scale = Vec3::splat(0.1);
            commands.entity(target_entity).despawn();
            
            println!("Попадание! Счёт: {}", score.0);
        }
    }
}
