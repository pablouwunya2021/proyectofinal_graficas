// =============================================================================
// Estructuras de Uniformes para Shaders
// =============================================================================

use bytemuck::{Pod, Zeroable};

/// Uniformes para planetas (debe coincidir con el shader WGSL)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct UniformesPlaneta {
    pub tiempo: f32,
    pub tipo_shader: u32,
    pub resolucion: [f32; 2],
    pub pos_planeta: [f32; 2],
    pub escala_planeta: f32,
    pub _padding: f32,
}

impl UniformesPlaneta {
    pub fn new() -> Self {
        Self {
            tiempo: 0.0,
            tipo_shader: 1,
            resolucion: [800.0, 600.0],
            pos_planeta: [0.0, 0.0],
            escala_planeta: 1.0,
            _padding: 0.0,
        }
    }
}

impl Default for UniformesPlaneta {
    fn default() -> Self {
        Self::new()
    }
}

/// Uniformes para la cámara (3D)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct UniformesCamara {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub posicion_camara: [f32; 3],
    pub _padding: f32,
}

impl UniformesCamara {
    pub fn new() -> Self {
        Self {
            view_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            projection_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            posicion_camara: [0.0, 0.0, 0.0],
            _padding: 0.0,
        }
    }
    
    pub fn actualizar_desde_camara(&mut self, camera: &crate::Camera3D) {
        // Convertir Mat4 de nalgebra a array
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        
        for i in 0..4 {
            for j in 0..4 {
                self.view_matrix[i][j] = view[(i, j)];
                self.projection_matrix[i][j] = proj[(i, j)];
            }
        }
        
        self.posicion_camara = [
            camera.posicion.x,
            camera.posicion.y,
            camera.posicion.z,
        ];
    }
}

impl Default for UniformesCamara {
    fn default() -> Self {
        Self::new()
    }
}
