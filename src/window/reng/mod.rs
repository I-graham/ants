pub mod data;
mod resources;
pub mod utils;

pub use data::*;

pub struct Renderer<UniformType: Copy + PartialEq, InstanceType> {
	resources: resources::RenderResources2D<UniformType, InstanceType>,
	render_data: data::RenderData,
	uniform: Option<UniformType>,
}

impl<UniformType: Copy + PartialEq, InstanceType> Renderer<UniformType, InstanceType> {
	const PREALLOCATED_INSTANCES: usize = 0x10;
	const DEFAULT_CHUNK_SIZE: wgpu::BufferAddress =
		(Self::PREALLOCATED_INSTANCES * std::mem::size_of::<InstanceType>()) as wgpu::BufferAddress;

	pub fn new(win: &winit::window::Window, sample_count: u32) -> Self {
		let resources =
			resources::RenderResources2D::<UniformType, InstanceType>::new(win, sample_count);

		let uniform_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Uniform"),
			size: std::mem::size_of::<UniformType>() as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let uniform_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &resources.uniform_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &uniform_buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		let instance_buffer = resources.device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Instance"),
			size: (Self::PREALLOCATED_INSTANCES * std::mem::size_of::<InstanceType>())
				as wgpu::BufferAddress,
			usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		let instance_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &resources.instance_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &instance_buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		let def_image = image::ImageBuffer::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]));

		let texture = resources.create_texture_from_image(&def_image);

		let sampler = resources.device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("nearest sampler"),
			address_mode_u: wgpu::AddressMode::MirrorRepeat,
			address_mode_v: wgpu::AddressMode::MirrorRepeat,
			address_mode_w: wgpu::AddressMode::MirrorRepeat,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});

		let texture_bg = resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: Some("default texture"),
				layout: &resources.texture_bgl,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: wgpu::BindingResource::TextureView(
							&texture.create_view(&wgpu::TextureViewDescriptor::default()),
						),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: wgpu::BindingResource::Sampler(&sampler),
					},
				],
			});

		let render_data = data::RenderData {
			uniform_buffer,
			uniform_bg,
			instance_buffer,
			instance_bg,
			instance_len: 0,
			instance_cap: Self::PREALLOCATED_INSTANCES,
			encoder: resources.device.create_command_encoder(&Default::default()),
			staging_belt: wgpu::util::StagingBelt::new(Self::DEFAULT_CHUNK_SIZE),
			texture_bg,
			nearest_sampler: sampler,
			current_frame: None,
			cached_buffers: Default::default(),
		};

		Self {
			resources,
			render_data,
			uniform: None,
		}
	}

	pub fn set_uniform(&mut self, uniform: UniformType) {
		if self.uniform != Some(uniform) {
			self.uniform = Some(uniform);
			let unif_data = &[uniform];
			let unif_slice = utils::to_char_slice(unif_data);
			self.render_data
				.staging_belt
				.write_buffer(
					&mut self.render_data.encoder,
					&self.render_data.uniform_buffer,
					0 as wgpu::BufferAddress,
					std::num::NonZeroU64::new(unif_slice.len() as u64).unwrap(),
					&self.resources.device,
				)
				.copy_from_slice(unif_slice);
		}
	}

	pub fn set_instances(&mut self, instances: &[InstanceType]) {
		if self.render_data.instance_cap < instances.len() {
			self.render_data.instance_cap = instances.len();
			self.render_data.instance_buffer =
				self.resources
					.device
					.create_buffer(&wgpu::BufferDescriptor {
						label: Some("Instance"),
						size: (instances.len() * std::mem::size_of::<InstanceType>())
							as wgpu::BufferAddress,
						usage: wgpu::BufferUsages::UNIFORM
							| wgpu::BufferUsages::STORAGE
							| wgpu::BufferUsages::COPY_DST,
						mapped_at_creation: false,
					});

			self.render_data.instance_bg =
				self.resources
					.device
					.create_bind_group(&wgpu::BindGroupDescriptor {
						label: None,
						layout: &self.resources.instance_bgl,
						entries: &[wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
								buffer: &self.render_data.instance_buffer,
								offset: 0,
								size: None,
							}),
						}],
					});
		}

		self.render_data.instance_len = instances.len();

		let inst_slice = utils::to_char_slice(instances);

		self.render_data
			.staging_belt
			.write_buffer(
				&mut self.render_data.encoder,
				&self.render_data.instance_buffer,
				0 as wgpu::BufferAddress,
				std::num::NonZeroU64::new(inst_slice.len() as u64).unwrap(),
				&self.resources.device,
			)
			.copy_from_slice(inst_slice);
	}

	pub fn set_texture(&mut self, texture: &wgpu::Texture) {
		self.render_data.texture_bg =
			self.resources
				.device
				.create_bind_group(&wgpu::BindGroupDescriptor {
					label: None,
					layout: &self.resources.texture_bgl,
					entries: &[
						wgpu::BindGroupEntry {
							binding: 0,
							resource: wgpu::BindingResource::TextureView(
								&texture.create_view(&wgpu::TextureViewDescriptor::default()),
							),
						},
						wgpu::BindGroupEntry {
							binding: 1,
							resource: wgpu::BindingResource::Sampler(
								&self.render_data.nearest_sampler,
							),
						},
					],
				});
	}

	pub fn submit(&mut self) {
		let encoder = std::mem::replace(
			&mut self.render_data.encoder,
			self.resources.create_encoder(),
		);

		self.render_data.staging_belt.finish();

		self.resources.queue.submit(Some(encoder.finish()));

		self.render_data.staging_belt.recall();

		if let Some(frame) = self.render_data.current_frame.take() {
			frame.present();
		}
	}

	pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
		self.resources.resize(dims);
	}

	pub fn clear(&mut self, color: wgpu::Color) {
		let view = &self.get_frame().texture.create_view(&Default::default());
		self.render_data
			.encoder
			.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(color),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
	}

	pub fn cache(&mut self, name: &'static str, instances: &[InstanceType]) {
		use wgpu::util::*;
		let buffer = self
			.resources
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: Some(name),
				contents: utils::to_char_slice(instances),
				usage: wgpu::BufferUsages::STORAGE,
			});

		let bg = self
			.resources
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				label: None,
				layout: &self.resources.instance_bgl,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
						buffer: &buffer,
						offset: 0,
						size: None,
					}),
				}],
			});

		self.render_data
			.cached_buffers
			.insert(name, (instances.len(), bg, buffer));
	}

	pub fn draw_cached(&mut self, name: &'static str) {
		self.set_uniform(self.uniform.expect("Uniform not given!"));

		let view = &self.get_frame().texture.create_view(&Default::default());

		let cached_buff = self.render_data.cached_buffers.get(&name).unwrap();

		let mut render_pass =
			self.render_data
				.encoder
				.begin_render_pass(&wgpu::RenderPassDescriptor {
					label: None,
					color_attachments: &[Some(wgpu::RenderPassColorAttachment {
						view,
						resolve_target: None,
						ops: wgpu::Operations {
							load: wgpu::LoadOp::Load,
							store: true,
						},
					})],
					depth_stencil_attachment: None,
				});

		render_pass.set_pipeline(&self.resources.pipeline);
		render_pass.set_bind_group(0, &self.render_data.uniform_bg, &[]);
		render_pass.set_bind_group(1, &cached_buff.1, &[]);
		render_pass.set_bind_group(2, &self.render_data.texture_bg, &[]);
		render_pass.draw(0..5, 0..cached_buff.0 as u32);

		drop(render_pass);
	}

	pub fn draw(&mut self, instances: &[InstanceType]) {
		if !instances.is_empty() {
			self.set_instances(instances);
			self.set_uniform(self.uniform.expect("Uniform not given!"));

			let view = &self.get_frame().texture.create_view(&Default::default());

			let mut render_pass =
				self.render_data
					.encoder
					.begin_render_pass(&wgpu::RenderPassDescriptor {
						label: None,
						color_attachments: &[Some(wgpu::RenderPassColorAttachment {
							view,
							resolve_target: None,
							ops: wgpu::Operations {
								load: wgpu::LoadOp::Load,
								store: true,
							},
						})],
						depth_stencil_attachment: None,
					});

			render_pass.set_pipeline(&self.resources.pipeline);
			render_pass.set_bind_group(0, &self.render_data.uniform_bg, &[]);
			render_pass.set_bind_group(1, &self.render_data.instance_bg, &[]);
			render_pass.set_bind_group(2, &self.render_data.texture_bg, &[]);
			render_pass.draw(0..5, 0..self.render_data.instance_len as u32);

			drop(render_pass);
		}
	}

	pub fn create_texture_from_image(&self, image: &image::RgbaImage) -> wgpu::Texture {
		self.resources.create_texture_from_image(image)
	}

	fn get_frame(&mut self) -> &wgpu::SurfaceTexture {
		self.render_data
			.current_frame
			.get_or_insert_with(|| self.resources.generate_frame())
	}
}
