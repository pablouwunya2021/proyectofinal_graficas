// =============================================================================
// Sistema de Colisiones
// =============================================================================

use nalgebra_glm::Vec3;
use crate::scene::OrbitalSystem;

pub struct CollisionSystem;

impl CollisionSystem {
    /// Verificar si hay colisión con algún cuerpo celeste
    pub fn verificar_colision(
        posicion: Vec3,
        radio: f32,
        sistema: &OrbitalSystem,
    ) -> Option<usize> {
        for (i, cuerpo) in sistema.cuerpos.iter().enumerate() {
            let distancia = nalgebra_glm::distance(&posicion, &cuerpo.posicion);
            let radio_minimo = radio + cuerpo.radio + crate::constants::MIN_DISTANCE_TO_BODY;
            
            if distancia < radio_minimo {
                return Some(i);
            }
        }
        None
    }
    
    /// Corregir posición si hay colisión, empujando fuera del cuerpo
    pub fn corregir_posicion(
        posicion_deseada: Vec3,
        radio: f32,
        sistema: &OrbitalSystem,
    ) -> Vec3 {
        if let Some(idx) = Self::verificar_colision(posicion_deseada, radio, sistema) {
            let cuerpo = &sistema.cuerpos[idx];
            let direccion = nalgebra_glm::normalize(&(posicion_deseada - cuerpo.posicion));
            let distancia_segura = cuerpo.radio + radio + crate::constants::MIN_DISTANCE_TO_BODY;
            
            return cuerpo.posicion + direccion * distancia_segura;
        }
        posicion_deseada
    }
    
    /// Verificar colisión entre dos esferas
    pub fn colision_esferas(
        pos1: Vec3,
        radio1: f32,
        pos2: Vec3,
        radio2: f32,
    ) -> bool {
        let distancia = nalgebra_glm::distance(&pos1, &pos2);
        distancia < (radio1 + radio2)
    }
    
    /// Obtener el cuerpo celeste más cercano a una posición
    pub fn get_cuerpo_mas_cercano(
        posicion: Vec3,
        sistema: &OrbitalSystem,
    ) -> Option<(usize, f32)> {
        sistema.cuerpos
            .iter()
            .enumerate()
            .map(|(idx, cuerpo)| {
                let dist = nalgebra_glm::distance(&posicion, &cuerpo.posicion);
                (idx, dist)
            })
            .min_by(|(_, dist_a), (_, dist_b)| {
                dist_a.partial_cmp(dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }
    
    /// Verificar si una posición está dentro de un radio seguro de un cuerpo
    pub fn esta_en_radio_seguro(
        posicion: Vec3,
        cuerpo_pos: Vec3,
        cuerpo_radio: f32,
        margen: f32,
    ) -> bool {
        let distancia = nalgebra_glm::distance(&posicion, &cuerpo_pos);
        distancia >= cuerpo_radio + margen
    }
    
    /// Calcular punto seguro alrededor de un cuerpo celeste
    pub fn calcular_punto_seguro(
        cuerpo_pos: Vec3,
        cuerpo_radio: f32,
        direccion_preferida: Vec3,
        margen: f32,
    ) -> Vec3 {
        let dir_normalizada = nalgebra_glm::normalize(&direccion_preferida);
        let distancia_segura = cuerpo_radio + margen;
        cuerpo_pos + dir_normalizada * distancia_segura
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::vec3;
    
    #[test]
    fn test_colision_esferas() {
        let pos1 = vec3(0.0, 0.0, 0.0);
        let pos2 = vec3(3.0, 0.0, 0.0);
        
        assert!(CollisionSystem::colision_esferas(pos1, 2.0, pos2, 2.0));
        assert!(!CollisionSystem::colision_esferas(pos1, 1.0, pos2, 1.0));
    }
    
    #[test]
    fn test_punto_seguro() {
        let cuerpo_pos = vec3(0.0, 0.0, 0.0);
        let direccion = vec3(1.0, 0.0, 0.0);
        
        let punto = CollisionSystem::calcular_punto_seguro(
            cuerpo_pos,
            5.0,
            direccion,
            2.0,
        );
        
        assert!((punto.x - 7.0).abs() < 0.001);
    }
}
