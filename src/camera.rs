#[allow(dead_code)]
use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,    // Posición de la cámara en el espacio mundial
    pub center: Vec3, // Punto que la cámara está mirando
    pub up: Vec3,     // Vector "arriba" de la cámara
}

impl Camera {
    // Constructor para crear una nueva cámara
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera { eye, center, up }
    }

    // Método para obtener la dirección de la vista (center - eye)
    pub fn view_direction(&self) -> Vec3 {
        (self.center - self.eye).normalize()
    }

    // Cambio de base
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize(); // Eje z negativo
        let right = forward.cross(&self.up).normalize();     // Eje x
        let up = right.cross(&forward).normalize();          // Eje y

        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated.normalize()
    }

    // Método para mover la cámara alrededor del centro en órbita
    pub fn orbit(&mut self, angle: f32, distance: f32) {
        let direction = self.view_direction();
        let right = direction.cross(&self.up).normalize();
        let new_eye = self.center - direction * distance + right * angle.sin();
        self.eye = new_eye;
    }

    // Método para actualizar la posición de la cámara
    pub fn move_camera(&mut self, delta: Vec3) {
        self.eye += delta;
        self.center += delta;
    }
}
