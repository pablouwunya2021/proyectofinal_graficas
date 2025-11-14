// =============================================================================
// Utilidades de Geometría
// =============================================================================

use nalgebra_glm::{Vec3, vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }
}

/// Generar una esfera UV
pub fn generar_esfera(radio: f32, slices: u32, stacks: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Generar vértices
    for stack in 0..=stacks {
        let phi = std::f32::consts::PI * stack as f32 / stacks as f32;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        
        for slice in 0..=slices {
            let theta = 2.0 * std::f32::consts::PI * slice as f32 / slices as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            let x = sin_phi * cos_theta;
            let y = cos_phi;
            let z = sin_phi * sin_theta;
            
            let normal = [x, y, z];
            let position = [x * radio, y * radio, z * radio];
            
            vertices.push(Vertex { position, normal });
        }
    }
    
    // Generar índices
    for stack in 0..stacks {
        for slice in 0..slices {
            let first = stack * (slices + 1) + slice;
            let second = first + slices + 1;
            
            indices.push(first);
            indices.push(second);
            indices.push(first + 1);
            
            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }
    
    (vertices, indices)
}

/// Generar un icosaedro (esfera de baja resolución)
pub fn generar_icosaedro(radio: f32) -> (Vec<Vertex>, Vec<u32>) {
    let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
    
    let vertices_pos = vec![
        vec3(-1.0, t, 0.0), vec3(1.0, t, 0.0), vec3(-1.0, -t, 0.0), vec3(1.0, -t, 0.0),
        vec3(0.0, -1.0, t), vec3(0.0, 1.0, t), vec3(0.0, -1.0, -t), vec3(0.0, 1.0, -t),
        vec3(t, 0.0, -1.0), vec3(t, 0.0, 1.0), vec3(-t, 0.0, -1.0), vec3(-t, 0.0, 1.0),
    ];
    
    let mut vertices = Vec::new();
    for pos in vertices_pos {
        let normalized = nalgebra_glm::normalize(&pos);
        let scaled = normalized * radio;
        vertices.push(Vertex {
            position: [scaled.x, scaled.y, scaled.z],
            normal: [normalized.x, normalized.y, normalized.z],
        });
    }
    
    let indices = vec![
        0, 11, 5, 0, 5, 1, 0, 1, 7, 0, 7, 10, 0, 10, 11,
        1, 5, 9, 5, 11, 4, 11, 10, 2, 10, 7, 6, 7, 1, 8,
        3, 9, 4, 3, 4, 2, 3, 2, 6, 3, 6, 8, 3, 8, 9,
        4, 9, 5, 2, 4, 11, 6, 2, 10, 8, 6, 7, 9, 8, 1,
    ];
    
    (vertices, indices)
}

/// Generar un cubo
pub fn generar_cubo(tamaño: f32) -> (Vec<Vertex>, Vec<u32>) {
    let s = tamaño / 2.0;
    
    let vertices = vec![
        // Frontal
        Vertex::new([-s, -s, s], [0.0, 0.0, 1.0]),
        Vertex::new([s, -s, s], [0.0, 0.0, 1.0]),
        Vertex::new([s, s, s], [0.0, 0.0, 1.0]),
        Vertex::new([-s, s, s], [0.0, 0.0, 1.0]),
        // Trasera
        Vertex::new([s, -s, -s], [0.0, 0.0, -1.0]),
        Vertex::new([-s, -s, -s], [0.0, 0.0, -1.0]),
        Vertex::new([-s, s, -s], [0.0, 0.0, -1.0]),
        Vertex::new([s, s, -s], [0.0, 0.0, -1.0]),
        // Superior
        Vertex::new([-s, s, s], [0.0, 1.0, 0.0]),
        Vertex::new([s, s, s], [0.0, 1.0, 0.0]),
        Vertex::new([s, s, -s], [0.0, 1.0, 0.0]),
        Vertex::new([-s, s, -s], [0.0, 1.0, 0.0]),
        // Inferior
        Vertex::new([-s, -s, -s], [0.0, -1.0, 0.0]),
        Vertex::new([s, -s, -s], [0.0, -1.0, 0.0]),
        Vertex::new([s, -s, s], [0.0, -1.0, 0.0]),
        Vertex::new([-s, -s, s], [0.0, -1.0, 0.0]),
        // Derecha
        Vertex::new([s, -s, s], [1.0, 0.0, 0.0]),
        Vertex::new([s, -s, -s], [1.0, 0.0, 0.0]),
        Vertex::new([s, s, -s], [1.0, 0.0, 0.0]),
        Vertex::new([s, s, s], [1.0, 0.0, 0.0]),
        // Izquierda
        Vertex::new([-s, -s, -s], [-1.0, 0.0, 0.0]),
        Vertex::new([-s, -s, s], [-1.0, 0.0, 0.0]),
        Vertex::new([-s, s, s], [-1.0, 0.0, 0.0]),
        Vertex::new([-s, s, -s], [-1.0, 0.0, 0.0]),
    ];
    
    let indices = vec![
        0, 1, 2, 2, 3, 0,       // Frontal
        4, 5, 6, 6, 7, 4,       // Trasera
        8, 9, 10, 10, 11, 8,    // Superior
        12, 13, 14, 14, 15, 12, // Inferior
        16, 17, 18, 18, 19, 16, // Derecha
        20, 21, 22, 22, 23, 20, // Izquierda
    ];
    
    (vertices, indices)
}

/// Generar círculo (para órbitas)
pub fn generar_circulo(radio: f32, segmentos: u32) -> Vec<Vec3> {
    let mut puntos = Vec::with_capacity(segmentos as usize + 1);
    
    for i in 0..=segmentos {
        let angulo = (i as f32 / segmentos as f32) * std::f32::consts::TAU;
        let x = radio * angulo.cos();
        let z = radio * angulo.sin();
        puntos.push(vec3(x, 0.0, z));
    }
    
    puntos
}
