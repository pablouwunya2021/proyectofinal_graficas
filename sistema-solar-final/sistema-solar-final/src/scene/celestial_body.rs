// =============================================================================
// Cuerpo Celeste (Planetas, Soles, Lunas)
// =============================================================================

use nalgebra_glm::{Vec3, vec3};
use crate::CelestialBodyType;

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub nombre: String,
    pub tipo: CelestialBodyType,
    
    // Posición y tamaño
    pub posicion: Vec3,
    pub radio: f32,
    pub escala_visual: f32,  // Escala para renderizado
    
    // Color/shader
    pub tipo_shader: u32,
    
    // Parámetros orbitales (para planetas)
    pub orbita_centro: Vec3,      // Centro de la órbita (usualmente el sol)
    pub orbita_radio: f32,         // Radio de la órbita
    pub orbita_velocidad: f32,     // Velocidad angular (rad/s)
    pub orbita_inclinacion: f32,   // Inclinación del plano orbital
    pub angulo_orbital: f32,       // Ángulo actual en la órbita
    
    // Rotación propia
    pub rotacion_velocidad: f32,   // Velocidad de rotación sobre su eje
    pub angulo_rotacion: f32,      // Ángulo de rotación actual
    
    // Para lunas (órbita relativa a planeta padre)
    pub es_luna: bool,
    pub planeta_padre: Option<usize>,  // Índice del planeta padre
}

impl CelestialBody {
    /// Crear un sol (sin órbita)
    pub fn new_sol(nombre: &str, posicion: Vec3, radio: f32) -> Self {
        Self {
            nombre: nombre.to_string(),
            tipo: CelestialBodyType::Sol,
            posicion,
            radio,
            escala_visual: radio,
            tipo_shader: 1,
            orbita_centro: Vec3::zeros(),
            orbita_radio: 0.0,
            orbita_velocidad: 0.0,
            orbita_inclinacion: 0.0,
            angulo_orbital: 0.0,
            rotacion_velocidad: 0.2,
            angulo_rotacion: 0.0,
            es_luna: false,
            planeta_padre: None,
        }
    }
    
    /// Crear un planeta que orbita
    pub fn new_planeta(
        nombre: &str,
        tipo: CelestialBodyType,
        orbita_radio: f32,
        orbita_velocidad: f32,
        radio: f32,
        angulo_inicial: f32,
    ) -> Self {
        Self {
            nombre: nombre.to_string(),
            tipo,
            posicion: Vec3::zeros(),  // Se calculará en update
            radio,
            escala_visual: radio,
            tipo_shader: tipo.to_shader_id(),
            orbita_centro: Vec3::zeros(),
            orbita_radio,
            orbita_velocidad,
            orbita_inclinacion: 0.0,
            angulo_orbital: angulo_inicial,
            rotacion_velocidad: 0.5,
            angulo_rotacion: 0.0,
            es_luna: false,
            planeta_padre: None,
        }
    }
    
    /// Crear una luna que orbita un planeta
    pub fn new_luna(
        nombre: &str,
        planeta_padre_idx: usize,
        orbita_radio: f32,
        orbita_velocidad: f32,
        radio: f32,
    ) -> Self {
        Self {
            nombre: nombre.to_string(),
            tipo: CelestialBodyType::Luna,
            posicion: Vec3::zeros(),
            radio,
            escala_visual: radio,
            tipo_shader: 6,
            orbita_centro: Vec3::zeros(),  // Se actualizará con posición del padre
            orbita_radio,
            orbita_velocidad,
            orbita_inclinacion: 0.1,  // Pequeña inclinación
            angulo_orbital: 0.0,
            rotacion_velocidad: 1.0,
            angulo_rotacion: 0.0,
            es_luna: true,
            planeta_padre: Some(planeta_padre_idx),
        }
    }
    
    /// Actualizar posición orbital
    pub fn actualizar(&mut self, delta_time: f32) {
        // Actualizar ángulo orbital
        self.angulo_orbital += self.orbita_velocidad * delta_time;
        
        // Mantener entre 0 y 2π
        if self.angulo_orbital > std::f32::consts::TAU {
            self.angulo_orbital -= std::f32::consts::TAU;
        }
        
        // Calcular nueva posición en órbita
        let x = self.orbita_centro.x + self.orbita_radio * self.angulo_orbital.cos();
        let z = self.orbita_centro.z + self.orbita_radio * self.angulo_orbital.sin();
        let y = self.orbita_centro.y + self.orbita_radio * self.orbita_inclinacion * self.angulo_orbital.sin();
        
        self.posicion = vec3(x, y, z);
        
        // Actualizar rotación
        self.angulo_rotacion += self.rotacion_velocidad * delta_time;
        if self.angulo_rotacion > std::f32::consts::TAU {
            self.angulo_rotacion -= std::f32::consts::TAU;
        }
    }
    
    /// Actualizar centro de órbita (para lunas)
    pub fn actualizar_centro_orbita(&mut self, nuevo_centro: Vec3) {
        self.orbita_centro = nuevo_centro;
    }
    
    /// Obtener puntos de la órbita para renderizado
    pub fn generar_puntos_orbita(&self, segmentos: usize) -> Vec<Vec3> {
        let mut puntos = Vec::with_capacity(segmentos + 1);
        
        for i in 0..=segmentos {
            let angulo = (i as f32 / segmentos as f32) * std::f32::consts::TAU;
            let x = self.orbita_centro.x + self.orbita_radio * angulo.cos();
            let z = self.orbita_centro.z + self.orbita_radio * angulo.sin();
            let y = self.orbita_centro.y + self.orbita_radio * self.orbita_inclinacion * angulo.sin();
            
            puntos.push(vec3(x, y, z));
        }
        
        puntos
    }
}
