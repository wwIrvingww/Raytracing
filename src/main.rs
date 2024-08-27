mod camera;
mod color;
mod framebuffer;
mod sphere;
mod ray_intersect;
mod raytracer;
mod light;

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::sphere::Sphere;
use crate::ray_intersect::Material;
use crate::raytracer::cast_ray;
use crate::light::Light;
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

    // Posición inicial de la cámara
    let mut camera = Camera::new(
        Vec3::new(3.0, 2.0, 5.0),    // Cambié la posición inicial de la cámara
        Vec3::new(0.0, 0.0, 0.0),    
        Vec3::new(0.0, 1.0, 0.0),
    );

    let sensitivity = 10.5;

    let black_material = Material {
        diffuse: Color::new(0, 0, 0),
        specular: 10.0,
    };

    let white_material = Material {
        diffuse: Color::new(255, 255, 255),
        specular: 50.0,
    };

    let orange_material = Material {
        diffuse: Color::new(255, 165, 0),
        specular: 25.0,
    };

    // Crear más esferas para la escena
    let ground = Sphere {
        center: Vec3::new(0.0, -10004.0, -3.0), // Esfera grande como suelo
        radius: 10000.0,
        material: white_material,
    };

    let left_sphere = Sphere {
        center: Vec3::new(-2.0, 0.0, -4.0), 
        radius: 1.0,
        material: orange_material,
    };

    let right_sphere = Sphere {
        center: Vec3::new(2.0, 0.0, -4.0),
        radius: 1.0,
        material: black_material,
    };

    let middle_sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -4.0),
        radius: 1.0,
        material: white_material,
    };

    let objects = vec![ground, left_sphere, right_sphere, middle_sphere];

    // Inicializar una luz principal
    let light1 = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        1.0,
    );

    let light2 = Light::new(
        Vec3::new(-5.0, 5.0, 5.0),
        Color::new(255, 0, 0), 
        0.7,
    );

    let lights = vec![light1, light2];

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Movimiento de la cámara
        if window.is_key_down(Key::W) {
            camera.move_camera(Vec3::new(0.0, 0.0, -0.1) * sensitivity);
        }
        if window.is_key_down(Key::S) {
            camera.move_camera(Vec3::new(0.0, 0.0, 0.1) * sensitivity);
        }
        if window.is_key_down(Key::A) {
            camera.move_camera(Vec3::new(-0.1, 0.0, 0.0) * sensitivity);
        }
        if window.is_key_down(Key::D) {
            camera.move_camera(Vec3::new(0.1, 0.0, 0.0) * sensitivity);
        }

        // Renderizado de la escena
        for y in 0..height {
            for x in 0..width {
                let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;
                let aspect_ratio = width as f32 / height as f32;
                let screen_x = screen_x * aspect_ratio;

                let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
                let transformed_ray_direction = camera.basis_change(&ray_direction);

                let mut color = Color::new(0, 0, 0); // Color inicial negro
                for light in &lights {
                    let light_color = cast_ray(&camera.eye, &transformed_ray_direction, &objects, light);
                    color = color.add(&light_color);
                }

                framebuffer.point(x as isize, y as isize, color);
            }
        }

        window.update_with_buffer(&framebuffer.buffer, width, height).unwrap();

        // Imprimir la posición actual de la cámara
        println!("Posición actual de la cámara: {:?}", camera.eye);
    }
}
