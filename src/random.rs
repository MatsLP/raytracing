
pub fn random_f64() -> f64 {
    let random_i32;
    unsafe {
        random_i32 = libc::rand();
    }
    random_i32 as f64 / libc::RAND_MAX as f64
}

pub fn random_f64_from_range(min: f64, max: f64) ->f64{
    random_f64() * (max - min) + min
}