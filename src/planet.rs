use nalgebra_glm::Vec3;

#[derive(Clone)]
pub struct Planet {
    pub name: String,
    pub shader_type: String,
    pub scale: f32,
    pub position: Vec3,
    pub rotation: Vec3,
    pub rotation_speed: Vec3,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
}

impl Planet {
    pub fn new(
        name: &str,
        shader_type: &str,
        scale: f32,
        orbit_radius: f32,
        orbit_speed: f32,
    ) -> Self {
        Planet {
            name: name.to_string(),
            shader_type: shader_type.to_string(),
            scale,
            position: Vec3::zeros(),
            rotation: Vec3::zeros(),
            rotation_speed: Vec3::new(0.0, 0.01, 0.0), // Rotación por defecto
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Actualizar rotación
        self.rotation += self.rotation_speed * delta_time;

        // Actualizar órbita
        self.orbit_angle += self.orbit_speed * delta_time;
        
        // Calcular posición en órbita
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
    }
}

// Función helper para crear todos los planetas del sistema solar
pub fn create_solar_system() -> Vec<Planet> {
    vec![
        Planet::new("Sol", "sun", 2.0, 0.0, 0.0),
        Planet::new("Marte", "rocky_mars", 0.5, 3.0, 0.5),
        Planet::new("Tierra", "rocky_earth", 0.6, 4.0, 0.4),
        Planet::new("Júpiter", "gas_jupiter", 1.2, 6.0, 0.2),
        Planet::new("Saturno", "gas_saturn", 1.0, 8.0, 0.15),
        Planet::new("Neptuno", "ice_neptune", 0.7, 10.0, 0.1),
    ]
}