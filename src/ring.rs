use crate::vertex::Vertex;
use nalgebra_glm::Vec3;

pub fn create_ring_vertices(inner_radius: f32, outer_radius: f32, segments: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
        
        let cos1 = angle1.cos();
        let sin1 = angle1.sin();
        let cos2 = angle2.cos();
        let sin2 = angle2.sin();
        
        // Triángulo 1
        vertices.push(Vertex::new(
            Vec3::new(cos1 * inner_radius, 0.0, sin1 * inner_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(0.0, 0.0),
        ));
        vertices.push(Vertex::new(
            Vec3::new(cos1 * outer_radius, 0.0, sin1 * outer_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::new(
            Vec3::new(cos2 * inner_radius, 0.0, sin2 * inner_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(0.0, 1.0),
        ));
        
        // Triángulo 2
        vertices.push(Vertex::new(
            Vec3::new(cos2 * inner_radius, 0.0, sin2 * inner_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(0.0, 1.0),
        ));
        vertices.push(Vertex::new(
            Vec3::new(cos1 * outer_radius, 0.0, sin1 * outer_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(1.0, 0.0),
        ));
        vertices.push(Vertex::new(
            Vec3::new(cos2 * outer_radius, 0.0, sin2 * outer_radius),
            Vec3::new(0.0, 1.0, 0.0),
            nalgebra_glm::Vec2::new(1.0, 1.0),
        ));
    }
    
    vertices
}