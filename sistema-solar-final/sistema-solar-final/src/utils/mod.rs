// =============================================================================
// Utilidades
// =============================================================================

pub mod geometry;

pub use geometry::*;

use nalgebra_glm::Vec3;

/// Interpolar suavemente entre dos valores (ease-in-out)
pub fn smooth_step(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Interpolar con aceleración (ease-in)
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Interpolar con desaceleración (ease-out)
pub fn ease_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Interpolar suavemente entre dos Vec3
pub fn lerp_vec3(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

/// Calcular ángulo entre dos vectores
pub fn angle_between(a: &Vec3, b: &Vec3) -> f32 {
    let dot = nalgebra_glm::dot(a, b);
    let mag_a = nalgebra_glm::length(a);
    let mag_b = nalgebra_glm::length(b);
    
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    
    (dot / (mag_a * mag_b)).acos()
}

/// Convertir grados a radianes
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convertir radianes a grados
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}
