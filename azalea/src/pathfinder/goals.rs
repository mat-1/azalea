use azalea_core::BlockPos;

use super::Goal;

pub struct BlockPosGoal {
    pub pos: BlockPos,
}
impl Goal for BlockPosGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: BlockPos) -> bool {
        n == self.pos
    }
}
impl From<BlockPos> for BlockPosGoal {
    fn from(pos: BlockPos) -> Self {
        Self { pos }
    }
}

pub struct RadiusGoal {
    pub pos: BlockPos,
    pub radius: f32,
}
impl Goal for RadiusGoal {
    fn heuristic(&self, n: BlockPos) -> f32 {
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz
    }
    fn success(&self, n: BlockPos) -> bool {
        let dx = (self.pos.x - n.x) as f32;
        let dy = (self.pos.y - n.y) as f32;
        let dz = (self.pos.z - n.z) as f32;
        dx * dx + dy * dy + dz * dz <= self.radius * self.radius
    }
}

pub struct InverseGoal<T: Goal> {
    pub goal: T,
}
impl<T: Goal> Goal for InverseGoal<T> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        -self.goal.heuristic(n)
    }
    fn success(&self, n: BlockPos) -> bool {
        !self.goal.success(n)
    }
}

pub struct OrGoal<T: Goal, U: Goal> {
    pub goal1: T,
    pub goal2: U,
}
impl<T: Goal, U: Goal> Goal for OrGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.goal1.heuristic(n).min(self.goal2.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.goal1.success(n) || self.goal2.success(n)
    }
}

pub struct AndGoal<T: Goal, U: Goal> {
    pub goal1: T,
    pub goal2: U,
}
impl<T: Goal, U: Goal> Goal for AndGoal<T, U> {
    fn heuristic(&self, n: BlockPos) -> f32 {
        self.goal1.heuristic(n).max(self.goal2.heuristic(n))
    }
    fn success(&self, n: BlockPos) -> bool {
        self.goal1.success(n) && self.goal2.success(n)
    }
}
