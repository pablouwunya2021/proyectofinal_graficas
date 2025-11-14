// =============================================================================
// Sistema Solar Interactivo - Main
// Autor: Pablo Cabrera - Carné: 231156
// =============================================================================

use sistema_solar_pablo_cabrera::*;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
use std::sync::Arc;
use std::time::Instant;
use nalgebra_glm::vec3;
use nalgebra_glm::Vec3;

struct App {
    // Sistemas principales
    camera: Camera3D,
    input: InputState,
    sistema_orbital: OrbitalSystem,
    spaceship: Option<Spaceship>,
    skybox: Skybox,
    
    // Renderizado (TODO: Adaptar tu código actual aquí)
    renderer: Renderer,
    
    // Tiempo
    last_frame: Instant,
    tiempo_inicio: Instant,
    
    // Sistema de warping
    warp_activo: bool,
    warp_tiempo: f32,
    warp_origen: Vec3,
    warp_destino: Vec3,
    warp_duracion: f32,
}

impl App {
    fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        
        let camera = Camera3D::new(vec3(0.0, 15.0, 30.0), aspect);
        let sistema_orbital = OrbitalSystem::crear_sistema_solar();
        
        let spaceship = match Spaceship::cargar_desde_obj("assets/nave.obj") {
            Ok(mut nave) => {
                nave.set_escala(0.3);
                nave.set_offset(vec3(1.0, -0.5, -3.0));
                Some(nave)
            }
            Err(e) => {
                eprintln!("Error cargando nave: {}. Continuando sin nave.", e);
                None
            }
        };
        
        let skybox = Skybox::new(500, 80.0);
        
        println!("Inicializando GPU...");
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        println!("GPU inicializada correctamente");
        
        Self {
            camera,
            input: InputState::new(),
            sistema_orbital,
            spaceship,
            skybox,
            renderer,
            last_frame: Instant::now(),
            tiempo_inicio: Instant::now(),
            warp_activo: false,
            warp_tiempo: 0.0,
            warp_origen: Vec3::zeros(),
            warp_destino: Vec3::zeros(),
            warp_duracion: 1.0,
        }
    }
    
    fn update(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;
        
        // 1. Actualizar sistema orbital
        self.sistema_orbital.actualizar(delta);
        
        // 2. Procesar input y mover cámara
        self.procesar_input(delta);
        
        // 3. Actualizar warping
        self.actualizar_warp(delta);
        
        // 4. Actualizar nave para seguir cámara
        if let Some(ref mut nave) = self.spaceship {
            nave.seguir_camara(&self.camera);
        }
    }
    
    fn procesar_input(&mut self, delta: f32) {
        if self.warp_activo {
            return;  // No procesar input durante warp
        }
        
        let velocidad = self.camera.velocidad_movimiento * delta;
        
        // Movimiento WASD
        if self.input.w_presionada() {
            let nueva_pos = self.camera.posicion + self.camera.direccion * velocidad;
            let pos_corregida = physics::CollisionSystem::corregir_posicion(
                nueva_pos,
                0.5,
                &self.sistema_orbital,
            );
            self.camera.posicion = pos_corregida;
        }
        
        if self.input.s_presionada() {
            let nueva_pos = self.camera.posicion - self.camera.direccion * velocidad;
            let pos_corregida = physics::CollisionSystem::corregir_posicion(
                nueva_pos,
                0.5,
                &self.sistema_orbital,
            );
            self.camera.posicion = pos_corregida;
        }
        
        if self.input.a_presionada() {
            self.camera.mover_derecha(-velocidad);
        }
        
        if self.input.d_presionada() {
            self.camera.mover_derecha(velocidad);
        }
        
        if self.input.q_presionada() {
            self.camera.mover_arriba(-velocidad);
        }
        
        if self.input.e_presionada() {
            self.camera.mover_arriba(velocidad);
        }
        
        // Mouse (rotación)
        if self.input.mouse_presionado {
            let (dx, dy) = self.input.delta_mouse;
            self.camera.rotar(dx, dy);
        }
        self.input.limpiar_delta();
        
        // Warping con teclas numéricas (1-7)
        if let Some(num) = self.input.numero_presionado() {
            if num < self.sistema_orbital.cuerpos.len() {
                self.iniciar_warp(num);
            }
        }
    }
    
    fn iniciar_warp(&mut self, destino_idx: usize) {
        let cuerpo = &self.sistema_orbital.cuerpos[destino_idx];
        
        // Calcular punto seguro alrededor del planeta
        let distancia_orbital = cuerpo.radio * 3.0 + 5.0;
        let offset = nalgebra_glm::normalize(&vec3(1.0, 0.5, 1.0)) * distancia_orbital;
        
        self.warp_activo = true;
        self.warp_tiempo = 0.0;
        self.warp_origen = self.camera.posicion;
        self.warp_destino = cuerpo.posicion + offset;
        
        println!("Warping a: {}", cuerpo.nombre);
    }
    
    fn actualizar_warp(&mut self, delta: f32) {
        if !self.warp_activo {
            return;
        }
        
        self.warp_tiempo += delta / self.warp_duracion;
        
        if self.warp_tiempo >= 1.0 {
            // Warp completado
            self.camera.posicion = self.warp_destino;
            self.warp_activo = false;
            self.warp_tiempo = 0.0;
            return;
        }
        
        // Interpolación suave (ease-in-out)
        let t = utils::smooth_step(self.warp_tiempo);
        
        self.camera.posicion = utils::lerp_vec3(
            &self.warp_origen,
            &self.warp_destino,
            t,
        );
    }
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let tiempo_transcurrido = self.tiempo_inicio.elapsed().as_secs_f32();
        
        self.renderer.renderizar(
            &self.sistema_orbital,
            &self.skybox,
            self.spaceship.as_ref(),
            &self.camera,
            tiempo_transcurrido,
        )
    }
}
fn main() {
    env_logger::init();
    
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_title("Sistema Solar Interactivo - Pablo Cabrera 231156")
            .with_inner_size(winit::dpi::LogicalSize::new(1200, 800))
            .build(&event_loop)
            .unwrap(),
    );
    
    let mut app = App::new(window.clone());
    
    println!("╔════════════════════════════════════════════════╗");
    println!("║   Sistema Solar Interactivo 3D                 ║");
    println!("║   Pablo Cabrera - Carné: 231156                ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    println!("Controles:");
    println!("  WASD        - Mover cámara (horizontal)");
    println!("  Q / E       - Subir / Bajar");
    println!("  Mouse       - Rotar vista (mantén click izquierdo)");
    println!("  1-7         - Teletransporte a planetas");
    println!("  ESC         - Salir");
    println!();
    println!("Planetas:");
    for (i, cuerpo) in app.sistema_orbital.cuerpos.iter().enumerate() {
        println!("  {} - {}", i + 1, cuerpo.nombre);
    }
    println!();
    
    event_loop
        .run(move |evento, control_flujo| {
            match evento {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        println!("Cerrando aplicación...");
                        control_flujo.exit();
                    }
                    WindowEvent::Resized(tamano_fisico) => {
                        let new_aspect = tamano_fisico.width as f32 / tamano_fisico.height as f32;
                        app.camera.set_aspect_ratio(new_aspect);
                        // TODO: app.renderer.redimensionar(*tamano_fisico);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        app.input.procesar_teclado(event);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        app.input.procesar_movimiento_mouse(position.x, position.y);
                    }
                    WindowEvent::MouseInput {
                        state: mouse_state,
                        button: winit::event::MouseButton::Left,
                        ..
                    } => {
                        app.input.procesar_mouse_click(*mouse_state == ElementState::Pressed);
                    }
                    WindowEvent::RedrawRequested => {
                        app.update();
                        match app.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                // TODO: app.renderer.redimensionar(app.renderer.tamaño)
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                eprintln!("Error: Sin memoria!");
                                control_flujo.exit();
                            }
                            Err(e) => eprintln!("Error de renderizado: {:?}", e),
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
