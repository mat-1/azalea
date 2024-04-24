// This file was generated by codegen/lib/code/tags.py, don't edit it manually!

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::Fluid;

pub static LAVA: Lazy<HashSet<Fluid>> =
    Lazy::new(|| HashSet::from_iter(vec![Fluid::Lava, Fluid::FlowingLava]));
pub static WATER: Lazy<HashSet<Fluid>> =
    Lazy::new(|| HashSet::from_iter(vec![Fluid::Water, Fluid::FlowingWater]));
