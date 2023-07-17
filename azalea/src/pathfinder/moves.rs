use azalea_core::{BlockPos, CardinalDirection};
use azalea_physics::collision::{self, BlockWithShape};
use azalea_world::Instance;

use super::astar::{self, Movement};

/// whether this block is passable
fn is_block_passable(pos: &BlockPos, world: &Instance) -> bool {
    if let Some(block) = world.chunks.get_block_state(pos) {
        block.shape() == &collision::empty_shape()
    } else {
        false
    }
}

/// whether this block has a solid hitbox (i.e. we can stand on it)
fn is_block_solid(pos: &BlockPos, world: &Instance) -> bool {
    if let Some(block) = world.chunks.get_block_state(pos) {
        block.shape() == &collision::block_shape()
    } else {
        false
    }
}

/// Whether this block and the block above are passable
fn is_passable(pos: &BlockPos, world: &Instance) -> bool {
    is_block_passable(pos, world) && is_block_passable(&pos.up(1), world)
}

/// Whether we can stand in this position. Checks if the block below is solid,
/// and that the two blocks above that are passable.

fn is_standable(pos: &BlockPos, world: &Instance) -> bool {
    is_block_solid(&pos.down(1), world) && is_passable(pos, world)
}

/// Get the amount of air blocks until the next solid block below this one.
fn fall_distance(pos: &BlockPos, world: &Instance) -> u32 {
    let mut distance = 0;
    let mut current_pos = pos.down(1);
    while is_block_passable(&current_pos, world) {
        distance += 1;
        current_pos = current_pos.down(1);

        if current_pos.y < world.chunks.min_y {
            return u32::MAX;
        }
    }
    distance
}

const JUMP_COST: f32 = 0.5;
const WALK_ONE_BLOCK_COST: f32 = 1.0;
const FALL_ONE_BLOCK_COST: f32 = 0.5;

type Edge = astar::Edge<BlockPos, MoveData, f32>;

pub trait Move: Send + Sync {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge>;
}

#[derive(Debug, Clone)]
pub struct MoveData {
    pub jump: bool,
}

pub struct ForwardMove(pub CardinalDirection);
impl Move for ForwardMove {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        let offset = BlockPos::new(self.0.x(), 0, self.0.z());

        if !is_standable(&(node + offset), world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST;

        Some(Edge {
            movement: Movement {
                target: node + offset,
                data: MoveData { jump: false },
            },
            cost,
        })
    }
}

pub struct AscendMove(pub CardinalDirection);
impl Move for AscendMove {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        let offset = BlockPos::new(self.0.x(), 1, self.0.z());

        if !is_block_passable(&node.up(2), world) || !is_standable(&(node + offset), world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST + JUMP_COST;

        Some(Edge {
            movement: Movement {
                target: node + offset,
                data: MoveData { jump: true },
            },
            cost,
        })
    }
}
pub struct DescendMove(pub CardinalDirection);
impl Move for DescendMove {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        let new_horizontal_position = node + BlockPos::new(self.0.x(), 0, self.0.z());
        let fall_distance = fall_distance(&new_horizontal_position, world);
        if fall_distance == 0 {
            return None;
        }
        if fall_distance > 3 {
            return None;
        }
        let new_position = new_horizontal_position.down(fall_distance as i32);

        // check whether 3 blocks vertically forward are passable
        if !is_passable(&new_horizontal_position, world) {
            return None;
        }

        let cost = WALK_ONE_BLOCK_COST + FALL_ONE_BLOCK_COST * fall_distance as f32;

        Some(Edge {
            movement: Movement {
                target: new_position,
                data: MoveData { jump: false },
            },
            cost,
        })
    }
}
pub struct DiagonalMove(pub CardinalDirection);
impl Move for DiagonalMove {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        let right = self.0.right();
        let offset = BlockPos::new(self.0.x() + right.x(), 0, self.0.z() + right.z());

        if !is_passable(&(node + BlockPos::new(self.0.x(), 0, self.0.z())), world)
            && !is_passable(
                &BlockPos::new(
                    node.x + self.0.right().x(),
                    node.y,
                    node.z + self.0.right().z(),
                ),
                world,
            )
        {
            return None;
        }
        if !is_standable(&(node + offset), world) {
            return None;
        }
        let cost = WALK_ONE_BLOCK_COST * 1.4;

        Some(Edge {
            movement: Movement {
                target: node + offset,
                data: MoveData { jump: false },
            },
            cost,
        })
    }
}

pub struct ParkourForwardMove(pub CardinalDirection);
impl Move for ParkourForwardMove {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        // there's a space to pass through
        if !is_passable(&(node + BlockPos::new(self.0.x(), 0, self.0.z())), world) {
            return None;
        }
        // check to make sure we actually do have to jump
        if is_block_solid(&(node + BlockPos::new(self.0.x(), -1, self.0.z())), world) {
            return None;
        }
        // check to make sure we can land
        let offset = BlockPos::new(self.0.x() * 2, 0, self.0.z() * 2);
        if !is_standable(&(node + offset), world) {
            return None;
        }

        let cost = JUMP_COST + WALK_ONE_BLOCK_COST + WALK_ONE_BLOCK_COST + FALL_ONE_BLOCK_COST;

        Some(Edge {
            movement: Movement {
                target: node + offset,
                data: MoveData { jump: true },
            },
            cost,
        })
    }
}
pub struct ParkourForward2Move(pub CardinalDirection);
impl Move for ParkourForward2Move {
    fn get(&self, world: &Instance, node: BlockPos) -> Option<Edge> {
        // there's a space to pass through
        if !is_passable(&(node + BlockPos::new(self.0.x(), 0, self.0.z())), world) {
            return None;
        }
        if !is_passable(
            &(node + BlockPos::new(self.0.x() * 2, 0, self.0.z() * 2)),
            world,
        ) {
            return None;
        }
        // check to make sure we actually do have to jump
        if is_block_solid(&(node + BlockPos::new(self.0.x(), -1, self.0.z())), world) {
            return None;
        }
        if is_block_solid(
            &(node + BlockPos::new(self.0.x() * 2, -1, self.0.z() * 2)),
            world,
        ) {
            return None;
        }
        // check to make sure we can land
        let offset = BlockPos::new(self.0.x() * 3, 0, self.0.z() * 3);
        if !is_standable(&(node + offset), world) {
            return None;
        }

        let cost = JUMP_COST
            + WALK_ONE_BLOCK_COST
            + WALK_ONE_BLOCK_COST
            + WALK_ONE_BLOCK_COST
            + FALL_ONE_BLOCK_COST;

        Some(Edge {
            movement: Movement {
                target: node + offset,
                data: MoveData { jump: true },
            },
            cost,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azalea_block::BlockState;
    use azalea_core::ChunkPos;
    use azalea_world::{Chunk, ChunkStorage, PartialInstance};

    #[test]
    fn test_is_passable() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert_eq!(is_block_passable(&BlockPos::new(0, 0, 0), &world), false);
        assert_eq!(is_block_passable(&BlockPos::new(0, 1, 0), &world), true);
    }

    #[test]
    fn test_is_solid() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();
        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert_eq!(is_block_solid(&BlockPos::new(0, 0, 0), &world), true);
        assert_eq!(is_block_solid(&BlockPos::new(0, 1, 0), &world), false);
    }

    #[test]
    fn test_is_standable() {
        let mut partial_world = PartialInstance::default();
        let mut chunk_storage = ChunkStorage::default();
        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 0, 0),
            azalea_registry::Block::Stone.into(),
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 1, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 2, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );
        partial_world.chunks.set_block_state(
            &BlockPos::new(0, 3, 0),
            BlockState::AIR,
            &mut chunk_storage,
        );

        let world = chunk_storage.into();
        assert!(is_standable(&BlockPos::new(0, 1, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 0, 0), &world));
        assert!(!is_standable(&BlockPos::new(0, 2, 0), &world));
    }
}
