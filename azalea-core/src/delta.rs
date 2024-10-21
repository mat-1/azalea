use crate::position::Vec3;
pub use azalea_buf::McBuf;

pub trait PositionDeltaTrait {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

/// Only works for up to 8 blocks
#[derive(Clone, Debug, McBuf, Default)]
pub struct PositionDelta8 {
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}

impl PositionDelta8 {
    #[deprecated]
    pub fn float(&self) -> (f64, f64, f64) {
        (
            (self.xa as f64) / 4096.0,
            (self.ya as f64) / 4096.0,
            (self.za as f64) / 4096.0,
        )
    }
}

impl PositionDeltaTrait for PositionDelta8 {
    fn x(&self) -> f64 {
        (self.xa as f64) / 4096.0
    }
    fn y(&self) -> f64 {
        (self.ya as f64) / 4096.0
    }
    fn z(&self) -> f64 {
        (self.za as f64) / 4096.0
    }
}

impl Vec3 {
    #[must_use]
    pub fn with_delta(&self, delta: &impl PositionDeltaTrait) -> Vec3 {
        Vec3 {
            x: self.x + delta.x(),
            y: self.y + delta.y(),
            z: self.z + delta.z(),
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let length = f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        if length < 1e-4 {
            return Vec3::default();
        }
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn multiply(&self, x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: self.x * x,
            y: self.y * y,
            z: self.z * z,
        }
    }
    pub fn scale(&self, amount: f64) -> Vec3 {
        self.multiply(amount, amount, amount)
    }
}
