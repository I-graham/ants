use super::messenger::Messenger;
use super::world::World;
use super::GameObject;
use crate::window::WinApi;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;

pub struct GameState {
	pub(super) api: WinApi,
	messenger: Messenger,
	world: World,
}

impl GameState {
	pub(super) fn new(event_loop: &EventLoop<()>) -> Self {
		let api = WinApi::new(event_loop);
		Self {
			world: World::new(),
			messenger: Messenger::new(),
			api,
		}
	}

	pub(super) fn step(&mut self) {
		self.messenger.cleanup();
		self.world.plan(
			&self.world,
			&self.api.external,
			&self.api.input,
			&mut self.messenger,
		);

		self.world.update(&self.api.external, &self.messenger);
		self.api.external.refresh();

		self.api.input.update_mouse();

		const SCALE_SPEED: f32 = 20.;

		self.api.external.camera.scale += SCALE_SPEED
			* self.api.external.delta
			* (self.api.input.key(VirtualKeyCode::Q) as i32
				- self.api.input.key(VirtualKeyCode::Z) as i32) as f32;
	}

	pub(super) fn draw(&mut self) {
		self.api.clear();

		self.world.render(&self.api.external, &mut self.api.output);

		self.api.draw();
	}
}
