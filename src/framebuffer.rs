pub fn rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}

pub struct Framebuffer {
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, width: usize, color: u32) {
        self.buffer[y * width + x] = color;
    }
}
