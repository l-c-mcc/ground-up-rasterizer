use std::time::{Duration, Instant};

pub struct Timer {
    current_time: Duration,
    last_instant: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            current_time: Duration::new(0, 0),
            last_instant: Instant::now(),
        }
    }
}

impl Timer {
    pub fn update(&mut self) -> f32 {
        self.current_time += self.last_instant.elapsed();
        self.last_instant = Instant::now();
        self.current_time.as_secs_f32()
    }
}
