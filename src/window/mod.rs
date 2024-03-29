pub mod glsl;
mod loader;
mod reng;
mod types;

pub use glsl::*;
use std::time::Instant;

pub use types::*;

const START_WIN_SIZE: winit::dpi::PhysicalSize<f32> = winit::dpi::PhysicalSize {
	width: 400.0,
	height: 400.0,
};

pub struct WinApi {
	pub window: winit::window::Window,
	pub external: External,
	pub output: Vec<Instance>,
	renderer: reng::Renderer<glsl::Uniform, Instance>,
}

impl WinApi {
	pub fn new<Texture: TextureType>(
		event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
	) -> Self {
		let window = winit::window::WindowBuilder::new()
			.with_min_inner_size(START_WIN_SIZE)
			.build(event_loop)
			.expect("unable to create window");

		let size = window.inner_size();

		let mut renderer = reng::Renderer::new(&window, 4);

		let (image, texture_map) = loader::load_textures::<Texture>();
		let texture = renderer.create_texture_from_image(&image);
		renderer.set_texture(&texture);

		Self {
			window,
			renderer,
			external: External {
				scroll: 0.,
				mouse_pos: cgmath::vec2(0.0, 0.0),
				left_mouse: ButtonState::Up,
				right_mouse: ButtonState::Up,
				keymap: fnv::FnvHashMap::default(),
				texture_map,
				camera: Camera {
					pos: cgmath::vec2(0., 0.),
					scale: 256.,
				},
				win_size: (size.width, size.height),
				now: Instant::now(),
				delta: 0.,
			},
			output: vec![],
		}
	}

	pub fn clear(&mut self) {
		//White for debugging purposes.
		self.output.clear();
		self.renderer.clear(wgpu::Color::WHITE);
	}

	pub fn draw(&mut self) {
		self.renderer.set_uniform(glsl::Uniform {
			ortho: self.external.camera.proj(self.external.aspect()),
		});
		self.renderer.draw(&self.output);
	}

	pub fn submit(&mut self) {
		self.renderer.submit();
	}

	pub fn resize(&mut self, dims: winit::dpi::PhysicalSize<u32>) {
		self.external.win_size = (dims.width, dims.height);
		self.renderer.resize(dims);
	}

	pub fn id(&self) -> winit::window::WindowId {
		self.window.id()
	}
}
