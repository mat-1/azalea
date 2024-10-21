use super::mergers::IndexMerger;
use crate::collision::{BitSetDiscreteVoxelShape, DiscreteVoxelShape, AABB};
use azalea_core::{
    block_hit_result::BlockHitResult,
    direction::{Axis, AxisCycle, Direction},
    math::{binary_search, EPSILON},
    position::{BlockPos, Vec3},
};
use std::{cmp, num::NonZeroU32, sync::LazyLock};

pub struct Shapes;

pub static BLOCK_SHAPE: LazyLock<VoxelShape> = LazyLock::new(|| {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(shape)))
});

pub fn box_shape(
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
) -> VoxelShape {
    assert!(min_x >= 0., "min_x must be >= 0 but was {min_x}");
    assert!(min_y >= 0.);
    assert!(min_z >= 0.);
    assert!(max_x >= 0.);
    assert!(max_y >= 0.);
    assert!(max_z >= 0.);

    box_shape_unchecked(min_x, min_y, min_z, max_x, max_y, max_z)
}

pub fn box_shape_unchecked(
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
) -> VoxelShape {
    if max_x - min_x < EPSILON && max_y - min_y < EPSILON && max_z - min_z < EPSILON {
        return EMPTY_SHAPE.clone();
    }

    let x_bits = find_bits(min_x, max_x);
    let y_bits = find_bits(min_y, max_y);
    let z_bits = find_bits(min_z, max_z);

    if x_bits < 0 || y_bits < 0 || z_bits < 0 {
        return VoxelShape::Array(ArrayVoxelShape::new(
            BLOCK_SHAPE.shape().to_owned(),
            vec![min_x, max_x],
            vec![min_y, max_y],
            vec![min_z, max_z],
        ));
    }
    if x_bits == 0 && y_bits == 0 && z_bits == 0 {
        return BLOCK_SHAPE.clone();
    }

    let x_bits = 1 << x_bits;
    let y_bits = 1 << y_bits;
    let z_bits = 1 << z_bits;
    let shape = BitSetDiscreteVoxelShape::with_filled_bounds(
        x_bits,
        y_bits,
        z_bits,
        (min_x * x_bits as f64).round() as i32,
        (min_y * y_bits as f64).round() as i32,
        (min_z * z_bits as f64).round() as i32,
        (max_x * x_bits as f64).round() as i32,
        (max_y * y_bits as f64).round() as i32,
        (max_z * z_bits as f64).round() as i32,
    );
    VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(shape)))
}

pub static EMPTY_SHAPE: LazyLock<VoxelShape> = LazyLock::new(|| {
    VoxelShape::Array(ArrayVoxelShape::new(
        DiscreteVoxelShape::BitSet(BitSetDiscreteVoxelShape::new(0, 0, 0)),
        vec![0.],
        vec![0.],
        vec![0.],
    ))
});

fn find_bits(min: f64, max: f64) -> i32 {
    if min < -EPSILON || max > 1. + EPSILON {
        return -1;
    }
    for bits in 0..=3 {
        let shifted_bits = 1 << bits;
        let min = min * shifted_bits as f64;
        let max = max * shifted_bits as f64;
        let min_ok = (min - min.round()).abs() < EPSILON * shifted_bits as f64;
        let max_ok = (max - max.round()).abs() < EPSILON * shifted_bits as f64;
        if min_ok && max_ok {
            return bits;
        }
    }
    -1
}

impl Shapes {
    pub fn or(a: VoxelShape, b: VoxelShape) -> VoxelShape {
        Self::join(a, b, |a, b| a || b)
    }

    pub fn collide(
        axis: &Axis,
        entity_box: &AABB,
        collision_boxes: &Vec<VoxelShape>,
        mut movement: f64,
    ) -> f64 {
        for shape in collision_boxes {
            if movement.abs() < EPSILON {
                return 0.;
            }
            movement = shape.collide(axis, entity_box, movement);
        }
        movement
    }

    pub fn join(a: VoxelShape, b: VoxelShape, op: fn(bool, bool) -> bool) -> VoxelShape {
        Self::join_unoptimized(a, b, op).optimize()
    }

    pub fn join_unoptimized(
        a: VoxelShape,
        b: VoxelShape,
        op: fn(bool, bool) -> bool,
    ) -> VoxelShape {
        if op(false, false) {
            panic!("Illegal operation");
        };
        // if (a == b) {
        //     return if op(true, true) { a } else { empty_shape() };
        // }
        let op_true_false = op(true, false);
        let op_false_true = op(false, true);
        if a.is_empty() {
            return if op_false_true {
                b
            } else {
                EMPTY_SHAPE.clone()
            };
        }
        if b.is_empty() {
            return if op_true_false {
                a
            } else {
                EMPTY_SHAPE.clone()
            };
        }
        // IndexMerger var5 = createIndexMerger(1, a.getCoords(Direction.Axis.X),
        // b.getCoords(Direction.Axis.X), var3, var4); IndexMerger var6 =
        // createIndexMerger(var5.size() - 1, a.getCoords(Direction.Axis.Y),
        // b.getCoords(Direction.Axis.Y), var3, var4); IndexMerger var7 =
        // createIndexMerger((var5.size() - 1) * (var6.size() - 1),
        // a.getCoords(Direction.Axis.Z), b.getCoords(Direction.Axis.Z), var3, var4);
        // BitSetDiscreteVoxelShape var8 = BitSetDiscreteVoxelShape.join(a.shape,
        // b.shape, var5, var6, var7, op); return (VoxelShape)(var5 instanceof
        // DiscreteCubeMerger && var6 instanceof DiscreteCubeMerger && var7 instanceof
        // DiscreteCubeMerger ? new CubeVoxelShape(var8) : new ArrayVoxelShape(var8,
        // var5.getList(), var6.getList(), var7.getList()));
        let var5 = Self::create_index_merger(
            1,
            a.get_coords(Axis::X),
            b.get_coords(Axis::X),
            op_true_false,
            op_false_true,
        );
        let var6 = Self::create_index_merger(
            (var5.size() - 1).try_into().unwrap(),
            a.get_coords(Axis::Y),
            b.get_coords(Axis::Y),
            op_true_false,
            op_false_true,
        );
        let var7 = Self::create_index_merger(
            ((var5.size() - 1) * (var6.size() - 1)).try_into().unwrap(),
            a.get_coords(Axis::Z),
            b.get_coords(Axis::Z),
            op_true_false,
            op_false_true,
        );
        let var8 = BitSetDiscreteVoxelShape::join(a.shape(), b.shape(), &var5, &var6, &var7, op);
        // if var5.is_discrete_cube_merger()

        if matches!(var5, IndexMerger::DiscreteCube { .. })
            && matches!(var6, IndexMerger::DiscreteCube { .. })
            && matches!(var7, IndexMerger::DiscreteCube { .. })
        {
            VoxelShape::Cube(CubeVoxelShape::new(DiscreteVoxelShape::BitSet(var8)))
        } else {
            VoxelShape::Array(ArrayVoxelShape::new(
                DiscreteVoxelShape::BitSet(var8),
                var5.get_list(),
                var6.get_list(),
                var7.get_list(),
            ))
        }
    }

    /// Check if the op is true anywhere when joining the two shapes
    /// vanilla calls this joinIsNotEmpty
    pub fn matches_anywhere(
        a: &VoxelShape,
        b: &VoxelShape,
        op: impl Fn(bool, bool) -> bool,
    ) -> bool {
        debug_assert!(!op(false, false));
        let a_is_empty = a.is_empty();
        let b_is_empty = b.is_empty();
        if a_is_empty || b_is_empty {
            return op(!a_is_empty, !b_is_empty);
        }
        if a == b {
            return op(true, true);
        }

        let op_true_false = op(true, false);
        let op_false_true = op(false, true);

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if a.max(axis) < b.min(axis) - EPSILON {
                return op_true_false || op_false_true;
            }
            if b.max(axis) < a.min(axis) - EPSILON {
                return op_true_false || op_false_true;
            }
        }

        let x_merger = Self::create_index_merger(
            1,
            a.get_coords(Axis::X),
            b.get_coords(Axis::X),
            op_true_false,
            op_false_true,
        );
        let y_merger = Self::create_index_merger(
            (x_merger.size() - 1) as i32,
            a.get_coords(Axis::Y),
            b.get_coords(Axis::Y),
            op_true_false,
            op_false_true,
        );
        let z_merger = Self::create_index_merger(
            ((x_merger.size() - 1) * (y_merger.size() - 1)) as i32,
            a.get_coords(Axis::Z),
            b.get_coords(Axis::Z),
            op_true_false,
            op_false_true,
        );

        Self::matches_anywhere_with_mergers(
            x_merger,
            y_merger,
            z_merger,
            a.shape().to_owned(),
            b.shape().to_owned(),
            op,
        )
    }

    pub fn matches_anywhere_with_mergers(
        merged_x: IndexMerger,
        merged_y: IndexMerger,
        merged_z: IndexMerger,
        shape1: DiscreteVoxelShape,
        shape2: DiscreteVoxelShape,
        op: impl Fn(bool, bool) -> bool,
    ) -> bool {
        !merged_x.for_merged_indexes(|var5x, var6, _var7| {
            merged_y.for_merged_indexes(|var6x, var7x, _var8| {
                merged_z.for_merged_indexes(|var7, var8x, _var9| {
                    !op(
                        shape1.is_full_wide(var5x, var6x, var7),
                        shape2.is_full_wide(var6, var7x, var8x),
                    )
                })
            })
        })
    }

    pub fn create_index_merger(
        _var0: i32,
        coords1: &[f64],
        coords2: &[f64],
        var3: bool,
        var4: bool,
    ) -> IndexMerger {
        let var5 = coords1.len() - 1;
        let var6 = coords2.len() - 1;
        // if (&var1 as &dyn Any).is::<CubePointRange>() && (&var2 as &dyn
        // Any).is::<CubePointRange>() {
        // return new DiscreteCubeMerger(var0, var5, var6, var3, var4);
        // let var7: i64 = lcm(var5 as u32, var6 as u32).try_into().unwrap();
        // //    if ((long)var0 * var7 <= 256L) {
        // if var0 as i64 * var7 <= 256 {
        //     return IndexMerger::new_discrete_cube(var5 as u32, var6 as u32);
        // }
        // }

        if coords1[var5] < coords2[0] - EPSILON {
            IndexMerger::NonOverlapping {
                lower: coords1.to_vec(),
                upper: coords2.to_vec(),
                swap: false,
            }
        } else if coords2[var6] < coords1[0] - EPSILON {
            IndexMerger::NonOverlapping {
                lower: coords2.to_vec(),
                upper: coords1.to_vec(),
                swap: true,
            }
        } else if var5 == var6 && coords1 == coords2 {
            IndexMerger::Identical {
                coords: coords1.to_vec(),
            }
        } else {
            IndexMerger::new_indirect(coords1, coords2, var3, var4)
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum VoxelShape {
    Array(ArrayVoxelShape),
    Cube(CubeVoxelShape),
}

impl VoxelShape {
    // public double min(Direction.Axis var1) {
    //     int var2 = this.shape.firstFull(var1);
    //     return var2 >= this.shape.getSize(var1) ? 1.0D / 0.0 : this.get(var1,
    // var2); }
    // public double max(Direction.Axis var1) {
    //     int var2 = this.shape.lastFull(var1);
    //     return var2 <= 0 ? -1.0D / 0.0 : this.get(var1, var2);
    // }
    fn min(&self, axis: Axis) -> f64 {
        let first_full = self.shape().first_full(axis);
        if first_full >= self.shape().size(axis) as i32 {
            f64::INFINITY
        } else {
            self.get(axis, first_full.try_into().unwrap())
        }
    }
    fn max(&self, axis: Axis) -> f64 {
        let last_full = self.shape().last_full(axis);
        if last_full <= 0 {
            f64::NEG_INFINITY
        } else {
            self.get(axis, last_full.try_into().unwrap())
        }
    }

    pub fn shape(&self) -> &DiscreteVoxelShape {
        match self {
            VoxelShape::Array(s) => s.shape(),
            VoxelShape::Cube(s) => s.shape(),
        }
    }

    pub fn get_coords(&self, axis: Axis) -> &[f64] {
        match self {
            VoxelShape::Array(s) => s.get_coords(axis),
            VoxelShape::Cube(s) => s.get_coords(axis),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shape().is_empty()
    }

    #[must_use]
    pub fn move_relative(&self, x: f64, y: f64, z: f64) -> VoxelShape {
        if self.shape().is_empty() {
            return EMPTY_SHAPE.clone();
        }

        VoxelShape::Array(ArrayVoxelShape::new(
            self.shape().to_owned(),
            self.get_coords(Axis::X).iter().map(|c| c + x).collect(),
            self.get_coords(Axis::Y).iter().map(|c| c + y).collect(),
            self.get_coords(Axis::Z).iter().map(|c| c + z).collect(),
        ))
    }

    #[inline]
    pub fn get(&self, axis: Axis, index: usize) -> f64 {
        // self.get_coords(axis)[index]
        match self {
            VoxelShape::Array(s) => s.get_coords(axis)[index],
            VoxelShape::Cube(s) => s.get_coords(axis)[index],
            // _ => self.get_coords(axis)[index],
        }
    }

    pub fn find_index(&self, axis: Axis, coord: f64) -> i32 {
        // let r = binary_search(0, (self.shape().size(axis) + 1) as i32, &|t| {
        //     coord < self.get(axis, t as usize)
        // }) - 1;
        // r
        match self {
            VoxelShape::Cube(s) => s.find_index(axis, coord),
            _ => {
                let upper_limit = (self.shape().size(axis) + 1) as i32;
                binary_search(0, upper_limit, &|t| coord < self.get(axis, t as usize)) - 1
            }
        }
    }

    pub fn clip(&self, from: &Vec3, to: &Vec3, block_pos: &BlockPos) -> Option<BlockHitResult> {
        if self.is_empty() {
            return None;
        }
        let vector = to - from;
        if vector.length_squared() < EPSILON {
            return None;
        }
        let right_after_start = from + &(vector * 0.001);

        if self.shape().is_full_wide(
            self.find_index(Axis::X, right_after_start.x - (block_pos.x as f64)),
            self.find_index(Axis::Y, right_after_start.y - (block_pos.y as f64)),
            self.find_index(Axis::Z, right_after_start.z - (block_pos.z as f64)),
        ) {
            Some(BlockHitResult {
                block_pos: *block_pos,
                direction: Direction::nearest(vector).opposite(),
                location: right_after_start,
                inside: true,
                miss: false,
            })
        } else {
            AABB::clip_iterable(&self.to_aabbs(), from, to, block_pos)
        }
    }

    pub fn collide(&self, axis: &Axis, entity_box: &AABB, movement: f64) -> f64 {
        self.collide_x(AxisCycle::between(*axis, Axis::X), entity_box, movement)
    }
    pub fn collide_x(&self, axis_cycle: AxisCycle, entity_box: &AABB, mut movement: f64) -> f64 {
        if self.shape().is_empty() {
            return movement;
        }
        if movement.abs() < EPSILON {
            return 0.;
        }

        let inverse_axis_cycle = axis_cycle.inverse();

        let x_axis = inverse_axis_cycle.cycle(Axis::X);
        let y_axis = inverse_axis_cycle.cycle(Axis::Y);
        let z_axis = inverse_axis_cycle.cycle(Axis::Z);

        let max_x = entity_box.max(&x_axis);
        let min_x = entity_box.min(&x_axis);

        let x_min_index = self.find_index(x_axis, min_x + EPSILON);
        let x_max_index = self.find_index(x_axis, max_x - EPSILON);

        let y_min_index = cmp::max(
            0,
            self.find_index(y_axis, entity_box.min(&y_axis) + EPSILON),
        );
        let y_max_index = cmp::min(
            self.shape().size(y_axis) as i32,
            self.find_index(y_axis, entity_box.max(&y_axis) - EPSILON) + 1,
        );

        let z_min_index = cmp::max(
            0,
            self.find_index(z_axis, entity_box.min(&z_axis) + EPSILON),
        );
        let z_max_index = cmp::min(
            self.shape().size(z_axis) as i32,
            self.find_index(z_axis, entity_box.max(&z_axis) - EPSILON) + 1,
        );

        if movement > 0. {
            for x in x_max_index + 1..(self.shape().size(x_axis) as i32) {
                for y in y_min_index..y_max_index {
                    for z in z_min_index..z_max_index {
                        if self
                            .shape()
                            .is_full_wide_axis_cycle(inverse_axis_cycle, x, y, z)
                        {
                            let var23 = self.get(x_axis, x as usize) - max_x;
                            if var23 >= -EPSILON {
                                movement = f64::min(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        } else if movement < 0. && x_min_index > 0 {
            for x in (0..x_min_index).rev() {
                for y in y_min_index..y_max_index {
                    for z in z_min_index..z_max_index {
                        if self
                            .shape()
                            .is_full_wide_axis_cycle(inverse_axis_cycle, x, y, z)
                        {
                            let var23 = self.get(x_axis, (x + 1) as usize) - min_x;
                            if var23 <= EPSILON {
                                movement = f64::max(movement, var23);
                            }
                            return movement;
                        }
                    }
                }
            }
        }

        movement
    }

    // public VoxelShape optimize() {
    //     VoxelShape[] var1 = new VoxelShape[]{Shapes.empty()};
    //     this.forAllBoxes((var1x, var3, var5, var7, var9, var11) -> {
    //         var1[0] = Shapes.joinUnoptimized(var1[0], Shapes.box(var1x, var3,
    // var5, var7, var9, var11), BooleanOp.OR);     });
    //     return var1[0];
    // }
    fn optimize(&self) -> VoxelShape {
        let mut shape = EMPTY_SHAPE.clone();
        self.for_all_boxes(|var1x, var3, var5, var7, var9, var11| {
            shape = Shapes::join_unoptimized(
                shape.clone(),
                box_shape(var1x, var3, var5, var7, var9, var11),
                |a, b| a || b,
            );
        });
        shape
    }

    // public void forAllBoxes(Shapes.DoubleLineConsumer var1) {
    //     DoubleList var2 = this.getCoords(Direction.Axis.X);
    //     DoubleList var3 = this.getCoords(Direction.Axis.Y);
    //     DoubleList var4 = this.getCoords(Direction.Axis.Z);
    //     this.shape.forAllBoxes((var4x, var5, var6, var7, var8, var9) -> {
    //     var1.consume(var2.getDouble(var4x), var3.getDouble(var5),
    // var4.getDouble(var6), var2.getDouble(var7), var3.getDouble(var8),
    // var4.getDouble(var9));     }, true);
    // }
    pub fn for_all_boxes(&self, mut consumer: impl FnMut(f64, f64, f64, f64, f64, f64))
    where
        Self: Sized,
    {
        // let x_coords = self.get_coords(Axis::X);
        // let y_coords = self.get_coords(Axis::Y);
        // let z_coords = self.get_coords(Axis::Z);
        // self.shape().for_all_boxes(
        //     |var4x, var5, var6, var7, var8, var9| {
        //         consumer(
        //             x_coords[var4x as usize],
        //             y_coords[var5 as usize],
        //             z_coords[var6 as usize],
        //             x_coords[var7 as usize],
        //             y_coords[var8 as usize],
        //             z_coords[var9 as usize],
        //         )
        //     },
        //     true,
        // );
        let x_coords = self.get_coords(Axis::X);
        let y_coords = self.get_coords(Axis::Y);
        let z_coords = self.get_coords(Axis::Z);
        self.shape().for_all_boxes(
            |min_x, min_y, min_z, max_x, max_y, max_z| {
                consumer(
                    x_coords[min_x as usize],
                    y_coords[min_y as usize],
                    z_coords[min_z as usize],
                    x_coords[max_x as usize],
                    y_coords[max_y as usize],
                    z_coords[max_z as usize],
                );
            },
            true,
        );
    }

    pub fn to_aabbs(&self) -> Vec<AABB> {
        let mut aabbs = Vec::new();
        self.for_all_boxes(|min_x, min_y, min_z, max_x, max_y, max_z| {
            aabbs.push(AABB {
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
            });
        });
        aabbs
    }
}

impl From<AABB> for VoxelShape {
    fn from(aabb: AABB) -> Self {
        box_shape_unchecked(
            aabb.min_x, aabb.min_y, aabb.min_z, aabb.max_x, aabb.max_y, aabb.max_z,
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ArrayVoxelShape {
    shape: DiscreteVoxelShape,
    // TODO: check where faces is used in minecraft
    #[allow(dead_code)]
    faces: Option<Vec<VoxelShape>>,

    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub zs: Vec<f64>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CubeVoxelShape {
    shape: DiscreteVoxelShape,
    // TODO: check where faces is used in minecraft
    #[allow(dead_code)]
    faces: Option<Vec<VoxelShape>>,

    x_coords: Vec<f64>,
    y_coords: Vec<f64>,
    z_coords: Vec<f64>,
}

impl ArrayVoxelShape {
    pub fn new(shape: DiscreteVoxelShape, xs: Vec<f64>, ys: Vec<f64>, zs: Vec<f64>) -> Self {
        let x_size = shape.size(Axis::X) + 1;
        let y_size = shape.size(Axis::Y) + 1;
        let z_size = shape.size(Axis::Z) + 1;

        // Lengths of point arrays must be consistent with the size of the VoxelShape.
        debug_assert_eq!(x_size, xs.len() as u32);
        debug_assert_eq!(y_size, ys.len() as u32);
        debug_assert_eq!(z_size, zs.len() as u32);

        Self {
            faces: None,
            shape,
            xs,
            ys,
            zs,
        }
    }
}

impl ArrayVoxelShape {
    fn shape(&self) -> &DiscreteVoxelShape {
        &self.shape
    }

    #[inline]
    fn get_coords(&self, axis: Axis) -> &[f64] {
        axis.choose(&self.xs, &self.ys, &self.zs)
    }
}

impl CubeVoxelShape {
    pub fn new(shape: DiscreteVoxelShape) -> Self {
        // pre-calculate the coor
        let x_coords = Self::calculate_coords(&shape, Axis::X);
        let y_coords = Self::calculate_coords(&shape, Axis::Y);
        let z_coords = Self::calculate_coords(&shape, Axis::Z);

        Self {
            shape,
            faces: None,
            x_coords,
            y_coords,
            z_coords,
        }
    }
}

impl CubeVoxelShape {
    fn shape(&self) -> &DiscreteVoxelShape {
        &self.shape
    }

    fn calculate_coords(shape: &DiscreteVoxelShape, axis: Axis) -> Vec<f64> {
        let size = shape.size(axis);
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..=size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }

    #[inline]
    fn get_coords(&self, axis: Axis) -> &[f64] {
        axis.choose(&self.x_coords, &self.y_coords, &self.z_coords)
    }

    fn find_index(&self, axis: Axis, coord: f64) -> i32 {
        let n = self.shape().size(axis);
        (f64::clamp(coord * (n as f64), -1f64, n as f64)) as i32
    }
}

#[derive(Debug)]
pub struct CubePointRange {
    /// Needs at least 1 part
    pub parts: NonZeroU32,
}
impl CubePointRange {
    pub fn get_double(&self, index: u32) -> f64 {
        index as f64 / self.parts.get() as f64
    }

    pub fn size(&self) -> u32 {
        self.parts.get() + 1
    }

    pub fn iter(&self) -> Vec<f64> {
        (0..=self.parts.get()).map(|i| self.get_double(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_shape() {
        let shape = &*BLOCK_SHAPE;
        assert_eq!(shape.shape().size(Axis::X), 1);
        assert_eq!(shape.shape().size(Axis::Y), 1);
        assert_eq!(shape.shape().size(Axis::Z), 1);

        assert_eq!(shape.get_coords(Axis::X).len(), 2);
        assert_eq!(shape.get_coords(Axis::Y).len(), 2);
        assert_eq!(shape.get_coords(Axis::Z).len(), 2);
    }

    #[test]
    fn test_box_shape() {
        let shape = box_shape(0., 0., 0., 1., 1., 1.);
        assert_eq!(shape.shape().size(Axis::X), 1);
        assert_eq!(shape.shape().size(Axis::Y), 1);
        assert_eq!(shape.shape().size(Axis::Z), 1);

        assert_eq!(shape.get_coords(Axis::X).len(), 2);
        assert_eq!(shape.get_coords(Axis::Y).len(), 2);
        assert_eq!(shape.get_coords(Axis::Z).len(), 2);
    }

    #[test]
    fn test_top_slab_shape() {
        let shape = box_shape(0., 0.5, 0., 1., 1., 1.);
        assert_eq!(shape.shape().size(Axis::X), 1);
        assert_eq!(shape.shape().size(Axis::Y), 2);
        assert_eq!(shape.shape().size(Axis::Z), 1);

        assert_eq!(shape.get_coords(Axis::X).len(), 2);
        assert_eq!(shape.get_coords(Axis::Y).len(), 3);
        assert_eq!(shape.get_coords(Axis::Z).len(), 2);
    }

    #[test]
    fn test_join_is_not_empty() {
        let shape = box_shape(0., 0., 0., 1., 1., 1.);
        let shape2 = box_shape(0., 0.5, 0., 1., 1., 1.);
        // detect if the shapes intersect at all
        let joined = Shapes::matches_anywhere(&shape, &shape2, |a, b| a && b);
        assert!(joined, "Shapes should intersect");
    }
}
