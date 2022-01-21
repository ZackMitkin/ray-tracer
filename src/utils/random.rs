use rand::Rng;

const RAND_MAX: u32 = 0;

pub fn random_f64() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_f64_min_max(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
