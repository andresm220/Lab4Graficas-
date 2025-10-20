use sdl2::{pixels::PixelFormatEnum, render::Canvas, video::Window, Sdl};
use image::{ImageBuffer, Rgba};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub struct Framebuffer {
    pub canvas: Canvas<Window>,
    pub buffer: Vec<u32>,
    pub depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(sdl: &Sdl) -> Self {
        let video = sdl.video().unwrap();
        let window = video
            .window("Software Renderer (Rust)", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().present_vsync().build().unwrap();

        let n = (WIDTH * HEIGHT) as usize;
        Self {
            canvas,
            buffer: vec![0x101018ff; n],
            depth: vec![f32::INFINITY; n],
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
        self.depth.fill(f32::INFINITY);
    }

    fn idx(x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return None;
        }
        Some((y as u32 * WIDTH + x as u32) as usize)
    }

    pub fn put_pixel_depth(&mut self, x: i32, y: i32, z: f32, color: u32) {
        if let Some(i) = Self::idx(x, y) {
            if z < self.depth[i] {
                self.depth[i] = z;
                self.buffer[i] = color;
            }
        }
    }

    pub fn present(&mut self) {
        use sdl2::surface::Surface;
        let mut surface = Surface::from_data(
            unsafe {
                std::slice::from_raw_parts_mut(
                    self.buffer.as_mut_ptr() as *mut u8,
                    (WIDTH * HEIGHT * 4) as usize,
                )
            },
            WIDTH,
            HEIGHT,
            WIDTH * 4,
            PixelFormatEnum::ARGB8888,
        )
        .unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&mut surface)
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }

    /// Guarda el framebuffer actual como imagen PNG
    pub fn save_png(&self, path: &str) {
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = (y * WIDTH + x) as usize;
                let color = self.buffer[i];
                let r = ((color >> 16) & 0xFF) as u8;
                let g = ((color >> 8) & 0xFF) as u8;
                let b = (color & 0xFF) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        img.save(path).expect("No se pudo guardar el PNG");
    }
}
