use azalea_core::position::BlockPos;

#[derive(Debug, Clone)]
pub struct BlockBox {
    min: BlockPos,
    max: BlockPos,
}

impl BlockBox {
    /// Create a new box from two corners.
    pub fn new(corner1: BlockPos, corner2: BlockPos) -> Self {
        Self {
            min: BlockPos::min(&corner1, &corner2),
            max: BlockPos::max(&corner1, &corner2),
        }
    }

    /// The lower corner of the box.
    pub fn min(&self) -> BlockPos {
        self.min
    }

    /// The upper corner of the box.
    pub fn max(&self) -> BlockPos {
        self.max
    }

    pub fn contains(&self, pos: BlockPos) -> bool {
        pos.x >= self.min.x
            && pos.x <= self.max.x
            && pos.y >= self.min.y
            && pos.y <= self.max.y
            && pos.z >= self.min.z
            && pos.z <= self.max.z
    }

    pub fn distances_to(&self, pos: BlockPos) -> (i32, i32, i32) {
        let dx = if pos.x < self.min.x {
            self.min.x - pos.x
        } else if pos.x > self.max.x {
            pos.x - self.max.x
        } else {
            0
        };
        let dy = if pos.y < self.min.y {
            self.min.y - pos.y
        } else if pos.y > self.max.y {
            pos.y - self.max.y
        } else {
            0
        };
        let dz = if pos.z < self.min.z {
            self.min.z - pos.z
        } else if pos.z > self.max.z {
            pos.z - self.max.z
        } else {
            0
        };

        (dx, dy, dz)
    }

    pub fn distance_squared_to(&self, pos: BlockPos) -> u32 {
        if self.contains(pos) {
            return 0;
        }

        let (dx, dy, dz) = self.distances_to(pos);
        (dx * dx + dy * dy + dz * dz) as u32
    }

    /// Get the block position inside of the box that is closest to the given
    /// position.
    pub fn closest_block_pos(&self, pos: BlockPos) -> BlockPos {
        let (dx, dy, dz) = self.distances_to(pos);
        BlockPos::new(pos.x + dx, pos.y + dy, pos.z + dz)
    }
}
