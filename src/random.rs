extern "C" {
    fn drand48() -> f64;
}

pub fn rand48() -> f32 {
    let val = unsafe {
        drand48()
    };
    val as f32
}