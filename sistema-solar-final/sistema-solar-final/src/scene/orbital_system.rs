// =============================================================================
// Sistema Orbital - Gestiona todos los cuerpos celestes
// =============================================================================

use crate::scene::CelestialBody;
use crate::CelestialBodyType;
use nalgebra_glm::{Vec3, vec3};

pub struct OrbitalSystem {
    pub cuerpos: Vec<CelestialBody>,
    pub tiempo_total: f32,
    pub velocidad_tiempo: f32,  // Multiplicador de velocidad temporal
}

impl OrbitalSystem {
    pub fn new() -> Self {
        Self {
            cuerpos: Vec::new(),
            tiempo_total: 0.0,
            velocidad_tiempo: 1.0,
        }
    }
    
    /// Crear sistema solar predefinido
    pub fn crear_sistema_solar() -> Self {
        let mut sistema = Self::new();
        
        // Sol (índice 0)
        sistema.cuerpos.push(CelestialBody::new_sol(
            "Sol",
            vec3(0.0, 0.0, 0.0),
            2.0,
        ));
        
        // Planetas interiores
        
        // Mercurio (índice 1)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Mercurio",
            CelestialBodyType::PlanetaRocoso,
            5.0,    // radio órbita
            0.8,    // velocidad orbital
            0.4,    // radio planeta
            0.0,    // ángulo inicial
        ));
        
        // Venus (índice 2)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Venus",
            CelestialBodyType::PlanetaVolcanico,
            8.0,
            0.6,
            0.6,
            std::f32::consts::PI / 4.0,
        ));
        
        // Tierra (índice 3)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Tierra",
            CelestialBodyType::PlanetaGaseoso,  // Usaremos el shader gaseoso con colores azules
            11.0,
            0.5,
            0.65,
            std::f32::consts::PI / 2.0,
        ));
        
        // Luna de la Tierra (índice 4)
        sistema.cuerpos.push(CelestialBody::new_luna(
            "Luna",
            3,      // índice del padre (Tierra)
            1.5,    // radio órbita alrededor de la Tierra
            2.0,    // velocidad orbital (rápida)
            0.2,    // radio
        ));
        
        // Marte (índice 5)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Marte",
            CelestialBodyType::PlanetaRocoso,
            15.0,
            0.4,
            0.5,
            std::f32::consts::PI,
        ));
        
        // Planetas exteriores
        
        // Júpiter (índice 6)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Júpiter",
            CelestialBodyType::PlanetaGaseoso,
            20.0,
            0.3,
            1.5,
            std::f32::consts::PI * 1.5,
        ));
        
        // Saturno (índice 7)
        sistema.cuerpos.push(CelestialBody::new_planeta(
            "Saturno",
            CelestialBodyType::PlanetaAnillos,
            26.0,
            0.25,
            1.3,
            0.5,
        ));
        
        sistema
    }
    
    /// Actualizar todos los cuerpos celestes
    pub fn actualizar(&mut self, delta_time: f32) {
        self.tiempo_total += delta_time * self.velocidad_tiempo;
        
        // Primero actualizar planetas
        for i in 0..self.cuerpos.len() {
            if !self.cuerpos[i].es_luna {
                self.cuerpos[i].actualizar(delta_time * self.velocidad_tiempo);
            }
        }
        
        // Luego actualizar lunas (necesitan la posición actualizada de sus padres)
        for i in 0..self.cuerpos.len() {
            if self.cuerpos[i].es_luna {
                if let Some(padre_idx) = self.cuerpos[i].planeta_padre {
                    let posicion_padre = self.cuerpos[padre_idx].posicion;
                    self.cuerpos[i].actualizar_centro_orbita(posicion_padre);
                    self.cuerpos[i].actualizar(delta_time * self.velocidad_tiempo);
                }
            }
        }
    }
    
    /// Obtener posición de un cuerpo por nombre
    pub fn get_posicion_por_nombre(&self, nombre: &str) -> Option<Vec3> {
        self.cuerpos
            .iter()
            .find(|c| c.nombre == nombre)
            .map(|c| c.posicion)
    }
    
    /// Obtener índice de un cuerpo por nombre
    pub fn get_indice_por_nombre(&self, nombre: &str) -> Option<usize> {
        self.cuerpos
            .iter()
            .position(|c| c.nombre == nombre)
    }
    
    /// Obtener cuerpo más cercano a una posición
    pub fn get_cuerpo_mas_cercano(&self, posicion: Vec3) -> Option<usize> {
        self.cuerpos
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = nalgebra_glm::distance(&a.posicion, &posicion);
                let dist_b = nalgebra_glm::distance(&b.posicion, &posicion);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(idx, _)| idx)
    }
    
    /// Aumentar velocidad del tiempo
    pub fn acelerar_tiempo(&mut self) {
        self.velocidad_tiempo *= 2.0;
        if self.velocidad_tiempo > 16.0 {
            self.velocidad_tiempo = 16.0;
        }
    }
    
    /// Disminuir velocidad del tiempo
    pub fn desacelerar_tiempo(&mut self) {
        self.velocidad_tiempo /= 2.0;
        if self.velocidad_tiempo < 0.125 {
            self.velocidad_tiempo = 0.125;
        }
    }
    
    /// Resetear velocidad del tiempo
    pub fn velocidad_tiempo_normal(&mut self) {
        self.velocidad_tiempo = 1.0;
    }
}

impl Default for OrbitalSystem {
    fn default() -> Self {
        Self::new()
    }
}
