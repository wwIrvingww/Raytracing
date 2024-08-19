use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY; // Distancia infinita al inicio

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;  // Actualizar zbuffer con la menor distancia
            intersect = tmp;  // Actualizar el objeto intersectado más cercano
        }
    }

    if !intersect.is_intersecting {
        // Si no hay intersección, retorna un color de fondo por defecto (por ejemplo, un color del cielo)
        return Color::new(135, 206, 235); // Color de fondo (azul claro del cielo)
    }

    let diffuse = intersect.material.diffuse;

    diffuse
}
