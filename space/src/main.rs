use macroquad::prelude::*;
use std::f32::consts::PI;

// Estructura para representar un cuerpo celeste
struct CelestialBody {
    identifier: &'static str,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    size: f32,
    base_color: Color,
    orbital_distance: f32,
    orbital_phase: f32,
    rotation_speed: f32,
    ring_tint: Option<Color>,
    show_features: bool,
}

impl CelestialBody {
    fn create(
        identifier: &'static str,
        orbital_distance: f32,
        rotation_speed: f32,
        size: f32,
        base_color: Color,
    ) -> Self {
        CelestialBody {
            identifier,
            position_x: orbital_distance,
            position_y: 0.0,
            position_z: 0.0,
            size,
            base_color,
            orbital_distance,
            orbital_phase: 0.0,
            rotation_speed,
            ring_tint: None,
            show_features: false,
        }
    }

    fn add_ring_system(mut self, ring_tint: Color) -> Self {
        self.ring_tint = Some(ring_tint);
        self
    }

    fn add_surface_features(mut self) -> Self {
        self.show_features = true;
        self
    }

    fn advance_time(&mut self, delta_time: f32) {
        self.orbital_phase += self.rotation_speed * delta_time;
        self.position_x = self.orbital_distance * self.orbital_phase.cos();
        self.position_z = self.orbital_distance * self.orbital_phase.sin();
    }

    fn render(
        &self,
        camera_offset_x: f32,
        camera_offset_y: f32,
        sun_pos_x: f32,
        sun_pos_y: f32,
        camera_rotation: f32,
    ) {
        // Aplicar rotación de cámara
        let cos_rot = camera_rotation.cos();
        let sin_rot = camera_rotation.sin();
        
        let rotated_x = self.position_x * cos_rot - self.position_z * sin_rot;
        let rotated_z = self.position_x * sin_rot + self.position_z * cos_rot;
        
        // Proyección simple con perspectiva
        let depth_factor = 1.0 / (1.0 + rotated_z * 0.001);
        let screen_x = camera_offset_x + rotated_x * depth_factor;
        let screen_y = camera_offset_y + self.position_y * depth_factor;
        let visual_size = self.size * depth_factor;

        // Calcular iluminación basada en la posición del sol
        let light_vector_x = sun_pos_x - screen_x;
        let light_vector_y = sun_pos_y - screen_y;
        let distance_to_sun = (light_vector_x * light_vector_x + light_vector_y * light_vector_y).sqrt();
        
        // Factor de iluminación (más cerca del sol = más brillante)
        let illumination = (1.0 - (distance_to_sun / 500.0).min(1.0)) * 0.7 + 0.3;
        
        // Calcular lado iluminado vs lado oscuro
        let angle_to_sun = light_vector_y.atan2(light_vector_x);
        
        // Dibujar órbita con profundidad
        let orbit_alpha = 0.15 * depth_factor;
        draw_circle_lines(
            camera_offset_x,
            camera_offset_y,
            self.orbital_distance * depth_factor,
            1.0,
            Color::new(0.4, 0.4, 0.5, orbit_alpha),
        );

        // Dibujar sombra proyectada
        if distance_to_sun > 50.0 {
            let shadow_offset_x = (screen_x - sun_pos_x) * 0.05;
            let shadow_offset_y = (screen_y - sun_pos_y) * 0.05;
            draw_circle(
                screen_x + shadow_offset_x,
                screen_y + shadow_offset_y,
                visual_size * 0.9,
                Color::new(0.0, 0.0, 0.0, 0.3),
            );
        }

        // Dibujar anillos si existen
        if let Some(ring_tint) = self.ring_tint {
            let ring_illumination = illumination * 0.8;
            let lit_ring_color = Color::new(
                ring_tint.r * ring_illumination,
                ring_tint.g * ring_illumination,
                ring_tint.b * ring_illumination,
                ring_tint.a,
            );
            draw_ellipse(
                screen_x,
                screen_y,
                visual_size * 1.9,
                visual_size * 0.4,
                0.0,
                lit_ring_color,
            );
            draw_ellipse_lines(
                screen_x,
                screen_y,
                visual_size * 1.9,
                visual_size * 0.4,
                0.0,
                2.0,
                lit_ring_color,
            );
        }

        // Lado iluminado del planeta
        let lit_color = Color::new(
            self.base_color.r * illumination,
            self.base_color.g * illumination,
            self.base_color.b * illumination,
            self.base_color.a,
        );
        draw_circle(screen_x, screen_y, visual_size, lit_color);

        // Lado oscuro del planeta (sombra)
        let shadow_start_angle = angle_to_sun + PI / 2.0;
        let shadow_end_angle = angle_to_sun - PI / 2.0;
        
        // Dibujar media luna de sombra
        let steps = 20;
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let angle = shadow_start_angle + (shadow_end_angle - shadow_start_angle) * t;
            let next_angle = shadow_start_angle + (shadow_end_angle - shadow_start_angle) * ((i + 1) as f32 / steps as f32);
            
            let x1 = screen_x + angle.cos() * visual_size;
            let y1 = screen_y + angle.sin() * visual_size;
            let x2 = screen_x + next_angle.cos() * visual_size;
            let y2 = screen_y + next_angle.sin() * visual_size;
            
            draw_triangle(
                vec2(screen_x, screen_y),
                vec2(x1, y1),
                vec2(x2, y2),
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
        }

        // Características de superficie
        if self.show_features {
            match self.identifier {

                "Mars" => {
                    // Cráteres de impacto
                    let crater_color = Color::new(0.6 * illumination, 0.1 * illumination, 0.0, 1.0);
                    draw_circle(
                        screen_x - visual_size * 0.3,
                        screen_y - visual_size * 0.2,
                        visual_size * 0.15,
                        crater_color,
                    );
                    draw_circle(
                        screen_x + visual_size * 0.2,
                        screen_y + visual_size * 0.3,
                        visual_size * 0.12,
                        crater_color,
                    );
                }
                _ => {}
            }
        }

        // Etiqueta del cuerpo celeste
        draw_text(
            self.identifier,
            screen_x - visual_size,
            screen_y - visual_size - 18.0,
            16.0,
            WHITE,
        );
    }
}

// Estructura para las estrellas del fondo
struct Star {
    x: f32,
    y: f32,
    brightness: f32,
    twinkle_phase: f32,
}

impl Star {
    fn generate_random() -> Self {
        Star {
            x: rand::gen_range(0.0, 1.0),
            y: rand::gen_range(0.0, 1.0),
            brightness: rand::gen_range(0.3, 1.0),
            twinkle_phase: rand::gen_range(0.0, PI * 2.0),
        }
    }

    fn render(&self, screen_width: f32, screen_height: f32, time: f32) {
        let x = self.x * screen_width;
        let y = self.y * screen_height;
        let twinkle = (self.twinkle_phase + time * 2.0).sin() * 0.3 + 0.7;
        let alpha = self.brightness * twinkle;
        draw_circle(x, y, 1.0, Color::new(1.0, 1.0, 1.0, alpha));
    }
}

#[macroquad::main("Planetary System Explorer")]
async fn main() {
    // Inicializar planetas
    let mut celestial_objects = vec![
        CelestialBody::create("Mercurio", 80.0, 0.5, 5.0, Color::new(0.6, 0.6, 0.6, 1.0)),
        CelestialBody::create("Venus", 120.0, 0.3, 8.0, Color::new(0.9, 0.8, 0.2, 1.0)),
        CelestialBody::create("tierra", 160.0, 0.2, 9.0, Color::new(0.1, 0.4, 0.9, 1.0))
            .add_surface_features(),
        CelestialBody::create("Marte", 200.0, 0.15, 6.0, Color::new(0.9, 0.3, 0.1, 1.0))
            .add_surface_features(),
        CelestialBody::create("Jupiter", 260.0, 0.08, 20.0, Color::new(0.9, 0.5, 0.2, 1.0))
            .add_surface_features(),
        CelestialBody::create("Saturno", 320.0, 0.05, 15.0, Color::new(0.9, 0.7, 0.3, 1.0))
            .add_ring_system(Color::new(0.8, 0.7, 0.4, 0.7)),
    ];

    // Generar campo de estrellas
    let background_stars: Vec<Star> = (0..300).map(|_| Star::generate_random()).collect();

    let star_color = Color::new(1.0, 0.9, 0.1, 1.0);
    let mut elapsed_time: f32 = 0.0;

    loop {
        let delta = get_frame_time();
        elapsed_time += delta;

        // Limpiar pantalla con color negro espacial
        clear_background(Color::new(0.0, 0.0, 0.02, 1.0));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Renderizar estrellas de fondo
        for star in &background_stars {
            star.render(screen_width(), screen_height(), elapsed_time);
        }

        // Control de rotación con mouse
        let mouse_position = mouse_position();
        let camera_angle = (mouse_position.0 / screen_width() - 0.5) * PI * 2.0;

        // Renderizar el sol con efecto de resplandor realista
        for i in (0..5).rev() {
            let glow_radius = 18.0 + i as f32 * 4.0;
            let glow_alpha = 0.15 / (i as f32 + 1.0);
            draw_circle(
                center_x,
                center_y,
                glow_radius,
                Color::new(1.0, 0.8, 0.2, glow_alpha),
            );
        }
        draw_circle(center_x, center_y, 14.0, Color::new(1.0, 0.9, 0.3, 1.0));
        draw_circle(center_x, center_y, 12.0, star_color);

        // Actualizar y renderizar planetas
        for celestial_object in &mut celestial_objects {
            celestial_object.advance_time(delta);
            celestial_object.render(center_x, center_y, center_x, center_y, camera_angle);
        }

        // Renderizar interfaz de usuario
        draw_text("SPACE BY PABLOO", 10.0, 25.0, 26.0, WHITE);
        draw_text(
            &format!("Frame Rate: {:.0}", get_fps()),
            10.0,
            55.0,
            18.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
        draw_text(
            "Mueve el mouse para cambiar la perspectiva",
            10.0,
            85.0,
            18.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
        draw_text(
            "Presiona esc para salir",
            screen_width() - 220.0,
            25.0,
            18.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );

        // Salir con tecla ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}