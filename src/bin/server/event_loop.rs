use std::thread;
use std::time;

#[derive(Debug)]
pub struct EventLoop {
    last_tick_time: time::Instant,
    fps: u32,
    fps_in_nanos: f32,
}

impl EventLoop {
    pub fn new(fps: u32) -> EventLoop {
        EventLoop {
            last_tick_time: time::Instant::now(),
            fps,
            fps_in_nanos: (1. / fps as f32) * 1_000_000_000.,
        }
    }

    pub fn tick(&mut self) {
        let elapsed = self.last_tick_time.elapsed();
        let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
        let delta = self.fps_in_nanos - (nanos as f32);
        if delta > 0. {
            thread::sleep(time::Duration::new(0, delta as u32))
        };
        self.last_tick_time = time::Instant::now();
    }
}