use nalgebra_glm::{Vec3, dot};
use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::light::Light;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light) -> Color {
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

    // Calcular la dirección de la luz
    let light_dir = (light.position - intersect.point).normalize();

    // Calcular el factor de difusión
    let diffuse_intensity = dot(&light_dir, &intersect.normal).max(0.0) * light.intensity;

    // Modificar el color basado en la luz
    let diffuse = intersect.material.diffuse * diffuse_intensity;

    diffuse
}