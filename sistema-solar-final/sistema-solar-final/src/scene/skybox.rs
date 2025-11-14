// =============================================================================
// Skybox con Estrellas
// =============================================================================

use nalgebra_glm::{Vec3, vec3};
use rand::Rng;

#[derive(Clone)]
pub struct Star {
    pub posicion: Vec3,
    pub brillo: f32,
    pub tamaño: f32,
    pub color_tint: [f32; 3],  // Ligero tinte de color
}

pub struct Skybox {
    pub estrellas: Vec<Star>,
    pub radio: f32,
}

impl Skybox {
    /// Crear skybox con estrellas distribuidas uniformemente
    pub fn new(cantidad: usize, radio: f32) -> Self {
        let estrellas = Self::generar_estrellas_fibonacci(cantidad, radio);
        
        Self {
            estrellas,
            radio,
        }
    }
    
    /// Distribución de Fibonacci Sphere para uniformidad perfecta
    fn generar_estrellas_fibonacci(cantidad: usize, radio: f32) -> Vec<Star> {
        let mut estrellas = Vec::with_capacity(cantidad);
        let mut rng = rand::thread_rng();
        
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;  // Golden ratio
        
        for i in 0..cantidad {
            // Fibonacci sphere algorithm
            let y = 1.0 - (i as f32 / (cantidad - 1) as f32) * 2.0;
            let radio_xz = (1.0 - y * y).sqrt();
            
            let theta = 2.0 * std::f32::consts::PI * (i as f32 / phi);
            
            let x = theta.cos() * radio_xz;
            let z = theta.sin() * radio_xz;
            
            let posicion = vec3(x, y, z) * radio;
            
            // Brillo aleatorio
            let brillo = rng.gen_range(0.4..1.0);
            
            // Tamaño aleatorio (la mayoría pequeñas, algunas grandes)
            let tamaño = if rng.gen::<f32>() < 0.95 {
                rng.gen_range(0.002..0.005)
            } else {
                rng.gen_range(0.005..0.015)  // Estrellas grandes
            };
            
            // Color: la mayoría blancas, algunas con tinte azul o amarillo
            let color_tint = match rng.gen_range(0..10) {
                0 => [0.8, 0.9, 1.0],   // Azuladas
                1 => [1.0, 0.95, 0.8],  // Amarillentas
                _ => [1.0, 1.0, 1.0],   // Blancas
            };
            
            estrellas.push(Star {
                posicion,
                brillo,
                tamaño,
                color_tint,
            });
        }
        
        estrellas
    }
    
    /// Distribución aleatoria simple (backup)
    #[allow(dead_code)]
    fn generar_estrellas_aleatorias(cantidad: usize, radio: f32) -> Vec<Star> {
        let mut estrellas = Vec::with_capacity(cantidad);
        let mut rng = rand::thread_rng();
        
        for _ in 0..cantidad {
            // Distribución uniforme en esfera
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            
            let x = radio * phi.sin() * theta.cos();
            let y = radio * phi.sin() * theta.sin();
            let z = radio * phi.cos();
            
            let posicion = vec3(x, y, z);
            let brillo = rng.gen_range(0.3..1.0);
            let tamaño = rng.gen_range(0.002..0.008);
            let color_tint = [1.0, 1.0, 1.0];
            
            estrellas.push(Star {
                posicion,
                brillo,
                tamaño,
                color_tint,
            });
        }
        
        estrellas
    }
    
    /// Agregar más estrellas dinámicamente
    pub fn agregar_estrellas(&mut self, cantidad: usize) {
        let nuevas = Self::generar_estrellas_fibonacci(cantidad, self.radio);
        self.estrellas.extend(nuevas);
    }
    
    /// Obtener estrellas visibles desde una posición (frustum culling básico)
    pub fn get_estrellas_visibles(&self, posicion_camara: Vec3, direccion: Vec3) -> Vec<&Star> {
        self.estrellas
            .iter()
            .filter(|star| {
                let dir_star = nalgebra_glm::normalize(&(star.posicion - posicion_camara));
                let dot = nalgebra_glm::dot(&dir_star, &direccion);
                dot > 0.3  // Aproximadamente 70 grados de FOV
            })
            .collect()
    }
}

impl Default for Skybox {
    fn default() -> Self {
        Self::new(500, 100.0)
    }
}
