// =============================================================================
// Sistema Solar Interactivo - Biblioteca Principal
// Autor: Pablo Cabrera - Carné: 231156
// =============================================================================

pub mod render;
pub mod camera;
pub mod scene;
pub mod physics;
pub mod input;
pub mod utils;

// Re-exportar tipos principales para fácil acceso
pub use camera::Camera3D;
pub use scene::{CelestialBody, OrbitalSystem, Spaceship, Skybox};
pub use physics::CollisionSystem;
pub use input::InputState;
pub use render::Renderer;



/// Constantes globales del sistema
pub mod constants {
    /// Velocidad base de la cámara
    pub const CAMERA_SPEED: f32 = 10.0;
    
    /// Sensibilidad del mouse
    pub const MOUSE_SENSITIVITY: f32 = 0.005;
    
    /// Distancia mínima para evitar colisiones
    pub const MIN_DISTANCE_TO_BODY: f32 = 0.5;
    
    /// Escala del sistema (en unidades)
    pub const SYSTEM_SCALE: f32 = 1.0;
    
    /// Campo de visión de la cámara (en grados)
    pub const FOV: f32 = 60.0;
    
    /// Planos de recorte cercano y lejano
    pub const NEAR_PLANE: f32 = 0.1;
    pub const FAR_PLANE: f32 = 1000.0;
}

/// Tipos de cuerpos celestes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CelestialBodyType {
    Sol,
    PlanetaRocoso,
    PlanetaGaseoso,
    PlanetaAnillos,
    PlanetaVolcanico,
    Luna,
}

impl CelestialBodyType {
    pub fn to_shader_id(&self) -> u32 {
        match self {
            Self::Sol => 1,
            Self::PlanetaRocoso => 2,
            Self::PlanetaGaseoso => 3,
            Self::PlanetaAnillos => 4,
            Self::PlanetaVolcanico => 5,
            Self::Luna => 6,
        }
    }
}
