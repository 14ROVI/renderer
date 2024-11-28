pub struct Canvas {
    width: u32,
    height: u32,
    pixel_buffer: Vec<u8>,
    depth_buffer: Vec<f32>,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            pixel_buffer: vec![0; (width * height * 3) as usize],
            depth_buffer: vec![0.0; (width * height) as usize],
        }
    }

    pub fn get_pixels(&self) -> Vec<u8> {
        self.pixel_buffer.clone()
    }

    pub fn get_canvas_buffer(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![];
        buffer.reserve((self.width * self.height * 4) as usize);

        for i in 0..((self.width * self.height) as usize) {
            buffer.push(
                (255 << 24)
                    + ((self.pixel_buffer[3 * i + 0] as u32) << 16)
                    + ((self.pixel_buffer[3 * i + 1] as u32) << 8)
                    + (self.pixel_buffer[3 * i + 2] as u32),
            );
        }

        buffer
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, depth: f32) {
        let di = (y * self.width + x) as usize;

        if depth > self.depth_buffer[di] {
            self.depth_buffer[di] = depth;

            let pi = 3 * (y * self.width + x) as usize;
            self.pixel_buffer[pi] = r;
            self.pixel_buffer[pi + 1] = g;
            self.pixel_buffer[pi + 2] = b;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let i = 3 * (y * self.width + x) as usize;
        (
            self.pixel_buffer[i],
            self.pixel_buffer[i + 1],
            self.pixel_buffer[i + 2],
        )
    }

    pub fn get_depth(&self, x: u32, y: u32) -> f32 {
        let i = (y * self.width + x) as usize;
        self.depth_buffer[i]
    }
}

// pub trait DrawCanvas {
//     fn draw_canvas(&mut self, canvas: &Canvas);
// }
// impl DrawCanvas for RenderTexture<u8> {
//     fn draw_canvas(&mut self, canvas: &Canvas) {
//         for (i, pixel) in canvas.get_pixels().chunks(3).enumerate() {
//             let x = i as u32 % canvas.width;
//             let y = i as u32 / canvas.width;
//             self.set_pixel(x, y, pixel[0], pixel[1], pixel[2]);
//         }
//     }
// }
