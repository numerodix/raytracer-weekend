use rand;
use rand::Rng;


extern "C" {
    fn drand48() -> f64;
}


pub struct RandomGenerator {
    rng: rand::rngs::ThreadRng,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            rng: rand::thread_rng()
        }
    }

    pub fn next(&mut self) -> f32 {
        let mut val: f32 = self.rng.gen();

        // make sure we never return 1.0
        while val == 1.0 {
            val = self.rng.gen();
        }

        assert!(0.0 <= val);
        assert!(val < 1.0);

        val
    }

    pub fn next_c(&self) -> f32 {
        let val = unsafe {
            drand48()
        };
        val as f32
    }
}