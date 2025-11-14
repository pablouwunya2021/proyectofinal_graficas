// =============================================================================
// Sistema de Entrada (Teclado y Mouse)
// =============================================================================

use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use std::collections::HashSet;

pub struct InputState {
    pub teclas_presionadas: HashSet<KeyCode>,
    pub mouse_presionado: bool,
    pub posicion_mouse: Option<(f64, f64)>,
    pub delta_mouse: (f32, f32),
}

impl InputState {
    pub fn new() -> Self {
        Self {
            teclas_presionadas: HashSet::new(),
            mouse_presionado: false,
            posicion_mouse: None,
            delta_mouse: (0.0, 0.0),
        }
    }
    
    pub fn procesar_teclado(&mut self, evento: &KeyEvent) {
        if let PhysicalKey::Code(keycode) = evento.physical_key {
            match evento.state {
                ElementState::Pressed => {
                    self.teclas_presionadas.insert(keycode);
                }
                ElementState::Released => {
                    self.teclas_presionadas.remove(&keycode);
                }
            }
        }
    }
    
    pub fn procesar_mouse_click(&mut self, presionado: bool) {
        self.mouse_presionado = presionado;
        if !presionado {
            // Resetear delta al soltar
            self.delta_mouse = (0.0, 0.0);
        }
    }
    
    pub fn procesar_movimiento_mouse(&mut self, x: f64, y: f64) {
        if let Some((last_x, last_y)) = self.posicion_mouse {
            if self.mouse_presionado {
                self.delta_mouse = (
                    (x - last_x) as f32,
                    (y - last_y) as f32,
                );
            }
        }
        self.posicion_mouse = Some((x, y));
    }
    
    pub fn limpiar_delta(&mut self) {
        self.delta_mouse = (0.0, 0.0);
    }
    
    // Helpers para verificar teclas
    pub fn esta_presionada(&self, keycode: KeyCode) -> bool {
        self.teclas_presionadas.contains(&keycode)
    }
    
    pub fn w_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyW)
    }
    
    pub fn s_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyS)
    }
    
    pub fn a_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyA)
    }
    
    pub fn d_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyD)
    }
    
    pub fn q_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyQ)
    }
    
    pub fn e_presionada(&self) -> bool {
        self.esta_presionada(KeyCode::KeyE)
    }
    
    pub fn numero_presionado(&self) -> Option<usize> {
        for (i, keycode) in [
            KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
            KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
            KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
        ].iter().enumerate() {
            if self.esta_presionada(*keycode) {
                return Some(i);
            }
        }
        None
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
