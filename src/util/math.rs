use std::ops::Range;

use rand::Rng;

pub fn map<
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
>(
    value: T,
    from: Range<T>,
    to: Range<T>,
) -> T {
    value - from.start * (to.end - to.start) / (from.end - from.start) + to.start
}

pub fn weighted_random(range: Range<f64>, towards: f64) -> f64 {
    let mut rng = rand::thread_rng();

    let min = range.start.min(range.end);
    let max = range.start.max(range.end);

    let towards_distance = (towards - min).abs();

    let range_distance = max - min;

    let random = rng.gen::<f64>();

    let weighted_random = random * range_distance + min;

    if random <= towards_distance / range_distance {
        towards
    } else {
        weighted_random
    }
}
