use lazy_static::lazy_static;
use rand::{rngs::StdRng, SeedableRng};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Globals {
    pub rng: StdRng,
}

impl Globals {
    fn new(seed: u64) -> Self {
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp_ms = duration_since_epoch.as_secs();

        let rng = StdRng::seed_from_u64(match seed {
            0 => timestamp_ms,
            _ => seed,
        });

        Globals { rng }
    }
}

lazy_static! {
    pub static ref GLOBALS: Mutex<Globals> = Mutex::new(Globals::new(0));
}

pub fn initialize_globals(seed: u64) {
    let mut globals = GLOBALS.lock().unwrap();
    *globals = Globals::new(seed);
}
