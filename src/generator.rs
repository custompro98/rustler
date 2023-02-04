use rand::prelude::*;

// Rand returns a integer between [1, n]
pub fn rand(n: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let max = n + 1;

    return rng.gen_range(1..max)
}
