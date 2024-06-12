use std::thread;
use std::time::{Duration, Instant};

pub struct FramesLimiter {
    pub max_frames_per_second: u32,
    min_frame_time_in_nanoseconds: u128,
}

impl FramesLimiter {
    pub fn new(max_frames_per_second: u32) -> Self {
        Self {
            max_frames_per_second,
            min_frame_time_in_nanoseconds: 1000_000_000 / (max_frames_per_second as u128),
        }
    }

    pub fn control_frame(&self, mut func: impl FnMut() -> ()) {
        let start = Instant::now();

        func();

        let elapsed = start.elapsed();

        if elapsed.as_nanos() >= self.min_frame_time_in_nanoseconds {
            return;
        }

        let nanos_left = self.min_frame_time_in_nanoseconds - elapsed.as_nanos();
        thread::sleep(Duration::from_nanos(nanos_left as u64));

        // println!("elapsed: {}", start.elapsed().as_nanos());
    }
}
