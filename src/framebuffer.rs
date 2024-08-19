use crate::color::Color;
use minifb::{Key, Window, WindowOptions};

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>, // Cambia el buffer a u32 para adaptarse a minifb
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height], // Inicializar con color negro
        }
    }

    pub fn point(&mut self, x: isize, y: isize, color: Color) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            // Convertir el color RGB a formato u32 (ARGB)
            let color_u32 = (255 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
            self.buffer[index] = color_u32;
        }
    }

    pub fn render_window(&self) {
        let mut window = Window::new(
            "Leonardo DiPenguin",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Mientras la ventana esté abierta y no se presione la tecla ESC
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }
    }
}
