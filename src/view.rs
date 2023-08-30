use minifb::{Key, Window, WindowOptions};
use crate::canvas::Canvas;

pub struct View {
    pub canvas: Canvas,
    pub window: Window,
}

impl View {
    pub fn new(canvas: Canvas) -> Self {
        let mut window = Window::new(
            "",
            canvas.width as usize,
            canvas.height as usize,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return Self {
            canvas,
            window,
        };
    }

    pub fn run(&mut self) {
        //self.canvas.clear();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.canvas.buffer, self.canvas.width as usize, self.canvas.height as usize)
                .unwrap();
        }
    }

    pub fn set_fps(&mut self, num_frames: u32) {
        let seconds_between_frames = 1.0 / num_frames as f32;
        let micros = (seconds_between_frames * 1000000.0).ceil() as u64;
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(micros)));
    }
}