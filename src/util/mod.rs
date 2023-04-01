use rand::Rng;

pub fn randomI32() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);
    random_number
}

use std::collections::HashSet;

pub fn unique(nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    for &num in nums.iter() {
        set.insert(num);
    }
    let unique_arr: Vec<_> = set.into_iter().collect();
    unique_arr
}
