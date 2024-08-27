use nalgebra_glm::{Vec3, dot};
use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::light::Light;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(135, 206, 235); // Color de fondo
    }

    // Calcular la dirección de la luz
    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&(-light_dir), &intersect.normal);

    // Calcular el factor de difusión
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
    let diffuse = intersect.material.diffuse * diffuse_intensity * light.intensity;

    // Calcular el componente especular
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * specular_intensity * light.intensity;

    // Sumar el componente difuso y especular
    diffuse + specular
}
