use crate::{
    color,
    geometry::{Plane, Point, Sphere},
    light::{DirectionalLight, PointLight},
    raytracer::Pathtracer,
    renderer::texture,
};
use nalgebra::Vector3;
use wgpu::util::DeviceExt;
use winit::{
    event::{VirtualKeyCode, WindowEvent},
    window::Window,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
    ];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Vertex::ATTRIBS,
        }
    }
}

// Cria um retângulo que ocupa toda a tela
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 2, 1, 1, 2, 3];

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_indices: u32,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    pathtracer: crate::raytracer::Pathtracer,
    last_update: std::time::Instant,
    mouse_pressed: bool,
    mouse_position: winit::dpi::PhysicalPosition<f64>,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // Create surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // Create adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            include_bytes!("../../assets/texture.png"),
            "diffuse_texture",
        )
        .unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let shader = if cfg!(fragment_pathtracer) {
            device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"))
        } else {
            device.create_shader_module(wgpu::include_wgsl!("../image_shader.wgsl"))
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        let pathtracer = Pathtracer::new(size.width, size.height);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            pathtracer,
            last_update: std::time::Instant::now(),
            mouse_pressed: false,
            mouse_position: Default::default(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.pathtracer.resize(new_size.width, new_size.height);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(winit::event::VirtualKeyCode::Space) => {
                    self.pathtracer.render();
                    true
                }
                Some(VirtualKeyCode::W) => {
                    if input.state == winit::event::ElementState::Pressed {
                        self.pathtracer.camera_mut().move_forward(true);
                    } else if input.state == winit::event::ElementState::Released {
                        self.pathtracer.camera_mut().move_forward(false);
                    };
                    true
                }
                Some(VirtualKeyCode::S) => {
                    if input.state == winit::event::ElementState::Pressed {
                        self.pathtracer.camera_mut().move_backward(true);
                    } else if input.state == winit::event::ElementState::Released {
                        self.pathtracer.camera_mut().move_backward(false);
                    };
                    true
                }
                Some(VirtualKeyCode::A) => {
                    if input.state == winit::event::ElementState::Pressed {
                        self.pathtracer.camera_mut().move_left(true);
                    } else if input.state == winit::event::ElementState::Released {
                        self.pathtracer.camera_mut().move_left(false);
                    };
                    true
                }
                Some(VirtualKeyCode::D) => {
                    if input.state == winit::event::ElementState::Pressed {
                        self.pathtracer.camera_mut().move_right(true);
                    } else if input.state == winit::event::ElementState::Released {
                        self.pathtracer.camera_mut().move_right(false);
                    };
                    true
                }
                _ => false,
            },
            WindowEvent::MouseInput {
                state,
                button: winit::event::MouseButton::Left,
                ..
            } => {
                if *state == winit::event::ElementState::Pressed {
                    self.mouse_pressed = true;
                } else if *state == winit::event::ElementState::Released {
                    self.mouse_pressed = false;
                };
                true
            }
            WindowEvent::CursorMoved { device_id: _, position, ..} => {
                if self.mouse_pressed {
                    let delta_x = (self.mouse_position.x - position.x) as f32;
                    let delta_y = (self.mouse_position.y - position.y) as f32;
                    self.pathtracer.camera_mut().rotate(delta_x, delta_y);
                    // self.window.set_cursor_position(winit::dpi::PhysicalPosition::new(self.size.width as i32 / 2, self.size.height as i32 / 2)).unwrap();
                    // self.mouse_position = winit::dpi::PhysicalPosition::new(self.size.width as f64 / 2.0f64, self.size.height as f64 / 2.0f64);
                }
                self.mouse_position = *position;
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let delta_time = (now - self.last_update).as_secs_f32();
        self.last_update = now;
        self.pathtracer.camera_mut().update(delta_time);
        self.pathtracer.render();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        // Update the image data

        let image = self.pathtracer.present();
        let new_texture = texture::Texture::from_image(
            &self.device,
            &self.queue,
            &image,
            Some("diffuse_texture"),
        )
        .unwrap();
        self.diffuse_texture = new_texture;
        let texture_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            // This should match the filterable field of the
                            // corresponding Texture entry above.
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        self.diffuse_bind_group = diffuse_bind_group;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 00, 0..1)
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub(crate) fn init(&mut self) {
        let w = self.pathtracer.world();
        w.add_object(Box::new(
            Sphere::new_with_material(0.0, 2.0, -4.0, 1.5, 
                Box::new(crate::material::Metal::new(color::BLUE, 0.1)))));
        w.add_object(Box::new(
            Sphere::new_with_material(3.0, 0.0, -5.0, 1.0, 
                Box::new(crate::material::Metal::new(color::WHITE, 0.0)))));

        w.add_object(Box::new(Plane::new(
            Point::new(0., -1., 0.),
            Vector3::new(0., 1., 0.),
            Box::new(crate::material::Metal::new(color::WHITE, 0.4)),
        )));

        w.add_object(Box::new(
            Sphere::new_with_material(-3.0, -1.0, -5.0, 1.0, 
                Box::new(crate::material::Emmisive::new(color::ORANGE, 1.3)))));

    }
}
