use rand::thread_rng;
use rand::rngs::ThreadRng;

pub struct RandomWrapper {
    rng: ThreadRng,
}

impl RandomWrapper {
    pub fn new() -> RandomWrapper {
        RandomWrapper { rng: thread_rng() }
    }

    pub fn get(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}
