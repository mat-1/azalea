//! Slightly more unusual goals than the normal
//! [pathfinder ones](crate::pathfinder::goals).

use azalea_core::position::BlockPos;
use azalea_world::ChunkStorage;

use crate::pathfinder::{
    block_box::BlockBox,
    goals::{xz_heuristic, y_heuristic, BlockPosGoal, Goal},
};

use super::utils::get_hit_result_while_looking_at;

/// Move to a position where we can reach the given block.
#[derive(Debug)]
pub struct ReachBlockPosGoal {
    pub pos: BlockPos,
    pub chunk_storage: ChunkStorage,
}
impl Goal for ReachBlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        BlockPosGoal(self.pos).heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        // only do the expensive check if we're close enough
        let max_pick_range = 6;

        let distance = (self.pos - n).length_squared();
        if distance > max_pick_range * max_pick_range {
            return false;
        }

        let block_hit_result = get_hit_result_while_looking_at(&self.chunk_storage, n, self.pos);

        block_hit_result == self.pos
    }
}

/// Move to a position inside of the given box (inclusive, so the corners are
/// included in the box).
#[derive(Debug)]
pub struct BoxGoal(pub BlockBox);

impl Goal for BoxGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = if n.x < self.0.min().x {
            self.0.min().x - n.x
        } else if n.x > self.0.max().x {
            n.x - self.0.max().x
        } else {
            0
        };
        let dy = if n.y < self.0.min().y {
            self.0.min().y - n.y
        } else if n.y > self.0.max().y {
            n.y - self.0.max().y
        } else {
            0
        };
        let dz = if n.z < self.0.min().z {
            self.0.min().z - n.z
        } else if n.z > self.0.max().z {
            n.z - self.0.max().z
        } else {
            0
        };

        xz_heuristic(dx as f32, dz as f32) + y_heuristic(dy as f32)
    }

    fn success(&self, n: BlockPos) -> bool {
        n.x >= self.0.min().x
            && n.x <= self.0.max().x
            && n.y >= self.0.min().y
            && n.y <= self.0.max().y
            && n.z >= self.0.min().z
            && n.z <= self.0.max().z
    }
}

/// Move to a position where we can reach at least one block from the given box.
/// This is usually used when digging out an area.
#[derive(Debug)]
pub struct ReachBoxGoal {
    pub bb: BlockBox,
    pub chunk_storage: ChunkStorage,
}
impl Goal for ReachBoxGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        BoxGoal(self.bb.clone()).heuristic(n)
    }

    fn success(&self, n: BlockPos) -> bool {
        // succeed if we're already in the box
        if self.bb.contains(n) {
            return true;
        }

        // only do the expensive check if we're close enough
        let max_pick_range = 6;

        let distance = self.bb.distance_squared_to(n);
        if distance > max_pick_range * max_pick_range {
            return false;
        }

        // look at the closest block
        let look_target = self.bb.closest_block_pos(n);
        let hit_result = get_hit_result_while_looking_at(&self.chunk_storage, n, look_target);

        self.bb.contains(hit_result)
    }
}
