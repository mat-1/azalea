use std::{f64::consts::PI, sync::LazyLock};

pub const EPSILON: f64 = 1.0E-7;

pub static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
    let mut sin = [0.0; 65536];
    for (i, item) in sin.iter_mut().enumerate() {
        *item = f64::sin((i as f64) * PI * 2.0 / 65536.0) as f32;
    }
    sin
});

/// A sine function that uses a lookup table.
pub fn sin(x: f32) -> f32 {
    let x = x * 10430.378;
    let x = x as i32 as usize & 65535;
    SIN[x]
}

/// A cosine function that uses a lookup table.
pub fn cos(x: f32) -> f32 {
    let x = x * 10430.378 + 16384.0;
    let x = x as i32 as usize & 65535;
    SIN[x]
}

// TODO: make this generic
pub fn binary_search(mut min: i32, max: i32, predicate: impl Fn(i32) -> bool) -> i32 {
    let mut diff = max - min;
    while diff > 0 {
        let diff_mid = diff / 2;
        let mid = min + diff_mid;
        if predicate(mid) {
            diff = diff_mid;
        } else {
            min = mid + 1;
            diff -= diff_mid + 1;
        }
    }

    min
}

pub fn lcm(a: u32, b: u32) -> u64 {
    let gcd = gcd(a, b);
    (a as u64) * (b / gcd) as u64
}
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lerp<T: num_traits::Float>(amount: T, a: T, b: T) -> T {
    a + amount * (b - a)
}

pub fn ceil_log2(x: u32) -> u32 {
    u32::BITS - x.leading_zeros()
}

pub fn fract(x: f64) -> f64 {
    let x_int = x as i64 as f64;
    let floor = if x < x_int { x_int - 1. } else { x_int };
    x - floor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 1), 1);

        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);

        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(8, 12), 4);

        assert_eq!(gcd(12, 9), 3);
        assert_eq!(gcd(9, 12), 3);

        assert_eq!(gcd(12, 7), 1);
        assert_eq!(gcd(7, 12), 1);
    }

    #[test]
    fn test_sin() {
        const PI: f32 = std::f32::consts::PI;
        // check that they're close enough
        fn assert_sin_eq_enough(number: f32) {
            let a = sin(number);
            let b = f32::sin(number);
            assert!((a - b).abs() < 0.01, "sin({number}) failed, {a} != {b}");
        }
        assert_sin_eq_enough(0.0);
        assert_sin_eq_enough(PI / 2.0);
        assert_sin_eq_enough(PI);
        assert_sin_eq_enough(PI * 2.0);
        assert_sin_eq_enough(PI * 3.0 / 2.0);
        assert_sin_eq_enough(-PI / 2.0);
        assert_sin_eq_enough(-PI);
        assert_sin_eq_enough(-PI * 2.0);
        assert_sin_eq_enough(-PI * 3.0 / 2.0);
    }
}
