mod color;
mod framebuffer;
mod triangle;
mod obj_loader;
mod vertex;
mod fragment;
mod shaders;
mod camera;
mod ring;

use crate::color::Color;
use crate::framebuffer::{Framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::triangle::Triangle;
use crate::obj_loader::Model;
use crate::vertex::Vertex;
use crate::shaders::{vertex_shader, fragment_shader, create_model_matrix, create_viewport_matrix, Uniforms};
use crate::camera::Camera;
use crate::ring::create_ring_vertices;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec3, Mat4};
use std::time::Instant;

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: &str,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push(Triangle::new_from_vertices(
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ));
        }
    }

    let mut all_fragments = Vec::new();
    for triangle in &triangles {
        let fragments = triangle.draw(framebuffer);
        all_fragments.extend(fragments);
    }

    for fragment in all_fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        
        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            framebuffer.set_current_color(shaded_color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Sistema Solar - Proyecto 2 [1-6: Planetas | WASD: Cámara | R: Reset | ESC: Salir]",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("No se pudo crear la ventana: {}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    framebuffer.set_background_color(Color::new(10, 5, 20));

    // Cargar modelo de esfera para planetas
    let mut model = Model::load_from_file("sphere.obj")
        .expect("No se pudo cargar sphere.obj");
    model.normalize_and_center(1.5);

    // Crear modelo para la luna (más pequeño)
    let mut moon_model = Model::load_from_file("sphere.obj")
        .expect("No se pudo cargar sphere.obj para la luna");
    moon_model.normalize_and_center(0.4);

    // Crear anillos de Saturno
    let ring_vertices = create_ring_vertices(1.2, 1.8, 100);

    println!("🌍 Sistema Solar - Proyecto 2");
    println!("================================");
    println!("Modelo cargado: sphere.obj");
    println!("  Vértices planeta: {}", model.vertices.len());
    println!("  Vértices luna: {}", moon_model.vertices.len());
    println!("  Vértices anillos: {}", ring_vertices.len());
    println!("\n🎮 CONTROLES:");
    println!("  [1] ☀️  Sol");
    println!("  [2] 🔴 Marte");
    println!("  [3] 🌍 Tierra (con Luna 🌙)");
    println!("  [4] 🟠 Júpiter");
    println!("  [5] 🪐 Saturno (con Anillos 💍)");
    println!("  [6] 🔵 Neptuno");
    println!("  [0] 🧪 Test Shader");
    println!("\n  W/S o ↑/↓: Orbitar verticalmente");
    println!("  A/D o ←/→: Orbitar horizontalmente");
    println!("  Q/E: Zoom");
    println!("  R: Resetear cámara");
    println!("  ESC: Salir");
    println!("================================\n");

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 4.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut current_shader = "test";
    let mut current_planet = "Test";
    let start_time = Instant::now();

    println!("Planeta actual: {} (Shader: {})", current_planet, current_shader);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = start_time.elapsed().as_secs_f32();

        // SELECCIÓN DE PLANETAS
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            current_shader = "sun";
            current_planet = "Sol ☀️";
            println!("\n🌟 Cambiado a: {} (Shader: {})", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            current_shader = "rocky_mars";
            current_planet = "Marte 🔴";
            println!("\n🪨 Cambiado a: {} (Shader: {})", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            current_shader = "rocky_earth";
            current_planet = "Tierra 🌍";
            println!("\n🌊 Cambiado a: {} (Shader: {}) + Luna 🌙", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            current_shader = "gas_jupiter";
            current_planet = "Júpiter 🟠";
            println!("\n🌪️  Cambiado a: {} (Shader: {})", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            current_shader = "gas_saturn";
            current_planet = "Saturno 🪐";
            println!("\n💍 Cambiado a: {} (Shader: {}) + Anillos 💍", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key6, minifb::KeyRepeat::No) {
            current_shader = "ice_neptune";
            current_planet = "Neptuno 🔵";
            println!("\n❄️  Cambiado a: {} (Shader: {})", current_planet, current_shader);
        }
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            current_shader = "test";
            current_planet = "Test 🧪";
            println!("\n🧪 Cambiado a: {} (Shader: {})", current_planet, current_shader);
        }

        // CONTROLES DE CÁMARA
        if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
            camera.orbit(0.0, 0.05);
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
            camera.orbit(0.0, -0.05);
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            camera.orbit(-0.05, 0.0);
        }
        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            camera.orbit(0.05, 0.0);
        }
        if window.is_key_down(Key::Q) {
            camera.zoom(-0.1);
        }
        if window.is_key_down(Key::E) {
            camera.zoom(0.1);
        }

        // Reset cámara
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 4.5),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            );
            rotation = Vec3::new(0.0, 0.0, 0.0);
            println!("📷 Cámara reseteada");
        }

        framebuffer.clear();

        // RENDERIZAR PLANETA PRINCIPAL
        let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix(SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32);
        let viewport_matrix = create_viewport_matrix(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            light_dir: Vec3::new(1.0, 1.0, 1.0),
        };

        render(&mut framebuffer, &uniforms, &model.vertices, current_shader);

        // RENDERIZAR LUNA SI ESTAMOS EN LA TIERRA
        if current_shader == "rocky_earth" {
            let moon_orbit_radius = 2.5;
            let moon_orbit_speed = 0.5;
            let moon_angle = time * moon_orbit_speed;
            
            let moon_position = Vec3::new(
                moon_angle.cos() * moon_orbit_radius,
                0.0,
                moon_angle.sin() * moon_orbit_radius
            );
            
            let moon_model_matrix = create_model_matrix(moon_position, 0.27, Vec3::new(0.0, time * 0.1, 0.0));
            
            let moon_uniforms = Uniforms {
                model_matrix: moon_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                light_dir: Vec3::new(1.0, 1.0, 1.0),
            };
            
            render(&mut framebuffer, &moon_uniforms, &moon_model.vertices, "moon");
        }

        // RENDERIZAR ANILLOS SI ESTAMOS EN SATURNO
        if current_shader == "gas_saturn" {
            let ring_rotation = Vec3::new(0.4, rotation.y, 0.0);
            let ring_model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, ring_rotation);
            
            let ring_uniforms = Uniforms {
                model_matrix: ring_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                light_dir: Vec3::new(1.0, 1.0, 1.0),
            };
            
            render(&mut framebuffer, &ring_uniforms, &ring_vertices, "ring");
        }

        window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}