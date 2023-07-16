use azalea_core::{Vec3, AABB};
use bevy_ecs::{query::Changed, system::Query};

use super::{Physics, Position};

#[derive(Debug, Default, Clone)]
pub struct EntityDimensions {
    pub width: f32,
    pub height: f32,
}

impl EntityDimensions {
    pub fn make_bounding_box(&self, pos: &Vec3) -> AABB {
        let radius = (self.width / 2.0) as f64;
        let height = self.height as f64;
        AABB {
            min_x: pos.x - radius,
            min_y: pos.y,
            min_z: pos.z - radius,

            max_x: pos.x + radius,
            max_y: pos.y + height,
            max_z: pos.z + radius,
        }
    }
}

/// Sets the position of the entity. This doesn't update the cache in
/// azalea-world, and should only be used within azalea-world!
///
/// # Safety
/// Cached position in the world must be updated.
pub fn update_bounding_box(mut query: Query<(&Position, &mut Physics), Changed<Position>>) {
    for (position, mut physics) in query.iter_mut() {
        let bounding_box = physics.dimensions.make_bounding_box(position);
        physics.bounding_box = bounding_box;
    }
}
