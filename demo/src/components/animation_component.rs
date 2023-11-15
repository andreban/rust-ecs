use rust_ecs::{derive::Component, Component};

#[derive(Component, Debug)]
pub struct AnimationComponent {
    pub num_frames: usize,
    pub current_frame: usize,
    pub framerate: usize,
    pub start_time: usize,
    pub is_loop: bool,
}

impl AnimationComponent {
    pub fn new() -> Self {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        Self { num_frames: 1, current_frame: 0, framerate: 1, start_time: time, is_loop: false }
    }

    pub fn num_frames(mut self, num_frames: usize) -> Self {
        self.num_frames = num_frames;
        self
    }

    pub fn framerate(mut self, framerate: usize) -> Self {
        self.framerate = framerate;
        self
    }

    pub fn is_loop(mut self, is_loop: bool) -> Self {
        self.is_loop = is_loop;
        self
    }
}
