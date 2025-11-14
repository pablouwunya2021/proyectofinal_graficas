// =============================================================================
// Módulo de Renderizado
// =============================================================================

pub mod uniforms;

pub use uniforms::*;

use wgpu;
use winit::window::Window;
use std::sync::Arc;
use crate::scene::{OrbitalSystem, Skybox, Spaceship};
use wgpu::util::DeviceExt;  // ⬅️ AGREGAR ESTA LÍNEA
use crate::camera::Camera3D;

pub struct Renderer {
    pub dispositivo: wgpu::Device,
    pub cola: wgpu::Queue,
    pub superficie: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub tamaño: winit::dpi::PhysicalSize<u32>,
    
    // Buffers para geometría
    pub buffer_vertices: wgpu::Buffer,
    pub buffer_indices: wgpu::Buffer,
    pub cantidad_indices: u32,
    
    // Pipeline de renderizado
    pub pipeline_render: wgpu::RenderPipeline,
    
    // Uniformes
    pub buffer_uniformes: wgpu::Buffer,
    pub grupo_bind_uniformes: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let tamaño = window.inner_size();
        
        // 1. Crear instancia de WGPU
        let instancia = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // 2. Crear superficie
        let superficie = instancia.create_surface(window.clone()).unwrap();
        
        // 3. Solicitar adaptador
        let adaptador = instancia
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&superficie),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // 4. Solicitar dispositivo y cola
        let (dispositivo, cola) = adaptador
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        
        // 5. Configurar superficie
        let capacidades_superficie = superficie.get_capabilities(&adaptador);
        let formato_superficie = capacidades_superficie.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capacidades_superficie.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: formato_superficie,
            width: tamaño.width,
            height: tamaño.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capacidades_superficie.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        superficie.configure(&dispositivo, &config);
        
        // 6. Generar geometría de esfera
        let (vertices, indices) = crate::utils::generar_esfera(1.0, 32, 16);
        
        // Convertir a formato plano
        let vertices_planos: Vec<f32> = vertices
            .iter()
            .flat_map(|v| {
                let mut data = Vec::new();
                data.extend_from_slice(&v.position);
                data.extend_from_slice(&v.normal);
                data
            })
            .collect();
        
        // 7. Crear buffers
        let buffer_vertices = dispositivo.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Buffer de Vértices"),
                contents: bytemuck::cast_slice(&vertices_planos),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        
        let buffer_indices = dispositivo.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Buffer de Índices"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        let cantidad_indices = indices.len() as u32;
        
        // 8. Crear uniformes
        let datos_uniformes = UniformesPlaneta::new();
        let buffer_uniformes = dispositivo.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Buffer de Uniformes"),
                contents: bytemuck::cast_slice(&[datos_uniformes]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        // 9. Crear bind group layout
        let bind_group_layout = dispositivo.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Bind Group Layout"),
            }
        );
        
        let grupo_bind_uniformes = dispositivo.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_uniformes.as_entire_binding(),
                }],
                label: Some("Bind Group de Uniformes"),
            }
        );
        
        // 10. Cargar shader
        let shader_module = dispositivo.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Principal"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        
        // 11. Crear pipeline layout
        let pipeline_layout = dispositivo.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            }
        );
        
        // 12. Crear render pipeline
        let pipeline_render = dispositivo.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vertex_principal",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: 24,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    }],
                    
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fragment_principal",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );
        
        Self {
            dispositivo,
            cola,
            superficie,
            config,
            tamaño,
            buffer_vertices,
            buffer_indices,
            cantidad_indices,
            pipeline_render,
            buffer_uniformes,
            grupo_bind_uniformes,
        }
    }
    
    pub fn redimensionar(&mut self, nuevo_tamaño: winit::dpi::PhysicalSize<u32>) {
        if nuevo_tamaño.width > 0 && nuevo_tamaño.height > 0 {
            self.tamaño = nuevo_tamaño;
            self.config.width = nuevo_tamaño.width;
            self.config.height = nuevo_tamaño.height;
            self.superficie.configure(&self.dispositivo, &self.config);
        }
    }
    
    pub fn renderizar(
        &mut self,
        sistema: &OrbitalSystem,
        skybox: &Skybox,
        _spaceship: Option<&Spaceship>,
        _camera: &Camera3D,
        tiempo: f32,
    ) -> Result<(), wgpu::SurfaceError> {
        let salida = self.superficie.get_current_texture()?;
        let vista = salida
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut codificador = self
            .dispositivo
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Codificador de Comandos de Render"),
            });
        
        {
            let mut pase_render = codificador.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Pase de Renderizado Principal"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &vista,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.02,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            pase_render.set_pipeline(&self.pipeline_render);
            pase_render.set_vertex_buffer(0, self.buffer_vertices.slice(..));
            pase_render.set_index_buffer(self.buffer_indices.slice(..), wgpu::IndexFormat::Uint32);
            
            // 1. Renderizar estrellas del skybox
            for estrella in skybox.estrellas.iter().take(200) {
                let mut uniformes_estrella = UniformesPlaneta::new();
                uniformes_estrella.tiempo = tiempo;
                uniformes_estrella.pos_planeta = [estrella.posicion.x * 0.01, estrella.posicion.y * 0.01];
                uniformes_estrella.escala_planeta = estrella.tamaño;
                uniformes_estrella.tipo_shader = 7;
                
                self.cola.write_buffer(
                    &self.buffer_uniformes,
                    0,
                    bytemuck::cast_slice(&[uniformes_estrella]),
                );
                pase_render.set_bind_group(0, &self.grupo_bind_uniformes, &[]);
                pase_render.draw_indexed(0..self.cantidad_indices, 0, 0..1);
            }
            
            // 2. Renderizar planetas
            for cuerpo in sistema.cuerpos.iter() {
                let mut uniformes_planeta = UniformesPlaneta::new();
                uniformes_planeta.tiempo = tiempo;
                uniformes_planeta.pos_planeta = [
                    cuerpo.posicion.x * 0.03,
                    cuerpo.posicion.z * 0.03,
                ];
                uniformes_planeta.escala_planeta = cuerpo.escala_visual * 0.5;
                uniformes_planeta.tipo_shader = cuerpo.tipo_shader;
                uniformes_planeta.resolucion = [
                    self.tamaño.width as f32,
                    self.tamaño.height as f32,
                ];
                
                self.cola.write_buffer(
                    &self.buffer_uniformes,
                    0,
                    bytemuck::cast_slice(&[uniformes_planeta]),
                );
                pase_render.set_bind_group(0, &self.grupo_bind_uniformes, &[]);
                pase_render.draw_indexed(0..self.cantidad_indices, 0, 0..1);
            }
        }
        
        self.cola.submit(std::iter::once(codificador.finish()));
        salida.present();
        
        Ok(())
    }
}