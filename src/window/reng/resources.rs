pub struct RenderResources2D<UniformType, InstanceType> {
    pub win_size: winit::dpi::PhysicalSize<u32>,
    pub sample_count: u32,
    pub surface_conf: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_bgl: wgpu::BindGroupLayout,
    pub instance_bgl: wgpu::BindGroupLayout,
    pub texture_bgl: wgpu::BindGroupLayout,
    _unif_marker: std::marker::PhantomData<UniformType>,
    _inst_marker: std::marker::PhantomData<InstanceType>,
}

impl<UniformType, InstanceType> RenderResources2D<UniformType, InstanceType> {
    pub fn new(win: &winit::window::Window, sample_count: u32) -> Self {
        let win_size = win.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(win).unwrap() };

        use futures::executor::block_on;
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let adapter_features = adapter.features();

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: adapter_features,
                limits: Default::default(),
            },
            None,
        ))
        .unwrap();

        let surf_caps = surface.get_capabilities(&adapter);
        let surf_fmt = wgpu::TextureFormat::Rgba8UnormSrgb;

        let surface_conf = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surf_fmt,
            width: win_size.width,
            height: win_size.height,
            present_mode: surf_caps.present_modes[0],
            alpha_mode: surf_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_conf);

        let uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(
                        std::num::NonZeroU64::new(std::mem::size_of::<UniformType>() as u64)
                            .unwrap(),
                    ),
                },
            }],
        });

        let instance_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
            label: Some("instances"),
        });

        let texture_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                },
            ],
            label: None,
        });

        let pipeline = {
            let vert_shader = unsafe {
                device.create_shader_module_spirv(&wgpu::include_spirv_raw!(
                    "./shaders/default.vert.spv"
                ))
            };

            let frag_shader = unsafe {
                device.create_shader_module_spirv(&wgpu::include_spirv_raw!(
                    "./shaders/default.frag.spv"
                ))
            };

            let layout = &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&uniform_bgl, &instance_bgl, &texture_bgl],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: &vert_shader,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &frag_shader,
                    entry_point: "main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surf_fmt,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    front_face: wgpu::FrontFace::default(),
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                    strip_index_format: Some(wgpu::IndexFormat::Uint32),
                },
                depth_stencil: None,
                multiview: None,
            })
        };

        let _unif_marker = std::marker::PhantomData::<UniformType>;
        let _inst_marker = std::marker::PhantomData::<InstanceType>;

        Self {
            win_size,
            sample_count,
            surface_conf,
            surface,
            adapter,
            device,
            queue,
            pipeline,
            uniform_bgl,
            instance_bgl,
            texture_bgl,
            _unif_marker,
            _inst_marker,
        }
    }

    pub fn create_encoder(&mut self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&Default::default())
    }

    pub fn generate_frame(&mut self) -> wgpu::SurfaceTexture {
        match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(_) => {
                self.surface.configure(&self.device, &self.surface_conf);
                self.surface.get_current_texture().unwrap()
            }
        }
    }

    pub fn create_texture_from_image(&self, image: &image::RgbaImage) -> wgpu::Texture {
        let dimensions = image.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let text = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("reng_texture"),
            size,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            mip_level_count: 1,
            sample_count: 1,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            view_formats: &[],
            dimension: wgpu::TextureDimension::D2,
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &text,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image.as_raw().as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        text
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = size;
        self.surface_conf.width = size.width;
        self.surface_conf.height = size.height;
        self.surface.configure(&self.device, &self.surface_conf);
    }
}
