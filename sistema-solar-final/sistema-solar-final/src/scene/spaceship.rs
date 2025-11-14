// =============================================================================
// Nave Espacial
// =============================================================================

use nalgebra_glm::{Vec3, Mat4, vec3, translate, scale, rotate_y, rotate_x, rotate_z};
use crate::camera::Camera3D;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpaceshipVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub struct Spaceship {
    pub vertices: Vec<SpaceshipVertex>,
    pub indices: Vec<u32>,
    pub posicion: Vec3,
    pub rotacion: Vec3,  // Euler angles (pitch, yaw, roll)
    pub escala: f32,
    pub offset_camara: Vec3,  // Offset respecto a la cámara
}

impl Spaceship {
    /// Cargar nave desde archivo OBJ
    pub fn cargar_desde_obj(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut vertices_pos: Vec<Vec3> = Vec::new();
        let mut normales: Vec<Vec3> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertices_finales: Vec<SpaceshipVertex> = Vec::new();
        
        // Parsear archivo OBJ
        for line in reader.lines() {
            let line = line?;
            let partes: Vec<&str> = line.split_whitespace().collect();
            
            if partes.is_empty() {
                continue;
            }
            
            match partes[0] {
                "v" => {
                    // Vértice
                    if partes.len() >= 4 {
                        let x: f32 = partes[1].parse().unwrap_or(0.0);
                        let y: f32 = partes[2].parse().unwrap_or(0.0);
                        let z: f32 = partes[3].parse().unwrap_or(0.0);
                        vertices_pos.push(vec3(x, y, z));
                    }
                }
                "vn" => {
                    // Normal
                    if partes.len() >= 4 {
                        let x: f32 = partes[1].parse().unwrap_or(0.0);
                        let y: f32 = partes[2].parse().unwrap_or(0.0);
                        let z: f32 = partes[3].parse().unwrap_or(0.0);
                        normales.push(vec3(x, y, z));
                    }
                }
                "f" => {
                    // Cara
                    if partes.len() >= 4 {
                        // Parsear índices (formato: v/vt/vn o v//vn o v)
                        let mut indices_cara: Vec<u32> = Vec::new();
                        
                        for i in 1..partes.len() {
                            let indices_str: Vec<&str> = partes[i].split('/').collect();
                            if let Ok(v_idx) = indices_str[0].parse::<u32>() {
                                indices_cara.push(v_idx - 1);  // OBJ indices son 1-based
                            }
                        }
                        
                        // Triangular cara (si tiene más de 3 vértices)
                        if indices_cara.len() >= 3 {
                            for i in 1..(indices_cara.len() - 1) {
                                indices.push(indices_cara[0]);
                                indices.push(indices_cara[i]);
                                indices.push(indices_cara[i + 1]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Si no hay normales, calcular normales planas
        if normales.is_empty() {
            normales = vec![vec3(0.0, 1.0, 0.0); vertices_pos.len()];
        }
        
        // Construir vértices finales
        for (i, pos) in vertices_pos.iter().enumerate() {
            let normal = if i < normales.len() {
                normales[i]
            } else {
                vec3(0.0, 1.0, 0.0)
            };
            
            vertices_finales.push(SpaceshipVertex {
                position: [pos.x, pos.y, pos.z],
                normal: [normal.x, normal.y, normal.z],
            });
        }
        
        Ok(Self {
            vertices: vertices_finales,
            indices,
            posicion: Vec3::zeros(),
            rotacion: Vec3::zeros(),
            escala: 1.0,
            offset_camara: vec3(0.0, -1.0, -3.0),  // Abajo y atrás de la cámara
        })
    }
    
    /// Actualizar posición para seguir la cámara
    pub fn seguir_camara(&mut self, camera: &Camera3D) {
        // Calcular vectores de la cámara
        let adelante = camera.direccion;
        let derecha = nalgebra_glm::normalize(&nalgebra_glm::cross(&adelante, &camera.up));
        let arriba = camera.up;
        
        // Calcular posición usando el offset
        self.posicion = camera.posicion
            + adelante * self.offset_camara.z
            + arriba * self.offset_camara.y
            + derecha * self.offset_camara.x;
        
        // Copiar rotación de la cámara
        self.rotacion.y = camera.yaw;
        self.rotacion.x = camera.pitch;
    }
    
    /// Obtener matriz de transformación
    pub fn get_transform_matrix(&self) -> Mat4 {
        let mut transform = Mat4::identity();
        
        // Trasladar
        transform = translate(&transform, &self.posicion);
        
        // Rotar (orden: Y -> X -> Z)
        transform = rotate_y(&transform, self.rotacion.y);
        transform = rotate_x(&transform, self.rotacion.x);
        transform = rotate_z(&transform, self.rotacion.z);
        
        // Escalar
        let scale_vec = vec3(self.escala, self.escala, self.escala);
        transform = scale(&transform, &scale_vec);
        
        transform
    }
    
    /// Cambiar offset de la cámara
    pub fn set_offset(&mut self, offset: Vec3) {
        self.offset_camara = offset;
    }
    
    /// Cambiar escala
    pub fn set_escala(&mut self, escala: f32) {
        self.escala = escala;
    }
}
