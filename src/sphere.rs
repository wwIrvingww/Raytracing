use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect, Material};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / (2.0 * a);
            if distance > 0.0 {
                let point = ray_origin + ray_direction * distance; // Calculamos el punto de impacto
                let normal = (point - self.center).normalize();    // Calculamos la normal en el punto de impacto

                return Intersect::new(point, normal, distance, self.material);
            }
        }

        Intersect::empty()
    }
}