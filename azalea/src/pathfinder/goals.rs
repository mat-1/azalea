//! The goals that a pathfinder can try to reach.

use std::f32::consts::SQRT_2;

use azalea_core::position::{BlockPos, Vec3};

use super::costs::{COST_HEURISTIC, FALL_N_BLOCKS_COST, JUMP_ONE_BLOCK_COST};

pub trait Goal: Send + Sync {
    #[must_use]
    fn heuristic(&self, n: BlockPos) -> f32;
    #[must_use]
    fn success(&self, n: BlockPos) -> bool;
}

/// Move to the given block position. This is the most commonly used goal.
#[derive(Debug)]
pub struct BlockPosGoal(pub BlockPos);
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.0.x - n.x) as f32;
        let dy = (self.0.y - n.y) as f32;
        let dz = (self.0.z - n.z) as f32;

        xz_heuristic(dx, dz) + y_heuristic(dy)
    }
    fn success(&self, n: BlockPos) -> bool {
        n == self.0
    }
}

pub fn xz_heuristic(dx: f32, dz: f32) -> f32 {
    let x = dx.abs();
    let z = dz.abs();

    let diagonal;
    let straight;

    if x < z {
        straight = z - x;
        diagonal = x;
    } else {
        straight = x - z;
        diagonal = z;
    }

    (diagonal * SQRT_2 + straight) * COST_HEURISTIC
}

/// Move to the given block position, ignoring the y axis.
#[derive(Debug)]
pub struct XZGoal {
    pub x: i32,
    pub z: i32,
}
impl Goal for XZGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.x - n.x) as f32;
        let dz = (self.z - n.z) as f32;
        xz_heuristic(dx, dz)
    }
    fn success(&self, n: BlockPos) -> bool {
        n.x == self.x && n.z == self.z
    }
}

pub fn y_heuristic(dy: f32) -> f32 {
    if dy > 0.0 {
        *JUMP_ONE_BLOCK_COST * dy
    } else {
        FALL_N_BLOCKS_COST[2] / 2. * -dy
    }
}

/// Move to the given y coordinate.
#[derive(Debug)]
pub struct YGoal {
    pub y: i32,
}
impl Goal for YGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dy = (self.y - n.y) as f32;
        y_heuristic(dy)
    }
    fn success(&self, n: BlockPos) -> bool {
        n.y == self.y
    }
}

/// Get within the given radius of the given position.
#[derive(Debug)]
pub struct RadiusGoal {
    pub pos: Vec3,
    pub radius: f32,
}
impl Goal for RadiusGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;

        xz_heuristic(dx, dz) + y_heuristic(dy)
    }
    fn success(&self, n: BlockPos) -> bool {
        let n = n.center();
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;

        dx * dx + dy * dy + dz * dz <= self.radius * self.radius
    }
}

/// Do the opposite of the given goal.
#[derive(Debug)]
pub struct InverseGoal<T: Goal>(pub T);
impl<T: Goal> Goal for InverseGoal<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        -self.0.heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        !self.0.success(n)
    }
}

/// Do either of the given goals, whichever is closer.
#[derive(Debug)]
pub struct OrGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for OrGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).min(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) || self.1.success(n)
    }
}

/// Do any of the given goals, whichever is closest.
#[derive(Debug)]
pub struct OrGoals<T: Goal>(pub Vec<T>);
impl<T: Goal> Goal for OrGoals<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0
            .iter()
            .map(|goal| goal.heuristic(n))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f32::INFINITY)
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.iter().any(|goal| goal.success(n))
    }
}

/// Try to reach both of the given goals.
#[derive(Debug)]
pub struct AndGoal<T: Goal, U: Goal>(pub T, pub U);
impl<T: Goal, U: Goal> Goal for AndGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n).max(self.1.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n) && self.1.success(n)
    }
}

/// Try to reach all of the given goals.
#[derive(Debug)]
pub struct AndGoals<T: Goal>(pub Vec<T>);
impl<T: Goal> Goal for AndGoals<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0
            .iter()
            .map(|goal| goal.heuristic(n))
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f32::INFINITY)
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.iter().all(|goal| goal.success(n))
    }
}

/// Multiply the heuristic of the given goal by the given factor.
///
/// Setting the value to less than 1 makes it be biased towards the goal, and
/// setting it to more than 1 makes it be biased away from the goal. For
/// example, setting the value to 0.5 makes the pathfinder think that the
/// goal is half the distance that it actually is.
///
/// Note that this may reduce the quality of paths or make the pathfinder slower
/// if used incorrectly.
///
/// This goal is most useful when combined with [`OrGoal`].
#[derive(Debug)]
pub struct ScaleGoal<T: Goal>(pub T, pub f32);
impl<T: Goal> Goal for ScaleGoal<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.0.heuristic(n) * self.1
    }
    fn success(&self, n: BlockPos) -> bool {
        self.0.success(n)
    }
}
