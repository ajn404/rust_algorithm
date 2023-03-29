
use rand::Rng;

pub fn randomI32() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);
    random_number
}
