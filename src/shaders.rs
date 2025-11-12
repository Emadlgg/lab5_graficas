use nalgebra_glm::{Vec3, Vec4, Mat4, Vec2};
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::color::Color;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
    pub light_dir: Vec3,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * ndc_position;

    let model_mat3 = Mat4::new(
        uniforms.model_matrix[(0, 0)], uniforms.model_matrix[(0, 1)], uniforms.model_matrix[(0, 2)], 0.0,
        uniforms.model_matrix[(1, 0)], uniforms.model_matrix[(1, 1)], uniforms.model_matrix[(1, 2)], 0.0,
        uniforms.model_matrix[(2, 0)], uniforms.model_matrix[(2, 1)], uniforms.model_matrix[(2, 2)], 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let normal4 = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
    let transformed_normal = model_mat3 * normal4;
    let final_normal = Vec3::new(transformed_normal.x, transformed_normal.y, transformed_normal.z).normalize();

    let mut new_vertex = vertex.clone();
    new_vertex.transformed_position = Vec3::new(screen_position.x, screen_position.y, screen_position.z);
    new_vertex.transformed_normal = final_normal;

    new_vertex
}

// ============================================
// SISTEMA DE FRAGMENT SHADERS PARA PLANETAS
// ============================================

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str) -> Color {
    match shader_type {
        "sun" => sun_shader(fragment, uniforms),
        "rocky_mars" => mars_shader(fragment, uniforms),
        "rocky_earth" => earth_shader(fragment, uniforms),
        "gas_jupiter" => jupiter_shader(fragment, uniforms),
        "gas_saturn" => saturn_shader(fragment, uniforms),
        "ice_neptune" => neptune_shader(fragment, uniforms),
        "moon" => moon_shader(fragment, uniforms),
        "ring" => ring_shader(fragment, uniforms),
        "test" => test_shader(fragment, uniforms),
        _ => default_shader(fragment, uniforms)
    }
}

// ============================================
// SHADER DE PRUEBA (Con iluminación mejorada)
// ============================================
fn test_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let light_dir = uniforms.light_dir.normalize();
    let normal = fragment.normal.normalize();
    
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.2;
    let intensity = ambient + diffuse * 0.8;
    
    let base_color = Color::new(255, 255, 0);
    base_color * intensity
}

// ============================================
// SHADER DEFAULT (Con iluminación)
// ============================================
fn default_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let light_dir = uniforms.light_dir.normalize();
    let normal = fragment.normal.normalize();
    
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.2;
    let intensity = ambient + diffuse * 0.8;
    
    fragment.color * intensity
}

// ============================================
// SHADER: SOL (Estrella) - 5 CAPAS
// ============================================
fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Color Base Amarillo-Naranja Brillante
    let core_yellow = Color::new(255, 220, 100);
    let surface_orange = Color::new(255, 180, 80);
    let bright_yellow = Color::new(255, 240, 150);
    
    let base_noise = fbm(uv.x * 4.0, uv.y * 4.0, 3);
    
    let base_color = if base_noise > 0.6 {
        mix_color(&core_yellow, &bright_yellow, (base_noise - 0.6) * 2.0)
    } else if base_noise < 0.4 {
        mix_color(&core_yellow, &surface_orange, (0.4 - base_noise) * 2.0)
    } else {
        core_yellow
    };
    
    // CAPA 2: Manchas Solares (Sunspots)
    let sunspot_color = Color::new(180, 100, 40);
    let sunspot_core = Color::new(120, 60, 20);
    
    let spot_noise1 = fbm(uv.x * 8.0 + uniforms.time * 0.01, uv.y * 8.0, 4);
    let spot_noise2 = fbm(uv.x * 15.0 - uniforms.time * 0.008, uv.y * 15.0 + 50.0, 3);
    
    let combined_spots = (spot_noise1 + spot_noise2) / 2.0;
    
    let color_with_spots = if combined_spots > 0.68 {
        let spot_intensity = smoothstep(0.68, 0.78, combined_spots);
        
        let spot_color_final = if combined_spots > 0.73 {
            mix_color(&sunspot_color, &sunspot_core, (combined_spots - 0.73) * 5.0)
        } else {
            sunspot_color
        };
        
        mix_color(&base_color, &spot_color_final, spot_intensity * 0.4)
    } else {
        base_color
    };
    
    // CAPA 3: Granulación Solar
    let granule_bright = Color::new(255, 230, 120);
    let granule_dark = Color::new(240, 190, 90);
    
    let granulation = fbm(uv.x * 40.0, uv.y * 40.0, 4);
    
    let color_with_granulation = if granulation > 0.52 {
        let gran_factor = (granulation - 0.52) * 2.0;
        mix_color(&color_with_spots, &granule_bright, gran_factor * 0.2)
    } else if granulation < 0.48 {
        let gran_factor = (0.48 - granulation) * 2.0;
        mix_color(&color_with_spots, &granule_dark, gran_factor * 0.15)
    } else {
        color_with_spots
    };
    
    // CAPA 4: Erupciones Solares ANIMADAS
    let flare_color = Color::new(255, 100, 50);
    let flare_bright = Color::new(255, 200, 100);
    
    let flare_noise = fbm(
        uv.x * 6.0 + uniforms.time * 0.05,
        uv.y * 6.0 + (uniforms.time * 0.03).sin() * 0.5,
        5
    );
    
    let time_pulse = (uniforms.time * 2.0).sin() * 0.5 + 0.5;
    let flare_threshold = 0.65 + time_pulse * 0.1;
    
    let color_with_flares = if flare_noise > flare_threshold {
        let flare_intensity = smoothstep(flare_threshold, flare_threshold + 0.15, flare_noise);
        let flare_final = mix_color(&flare_color, &flare_bright, flare_intensity);
        mix_color(&color_with_granulation, &flare_final, flare_intensity * 0.6)
    } else {
        color_with_granulation
    };
    
    // CAPA 5: Corona Solar
    let corona_color = Color::new(255, 200, 100);
    
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let fresnel = 1.0 - nalgebra_glm::dot(&normal, &view_dir).abs();
    let fresnel_pow = fresnel.powf(2.0);
    
    let color_with_corona = if fresnel_pow > 0.3 {
        let corona_intensity = (fresnel_pow - 0.3) * 1.5;
        
        let corona_variation = fbm(
            uv.x * 20.0 + uniforms.time * 0.02,
            uv.y * 20.0,
            3
        );
        
        let corona_factor = (corona_intensity * (0.8 + corona_variation * 0.4)).min(0.8);
        mix_color(&color_with_flares, &corona_color, corona_factor)
    } else {
        color_with_flares
    };
    
    // Brillo propio
    let brightness_boost = 1.15;
    
    Color::new(
        (color_with_corona.r as f32 * brightness_boost).min(255.0) as u8,
        (color_with_corona.g as f32 * brightness_boost).min(255.0) as u8,
        (color_with_corona.b as f32 * brightness_boost).min(255.0) as u8,
    )
}

// ============================================
// SHADER: MARTE (Planeta Rocoso) - 4 CAPAS
// ============================================
fn mars_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Color Base Rojo-Naranja
    let rust_color = Color::new(193, 68, 14);
    let dark_rust = Color::new(120, 40, 10);
    let light_rust = Color::new(220, 100, 50);
    
    let base_noise = fbm(uv.x * 8.0, uv.y * 8.0, 3);
    let base_color = if base_noise > 0.6 {
        mix_color(&rust_color, &light_rust, (base_noise - 0.6) * 2.5)
    } else if base_noise < 0.4 {
        mix_color(&rust_color, &dark_rust, (0.4 - base_noise) * 2.5)
    } else {
        rust_color
    };
    
    // CAPA 2: Cráteres REALISTAS
    let crater_color = Color::new(80, 30, 10);
    let rim_color = Color::new(210, 85, 30);
    
    let mut color_with_craters = base_color;
    
    let crater_noise1 = fbm(uv.x * 15.0, uv.y * 15.0, 5);
    let crater_noise2 = fbm(uv.x * 25.0 + 100.0, uv.y * 25.0 + 100.0, 4);
    let crater_noise3 = fbm(uv.x * 40.0 + 200.0, uv.y * 40.0 + 200.0, 3);
    
    if crater_noise1 > 0.72 {
        let crater_intensity = smoothstep(0.72, 0.85, crater_noise1);
        color_with_craters = mix_color(&color_with_craters, &crater_color, crater_intensity * 0.4);
    }
    
    if crater_noise2 > 0.68 {
        let crater_intensity = smoothstep(0.68, 0.78, crater_noise2);
        color_with_craters = mix_color(&color_with_craters, &crater_color, crater_intensity * 0.35);
    }
    
    if crater_noise3 > 0.65 {
        let crater_intensity = smoothstep(0.65, 0.72, crater_noise3);
        color_with_craters = mix_color(&color_with_craters, &crater_color, crater_intensity * 0.25);
    }
    
    let rim_noise = fbm(uv.x * 20.0 + 50.0, uv.y * 20.0 + 50.0, 4);
    if rim_noise > 0.70 && rim_noise < 0.74 {
        let rim_intensity = 1.0 - ((rim_noise - 0.70) / 0.04 - 0.5).abs() * 2.0;
        color_with_craters = mix_color(&color_with_craters, &rim_color, rim_intensity * 0.15);
    }
    
    // CAPA 3: Casquetes Polares
    let polar_threshold = 0.87;
    let latitude = fragment.normal.y.abs();
    let ice_color = Color::new(245, 248, 255);
    
    let color_with_poles = if latitude > polar_threshold {
        let pole_factor = smoothstep(polar_threshold, 0.96, latitude);
        let pole_noise = fbm(uv.x * 60.0, uv.y * 60.0, 3);
        let pole_factor_varied = (pole_factor * (0.5 + pole_noise * 0.5)).clamp(0.0, 1.0);
        mix_color(&color_with_craters, &ice_color, pole_factor_varied * 0.9)
    } else {
        color_with_craters
    };
    
    // CAPA 4: Variación de Terreno
    let terrain_noise = fbm(uv.x * 10.0, uv.y * 10.0, 4);
    let valley_color = Color::new(150, 55, 18);
    let mountain_color = Color::new(205, 85, 35);
    
    let color_with_terrain = if terrain_noise > 0.58 {
        let mountain_factor = (terrain_noise - 0.58) * 2.5;
        mix_color(&color_with_poles, &mountain_color, mountain_factor * 0.2)
    } else if terrain_noise < 0.42 {
        let valley_factor = (0.42 - terrain_noise) * 2.5;
        mix_color(&color_with_poles, &valley_color, valley_factor * 0.2)
    } else {
        color_with_poles
    };
    
    // Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.3;
    let intensity = ambient + diffuse * 0.7;
    
    color_with_terrain * intensity
}

fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let uv = get_uv_from_position(&fragment.normal);

    // ============================================================
    // CAPA 1: OCÉANOS
    // ============================================================
    let deep_ocean = Color::new(8, 25, 80);
    let ocean_mid = Color::new(25, 100, 200);
    let shallow_ocean = Color::new(80, 170, 230);

    let ocean_noise = fbm(uv.x * 6.0, uv.y * 6.0, 5);
    let base_ocean = if ocean_noise < 0.5 {
        mix_color(&deep_ocean, &ocean_mid, ocean_noise * 2.0)
    } else {
        mix_color(&ocean_mid, &shallow_ocean, (ocean_noise - 0.5) * 2.0)
    };

    // ============================================================
    // CAPA 2: CONTINENTES
    // ============================================================
    let land_grass = Color::new(75, 130, 55);
    let land_forest = Color::new(40, 95, 40);
    let land_desert = Color::new(210, 185, 110);
    let land_mountain = Color::new(160, 150, 140);

    let continent_noise = fbm(uv.x * 2.0, uv.y * 2.0, 6);

    // 🔹 Ligeramente más tierra que antes
    let land_threshold = 0.74;

    let mut color_with_land = base_ocean;

    if continent_noise > land_threshold {
        let terrain = fbm(uv.x * 10.0 + 20.0, uv.y * 10.0 + 80.0, 4);
        let land_color = if terrain > 0.72 {
            land_mountain
        } else if terrain > 0.55 {
            land_forest
        } else if terrain > 0.4 {
            land_grass
        } else {
            land_desert
        };

        // Borde suave entre costa y mar
        let coast = smoothstep(land_threshold - 0.04, land_threshold + 0.05, continent_noise);
        color_with_land = mix_color(&base_ocean, &land_color, coast);
    }

    // ============================================================
    // CAPA 3: POLOS
    // ============================================================
    let ice_color = Color::new(250, 250, 255);
    let ice_shadow = Color::new(220, 230, 245);
    let latitude = fragment.normal.y.abs();
    let polar_start = 0.77;

    let mut color_with_poles = color_with_land;
    if latitude > polar_start {
        let pole_factor = smoothstep(polar_start, 0.95, latitude);
        let ice_pattern = fbm(uv.x * 40.0, uv.y * 40.0, 3);
        let ice_mix = mix_color(&ice_shadow, &ice_color, ice_pattern);
        color_with_poles = mix_color(&color_with_land, &ice_mix, pole_factor);
    }

    // ============================================================
    // CAPA 4: NUBES
    // ============================================================
    let cloud_color = Color::new(255, 255, 255);
    let cloud_noise1 = fbm(uv.x * 6.0 + uniforms.time * 0.005, uv.y * 6.0, 4);
    let cloud_noise2 = fbm(uv.x * 12.0 - uniforms.time * 0.004, uv.y * 12.0 + 50.0, 3);
    let clouds = (cloud_noise1 * 0.6 + cloud_noise2 * 0.4).powf(1.4);
    let cloud_threshold = 0.73;

    let mut color_with_clouds = color_with_poles;
    if clouds > cloud_threshold {
        let cloud_intensity = smoothstep(cloud_threshold, 0.9, clouds);
        color_with_clouds = mix_color(&color_with_poles, &cloud_color, cloud_intensity * 0.35);
    }

    // ============================================================
    // CAPA 5: ATMÓSFERA con dispersión
    // ============================================================
    let atmosphere_color = Color::new(120, 190, 255);
    let fresnel = (1.0 - nalgebra_glm::dot(&normal, &view_dir).abs()).powf(4.0);

    // Dispersión azulada más fuerte hacia los bordes
    let atmosphere_intensity = fresnel * 0.5;
    let color_with_atmosphere = mix_color(&color_with_clouds, &atmosphere_color, atmosphere_intensity);

    // ============================================================
    // EFECTO ESPECULAR sobre el océano
    // ============================================================
    let half_dir = (light_dir + view_dir).normalize();
    let spec = nalgebra_glm::dot(&normal, &half_dir).max(0.0).powf(80.0);
    let specular_strength = 0.5;
    let specular_color = Color::new(180, 220, 255) * (spec * specular_strength);

    // Aplicamos solo en regiones oceánicas (usando inverso del threshold)
    let ocean_factor = smoothstep(0.6, land_threshold - 0.05, continent_noise);
    let color_with_specular = color_with_atmosphere + (specular_color * ocean_factor);

    // ============================================================
    // ILUMINACIÓN FINAL
    // ============================================================
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.22;
    let intensity = ambient + diffuse * 0.78;

    color_with_specular * intensity
}


// ============================================
// SHADER: JÚPITER (Gigante Gaseoso) - 4 CAPAS
// ============================================
fn jupiter_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Bandas Horizontales
    let band_color_1 = Color::new(220, 190, 160);
    let band_color_2 = Color::new(180, 140, 100);
    let band_color_3 = Color::new(140, 100, 70);
    let band_color_4 = Color::new(200, 170, 130);
    
    let y_coord = fragment.normal.y;
    let band_frequency = 8.0;
    let band_position = (y_coord * band_frequency).sin();
    let band_noise = fbm(uv.x * 3.0, uv.y * 15.0, 3);
    let band_with_noise = band_position + band_noise * 0.3;
    
    let base_bands = if band_with_noise > 0.5 {
        band_color_1
    } else if band_with_noise > 0.0 {
        band_color_2
    } else if band_with_noise > -0.5 {
        band_color_3
    } else {
        band_color_4
    };
    
    // CAPA 2: Turbulencia
    let turbulence_noise = fbm(uv.x * 10.0 + uniforms.time * 0.005, uv.y * 20.0, 5);
    let turbulence_light = Color::new(230, 200, 170);
    let turbulence_dark = Color::new(130, 90, 60);
    
    let color_with_turbulence = if turbulence_noise > 0.65 {
        let turb_factor = smoothstep(0.65, 0.75, turbulence_noise);
        mix_color(&base_bands, &turbulence_light, turb_factor * 0.4)
    } else if turbulence_noise < 0.35 {
        let turb_factor = smoothstep(0.35, 0.25, turbulence_noise);
        mix_color(&base_bands, &turbulence_dark, turb_factor * 0.3)
    } else {
        base_bands
    };
    
    // CAPA 3: Gran Mancha Roja
    let red_spot_color = Color::new(200, 100, 80);
    let red_spot_center = Color::new(180, 80, 60);
    
    let spot_center_u = 0.35;
    let spot_center_v = 0.45;
    
    let du = uv.x - spot_center_u;
    let dv = (uv.y - spot_center_v) * 2.0;
    let dist_to_spot = (du * du + dv * dv).sqrt();
    let spot_radius = 0.08;
    
    let color_with_spot = if dist_to_spot < spot_radius {
        let normalized_dist = dist_to_spot / spot_radius;
        let spot_intensity = 1.0 - smoothstep(0.0, 1.0, normalized_dist);
        
        let spot_swirl = fbm(uv.x * 40.0 + uniforms.time * 0.01, uv.y * 40.0, 3);
        
        let spot_color_final = if spot_swirl > 0.5 {
            mix_color(&red_spot_color, &red_spot_center, spot_intensity * 0.6)
        } else {
            mix_color(&red_spot_color, &red_spot_center, spot_intensity * 0.4)
        };
        
        let blend_factor = 1.0 - smoothstep(spot_radius * 0.6, spot_radius, dist_to_spot);
        mix_color(&color_with_turbulence, &spot_color_final, blend_factor)
    } else {
        color_with_turbulence
    };
    
    // CAPA 4: Oscurecimiento Polar
    let latitude = fragment.normal.y.abs();
    
    let pole_darkening = if latitude > 0.7 {
        let darkness_factor = smoothstep(0.7, 0.95, latitude);
        let dark_color = Color::new(100, 70, 50);
        mix_color(&color_with_spot, &dark_color, darkness_factor * 0.4)
    } else {
        color_with_spot
    };
    
    // Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.30;
    let intensity = ambient + diffuse * 0.70;
    
    pole_darkening * intensity
}

// ============================================
// SHADER: SATURNO (Gigante Gaseoso) - 4 CAPAS
// ============================================
fn saturn_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Color Base Crema/Dorado
    let base_cream = Color::new(230, 210, 180);
    let light_cream = Color::new(245, 230, 200);
    let warm_cream = Color::new(210, 185, 150);
    
    let base_variation = fbm(uv.x * 6.0, uv.y * 6.0, 3);
    
    let base_color = if base_variation > 0.6 {
        mix_color(&base_cream, &light_cream, (base_variation - 0.6) * 2.5)
    } else if base_variation < 0.4 {
        mix_color(&base_cream, &warm_cream, (0.4 - base_variation) * 2.5)
    } else {
        base_cream
    };
    
    // CAPA 2: Bandas Horizontales SUTILES
    let band_light = Color::new(245, 225, 190);
    let band_medium = Color::new(220, 200, 170);
    let band_dark = Color::new(200, 175, 145);
    
    let y_coord = fragment.normal.y;
    let band_frequency = 12.0;
    let band_position = (y_coord * band_frequency).sin();
    let band_noise = fbm(uv.x * 2.0, uv.y * 25.0, 2);
    let band_with_noise = band_position + band_noise * 0.15;
    
    let bands_color = if band_with_noise > 0.3 {
        let t = smoothstep(0.3, 0.6, band_with_noise);
        mix_color(&band_medium, &band_light, t)
    } else if band_with_noise < -0.3 {
        let t = smoothstep(-0.3, -0.6, band_with_noise);
        mix_color(&band_medium, &band_dark, t)
    } else {
        band_medium
    };
    
    let color_with_bands = mix_color(&base_color, &bands_color, 0.5);
    
    // CAPA 3: Turbulencia SUTIL
    let turbulence_light = Color::new(250, 230, 195);
    let turbulence_shadow = Color::new(195, 170, 140);
    
    let turbulence_noise = fbm(uv.x * 8.0 + uniforms.time * 0.003, uv.y * 16.0, 4);
    
    let color_with_turbulence = if turbulence_noise > 0.62 {
        let turb_factor = smoothstep(0.62, 0.72, turbulence_noise);
        mix_color(&color_with_bands, &turbulence_light, turb_factor * 0.25)
    } else if turbulence_noise < 0.38 {
        let turb_factor = smoothstep(0.38, 0.28, turbulence_noise);
        mix_color(&color_with_bands, &turbulence_shadow, turb_factor * 0.20)
    } else {
        color_with_bands
    };
    
    // CAPA 4: Hexágono Polar
    let hexagon_color = Color::new(180, 160, 130);
    let latitude = fragment.normal.y;
    
    let color_with_hexagon = if latitude > 0.85 {
        let pole_factor = smoothstep(0.85, 0.95, latitude);
        let hex_noise = fbm(uv.x * 30.0, uv.y * 30.0, 3);
        
        let hex_intensity = if hex_noise > 0.55 {
            pole_factor * 0.3
        } else {
            pole_factor * 0.15
        };
        
        mix_color(&color_with_turbulence, &hexagon_color, hex_intensity)
    } else if latitude < -0.80 {
        let pole_factor = smoothstep(-0.80, -0.92, latitude);
        let dark_pole = Color::new(190, 165, 135);
        mix_color(&color_with_turbulence, &dark_pole, pole_factor * 0.25)
    } else {
        color_with_turbulence
    };
    
    // Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.35;
    let intensity = ambient + diffuse * 0.65;
    
    color_with_hexagon * intensity
}

// ============================================
// SHADER: NEPTUNO (Gigante de Hielo) - 4 CAPAS
// ============================================
fn neptune_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Color Base Azul Intenso
    let base_color = Color::new(62, 84, 232);
    let dark_blue = Color::new(30, 50, 150);
    
    // CAPA 2: Manchas de Tormenta
    let noise_scale = 10.0;
    let storm_noise = fbm(uv.x * noise_scale, uv.y * noise_scale, 4);
    let storm_threshold = 0.6;
    let storm_factor = if storm_noise > storm_threshold {
        smoothstep(storm_threshold, storm_threshold + 0.2, storm_noise)
    } else {
        0.0
    };
    
    let color_with_storms = mix_color(&base_color, &dark_blue, storm_factor * 0.4);
    
    // CAPA 3: Variación de Color con Latitud
    let latitude = fragment.normal.y;
    let latitude_factor = (1.0 - latitude.abs()) * 0.3;
    let lighter_blue = Color::new(100, 120, 255);
    
    let color_with_latitude = mix_color(&color_with_storms, &lighter_blue, latitude_factor);
    
    // CAPA 4: Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.3;
    let intensity = ambient + diffuse * 0.7;
    
    color_with_latitude * intensity
}

// ============================================
// SHADER: LUNA (Satélite de la Tierra) - 3 CAPAS
// ============================================
fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    let uv = get_uv_from_position(&fragment.normal);
    
    // CAPA 1: Color Base Gris
    let moon_gray = Color::new(180, 180, 180);
    let moon_light = Color::new(200, 200, 200);
    let moon_dark = Color::new(140, 140, 140);
    
    let base_noise = fbm(uv.x * 6.0, uv.y * 6.0, 3);
    
    let base_color = if base_noise > 0.6 {
        mix_color(&moon_gray, &moon_light, (base_noise - 0.6) * 2.0)
    } else if base_noise < 0.4 {
        mix_color(&moon_gray, &moon_dark, (0.4 - base_noise) * 2.0)
    } else {
        moon_gray
    };
    
    // CAPA 2: Cráteres Circulares
    let crater_color = Color::new(100, 100, 100);
    let mut color_with_craters = base_color;
    
    for layer in 0..2 {
        let scale = 6.0 + layer as f32 * 4.0;
        
        let grid_x = (uv.x * scale).floor();
        let grid_y = (uv.y * scale).floor();
        
        let local_x = (uv.x * scale) - grid_x;
        let local_y = (uv.y * scale) - grid_y;
        
        let cell_hash = hash_2d(grid_x as i32 * 127 + layer as i32 * 311, 
                                 grid_y as i32 * 257 + layer as i32 * 419);
        
        let crater_probability = 0.35 - layer as f32 * 0.1;
        
        if cell_hash > (1.0 - crater_probability) {
            let offset_hash_x = hash_2d(grid_x as i32 * 73, grid_y as i32 * 151 + layer as i32);
            let offset_hash_y = hash_2d(grid_x as i32 * 179, grid_y as i32 * 283 + layer as i32);
            
            let crater_center_x = 0.3 + offset_hash_x * 0.4;
            let crater_center_y = 0.3 + offset_hash_y * 0.4;
            
            let dx = local_x - crater_center_x;
            let dy = local_y - crater_center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            let crater_radius = 0.18 + cell_hash * 0.15;
            
            if dist < crater_radius {
                let normalized_dist = dist / crater_radius;
                let crater_depth = smoothstep(0.0, 0.8, normalized_dist);
                let inverted_depth = 1.0 - crater_depth;
                color_with_craters = mix_color(&color_with_craters, &crater_color, inverted_depth * 0.5);
            }
        }
    }
    
    // CAPA 3: Mares Lunares
    let mare_color = Color::new(120, 120, 120);
    let mare_noise = fbm(uv.x * 3.0, uv.y * 3.0, 4);
    
    let color_with_maria = if mare_noise > 0.58 {
        let mare_intensity = smoothstep(0.58, 0.70, mare_noise);
        mix_color(&color_with_craters, &mare_color, mare_intensity * 0.4)
    } else {
        color_with_craters
    };
    
    // Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).max(0.0);
    let ambient = 0.15;
    let intensity = ambient + diffuse * 0.85;
    
    color_with_maria * intensity
}

// ============================================
// SHADER: ANILLOS DE SATURNO - 3 CAPAS
// ============================================
fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let normal = fragment.normal.normalize();
    let light_dir = uniforms.light_dir.normalize();
    
    // Calcular distancia desde el centro
    let pos = fragment.position;
    let distance_from_center = (pos.x * pos.x + pos.y * pos.y).sqrt();
    let normalized_dist = (distance_from_center - 400.0) / 400.0;
    
    // CAPA 1: Bandas Concéntricas
    let band1 = Color::new(220, 200, 170);
    let band2 = Color::new(200, 180, 150);
    let band3 = Color::new(180, 160, 130);
    let gap_color = Color::new(50, 50, 50);
    
    let band_pattern = (normalized_dist * 15.0).sin();
    
    let base_color = if normalized_dist > 0.65 && normalized_dist < 0.70 {
        gap_color
    } else if band_pattern > 0.6 {
        band1
    } else if band_pattern > 0.0 {
        band2
    } else {
        band3
    };
    
    // CAPA 2: Variación de Densidad
    let density_noise = simple_noise(normalized_dist * 50.0, 0.0);
    let color_with_density = mix_color(&base_color, &gap_color, density_noise * 0.3);
    
    // CAPA 3: Transparencia Variable
    let alpha = if normalized_dist > 0.65 && normalized_dist < 0.70 {
        0.2
    } else {
        0.7 + density_noise * 0.2
    };
    
    let background = Color::new(10, 5, 20);
    let final_color = mix_color(&background, &color_with_density, alpha);
    
    // Iluminación
    let diffuse = nalgebra_glm::dot(&normal, &light_dir).abs().max(0.3);
    
    Color::new(
        (final_color.r as f32 * diffuse) as u8,
        (final_color.g as f32 * diffuse) as u8,
        (final_color.b as f32 * diffuse) as u8,
    )
}

// ============================================
// FUNCIONES AUXILIARES
// ============================================

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );

    transform_matrix * rotation_matrix
}

pub fn mix_color(color1: &Color, color2: &Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        (color1.r as f32 * (1.0 - t) + color2.r as f32 * t) as u8,
        (color1.g as f32 * (1.0 - t) + color2.g as f32 * t) as u8,
        (color1.b as f32 * (1.0 - t) + color2.b as f32 * t) as u8,
    )
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn simple_noise(x: f32, y: f32) -> f32 {
    let x_int = x.floor() as i32;
    let y_int = y.floor() as i32;
    
    let x_frac = x - x.floor();
    let y_frac = y - y.floor();
    
    let a = hash_2d(x_int, y_int);
    let b = hash_2d(x_int + 1, y_int);
    let c = hash_2d(x_int, y_int + 1);
    let d = hash_2d(x_int + 1, y_int + 1);
    
    let x_smooth = x_frac * x_frac * (3.0 - 2.0 * x_frac);
    let y_smooth = y_frac * y_frac * (3.0 - 2.0 * y_frac);
    
    let ab = a * (1.0 - x_smooth) + b * x_smooth;
    let cd = c * (1.0 - x_smooth) + d * x_smooth;
    
    ab * (1.0 - y_smooth) + cd * y_smooth
}

fn hash_2d(x: i32, y: i32) -> f32 {
    let mut h = x.wrapping_mul(374761393);
    h = h.wrapping_add(y.wrapping_mul(668265263));
    h ^= h >> 13;
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    
    (h as f32 / 4294967296.0 + 0.5).abs()
}

pub fn fbm(x: f32, y: f32, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += simple_noise(x * frequency, y * frequency) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

pub fn get_uv_from_position(position: &Vec3) -> Vec2 {
    let normalized = position.normalize();
    let u = 0.5 + (normalized.z.atan2(normalized.x)) / (2.0 * std::f32::consts::PI);
    let v = 0.5 - (normalized.y.asin()) / std::f32::consts::PI;
    Vec2::new(u, v)
}