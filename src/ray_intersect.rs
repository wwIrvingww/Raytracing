use nalgebra_glm::Vec3;
use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32, // Coeficiente especular
}

impl Material {
    pub fn new(diffuse: Color, specular: f32) -> Self {
        Material {
            diffuse,
            specular,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]

pub struct Intersect {
    pub point: Vec3,       // El punto de impacto
    pub normal: Vec3,      // La normal en el punto de impacto
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0), // Color negro por defecto para no intersección
                specular: 0.0, // Valor especular por defecto
            },
        }
    }
    
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

