use std::time::{Duration, Instant};

pub struct Timer {
    current_time: Duration,
    delta: Duration,
    last_instant: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            current_time: Duration::new(0, 0),
            delta: Duration::new(0, 0),
            last_instant: Instant::now(),
        }
    }
}

impl Timer {
    pub fn tick(&mut self) {
        let prev_time = self.current_time;
        self.current_time += self.last_instant.elapsed();
        self.delta = self.current_time - prev_time;
        self.last_instant = Instant::now();
    }

    pub fn delta_time_secs(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn time_elapsed_secs(&self) -> f32 {
        self.current_time.as_secs_f32()
    }
}
