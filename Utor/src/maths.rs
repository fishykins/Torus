use num::{Float};

pub fn clamp<T: Float>(min: T, max: T, value: T) -> T {
    if value > max {
        return max;
    } else if value < min {
        return min;
    }
    value
}

/// Lerp between a and b by amount (0 - 1)
pub fn lerp<T: Float>(a: T, b: T, amount: T) -> T {
    if a == b {
        return a;
    }
    if a > b {
        let range: T = a - b;
        return b + ( range * amount);
    } else {
        let range = b - a;
        return a + ( range * amount);
    }
}
