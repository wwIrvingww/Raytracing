mod camera;
mod color;
mod framebuffer;
mod sphere;
mod ray_intersect;
mod raytracer;
mod light;  // Asegúrate de importar el módulo light

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::sphere::Sphere;
use crate::ray_intersect::Material;
use crate::raytracer::cast_ray;
use crate::light::Light;  // Asegúrate de importar Light
use nalgebra_glm::Vec3;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Crear la ventana con miniFB
    let mut window = Window::new(
        "Raytracing - Esc para salir",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Cambiar la posición de la cámara
    let mut camera = Camera::new(
        Vec3::new(2.0, 2.0, 5.0),    // Nueva posición de la cámara (eye)
        Vec3::new(0.0, 0.0, 0.0),    // Punto de enfoque (center)
        Vec3::new(0.0, 1.0, 0.0),    // Vector "arriba"
    );

    // Sensibilidad de la cámara (factor de ajuste)
    let sensitivity = 10.5; // Aumenta o disminuye este valor para ajustar la sensibilidad

    // Colores de los materiales
    let black_material = Material {
        diffuse: Color::new(0, 0, 0),
    };
    let white_material = Material {
        diffuse: Color::new(255, 255, 255),
    };
    let orange_material = Material {
        diffuse: Color::new(255, 165, 0),
    };

    // Crear esferas para el pingüino
    let body = Sphere {
        center: Vec3::new(0.0, -0.4, -3.5),
        radius: 0.9,
        material: black_material,
    };

    let belly = Sphere {
        center: Vec3::new(0.0, -0.4, -3.0),
        radius: 0.6,
        material: white_material,
    };

    let head = Sphere {
        center: Vec3::new(0.0, 0.4, -3.0),
        radius: 0.5,
        material: black_material,
    };

    let objects = vec![body, belly, head];

    // Inicializar la luz
    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        1.0,
    );

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Procesar entrada del teclado para mover la cámara
        if window.is_key_down(Key::W) {
            camera.move_camera(Vec3::new(0.0, 0.0, -0.1) * sensitivity);
            println!("Tecla W presionada. Nueva posición de la cámara: {:?}", camera.eye);
        }
        if window.is_key_down(Key::S) {
            camera.move_camera(Vec3::new(0.0, 0.0, 0.1) * sensitivity);
            println!("Tecla S presionada. Nueva posición de la cámara: {:?}", camera.eye);
        }
        if window.is_key_down(Key::A) {
            camera.move_camera(Vec3::new(-0.1, 0.0, 0.0) * sensitivity);
            println!("Tecla A presionada. Nueva posición de la cámara: {:?}", camera.eye);
        }
        if window.is_key_down(Key::D) {
            camera.move_camera(Vec3::new(0.1, 0.0, 0.0) * sensitivity);
            println!("Tecla D presionada. Nueva posición de la cámara: {:?}", camera.eye);
        }

        // Recorrer cada píxel en el framebuffer
        for y in 0..height {
            for x in 0..width {
                let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;
                let aspect_ratio = width as f32 / height as f32;
                let screen_x = screen_x * aspect_ratio;

                let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
                let transformed_ray_direction = camera.basis_change(&ray_direction);

                // Llamar a cast_ray con la luz como parámetro
                let color = cast_ray(&camera.eye, &transformed_ray_direction, &objects, &light);
                framebuffer.point(x as isize, y as isize, color);
            }
        }

        // Renderizar la ventana
        window.update_with_buffer(&framebuffer.buffer, width, height).unwrap();

        // Imprimir la posición actual de la cámara en cada iteración del bucle
        println!("Posición actual de la cámara: {:?}", camera.eye);
    }
}
