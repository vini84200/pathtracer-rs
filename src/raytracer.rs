pub struct Pathtracer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Pathtracer {
    pub fn new(width : u32, height : u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn resize(&mut self, width : u32, height : u32) {
        self.width = width;
        self.height = height;
        self.pixels = vec![0; (width * height * 4) as usize];
    }

    pub fn render(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (x + y * self.width) as usize * 4;
                self.pixels[offset    ] = (x % 256) as u8;
                self.pixels[offset + 1] = (y % 256) as u8;
                self.pixels[offset + 2] = 0;
                self.pixels[offset + 3] = 255;
            }
        }
    }

    pub fn present(&self, window : &winit::window::Window) {
        // ...
    }
}