mod camera;
mod color;
mod framebuffer;
mod sphere;
mod ray_intersect;
mod raytracer; // Importa el módulo raytracer

use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::sphere::Sphere;
use crate::ray_intersect::Material; // Importa la estructura Material
use crate::raytracer::cast_ray; // Importa la función cast_ray
use nalgebra_glm::Vec3;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Crear la cámara
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),    // Posición de la cámara
        Vec3::new(0.0, 0.0, 0.0),    // Punto de enfoque (centro)
        Vec3::new(0.0, 1.0, 0.0),    // Vector "arriba"
    );

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
    let _grey_material = Material {
        diffuse: Color::new(169, 169, 169),
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

    let left_eye = Sphere {
        center: Vec3::new(-0.2, 0.5, -2.5),
        radius: 0.1,
        material: white_material,
    };

    let right_eye = Sphere {
        center: Vec3::new(0.2, 0.5, -2.5),
        radius: 0.1,
        material: white_material,
    };

    let left_pupil = Sphere {
        center: Vec3::new(-0.2, 0.5, -2.4),
        radius: 0.05,
        material: black_material,
    };

    let right_pupil = Sphere {
        center: Vec3::new(0.2, 0.5, -2.4),
        radius: 0.05,
        material: black_material,
    };

    let beak = Sphere {
        center: Vec3::new(0.0, 0.3, -2.4),
        radius: 0.08,
        material: orange_material,
    };

    let left_wing = Sphere {
        center: Vec3::new(-0.6, -0.4, -3.2),
        radius: 0.4,
        material: black_material,
    };

    let right_wing = Sphere {
        center: Vec3::new(0.6, -0.4, -3.2),
        radius: 0.4,
        material: black_material,
    };

    // Agregar las esferas a la lista de objetos
    let objects = vec![
        body, belly, head, left_eye, right_eye, left_pupil, right_pupil,
        beak, left_wing, right_wing,
    ];

    // Recorrer cada píxel en el framebuffer
    for y in 0..height {
        for x in 0..width {
            // Convertir las coordenadas del píxel a coordenadas de pantalla
            let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
            let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;
            let aspect_ratio = width as f32 / height as f32;
            let screen_x = screen_x * aspect_ratio;

            // Direccion del rayo desde la cámara a través del píxel
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            // Cambiar la base del rayo
            let transformed_ray_direction = camera.basis_change(&ray_direction);

            // Obtener el color del objeto más cercano
            let color = cast_ray(&camera.eye, &transformed_ray_direction, &objects);

            // Dibujar el píxel en el framebuffer
            framebuffer.point(x as isize, y as isize, color);
        }
    }

    // Renderizar la ventana
    framebuffer.render_window();
}
