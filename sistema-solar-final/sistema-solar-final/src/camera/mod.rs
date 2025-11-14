// =============================================================================
// Cámara 3D Completa
// =============================================================================

use nalgebra_glm::{Vec3, Mat4, vec3, look_at, perspective};
use crate::constants::*;

pub struct Camera3D {
    pub posicion: Vec3,
    pub direccion: Vec3,
    pub up: Vec3,
    
    // Ángulos de rotación
    pub yaw: f32,    // Rotación horizontal (izquierda/derecha)
    pub pitch: f32,  // Rotación vertical (arriba/abajo)
    
    // Parámetros de proyección
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    
    // Velocidad
    pub velocidad_movimiento: f32,
    pub velocidad_rotacion: f32,
}

impl Camera3D {
    pub fn new(posicion: Vec3, aspect_ratio: f32) -> Self {
        Self {
            posicion,
            direccion: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0_f32.to_radians(),  // Mirando hacia -Z
            pitch: 0.0,
            fov: FOV.to_radians(),
            aspect_ratio,
            near: NEAR_PLANE,
            far: FAR_PLANE,
            velocidad_movimiento: CAMERA_SPEED,
            velocidad_rotacion: MOUSE_SENSITIVITY,
        }
    }
    
    /// Actualizar vectores de dirección basándose en yaw y pitch
    pub fn actualizar_vectores(&mut self) {
        // Calcular nueva dirección
        let dir_x = self.yaw.cos() * self.pitch.cos();
        let dir_y = self.pitch.sin();
        let dir_z = self.yaw.sin() * self.pitch.cos();
        
        self.direccion = nalgebra_glm::normalize(&vec3(dir_x, dir_y, dir_z));
    }
    
    /// Mover la cámara adelante/atrás
    pub fn mover_adelante(&mut self, distancia: f32) {
        self.posicion += self.direccion * distancia;
    }
    
    /// Mover la cámara a los lados
    pub fn mover_derecha(&mut self, distancia: f32) {
        let derecha = nalgebra_glm::cross(&self.direccion, &self.up);
        let derecha_normalizada = nalgebra_glm::normalize(&derecha);
        self.posicion += derecha_normalizada * distancia;
    }
    
    /// Mover la cámara arriba/abajo
    pub fn mover_arriba(&mut self, distancia: f32) {
        self.posicion += self.up * distancia;
    }
    
    /// Rotar la cámara con el mouse
    pub fn rotar(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.velocidad_rotacion;
        self.pitch -= delta_y * self.velocidad_rotacion;
        
        // Limitar pitch para evitar flip
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
        
        self.actualizar_vectores();
    }
    
    /// Obtener matriz de vista
    pub fn get_view_matrix(&self) -> Mat4 {
        look_at(
            &self.posicion,
            &(self.posicion + self.direccion),
            &self.up,
        )
    }
    
    /// Obtener matriz de proyección
    pub fn get_projection_matrix(&self) -> Mat4 {
        perspective(self.aspect_ratio, self.fov, self.near, self.far)
    }
    
    /// Teletransportarse a una posición
    pub fn teleport_to(&mut self, destino: Vec3, mirar_hacia: Vec3) {
        self.posicion = destino;
        let direccion = nalgebra_glm::normalize(&(mirar_hacia - destino));
        
        // Calcular yaw y pitch desde la dirección
        self.yaw = direccion.z.atan2(direccion.x);
        self.pitch = direccion.y.asin();
        
        self.actualizar_vectores();
    }
    
    /// Actualizar aspect ratio (para resize de ventana)
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect_ratio = aspect;
    }
}
