use crate::color::Color;

pub struct Canvas {
    pub width: usize, 
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width,
            height,
            buffer: vec![0; width * height],
        };
    }

    pub fn clear(&mut self) {
        let black = Color::new(0.0, 0.0, 0.0);

        for x in 0..self.width {
            for y in 0..self.height {
                self.set_color(x, y, &black);
            }
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Color) {
        if x > self.width-1 ||y > self.height-1 {
            return;
        }
        //let y_offset = self.height - y - 1;
        self.buffer[x + y * self.width] = color.rgb();
    }   

    pub fn color_at(&self, x: usize, y: usize) -> &u32 {
        if x > self.width-1 ||y > self.height-1 {
            return &0;
        }
        //let y_offset = self.height - y - 1;
        return &self.buffer[x + y * self.width];
    }
}