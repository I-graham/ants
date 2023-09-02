use super::*;

use crate::window::WinApi;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;

pub struct GameState<World: Root> {
	pub(super) api: WinApi,
	messenger: Messenger,
	world: World,
}

impl<World: Root> GameState<World> {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let api = WinApi::new::<World::Texture>(event_loop);
		Self {
			world: World::init(),
			messenger: Messenger::new(),
			api,
		}
	}

	pub fn step(&mut self) {
		self.world
			.plan(&(), &self.api.external, &self.messenger.sender());
		self.world.update(&self.api.external, &self.messenger);

		let now = std::time::Instant::now();
		self.api.external.update(now);
		self.messenger.update(now);

		const CAM_MOVE_SPEED: f32 = 50.;

		self.api.external.camera.pos.x += CAM_MOVE_SPEED
			* self.api.external.delta
			* (self.api.external.key(VirtualKeyCode::D).is_down() as i32
				- self.api.external.key(VirtualKeyCode::A).is_down() as i32) as f32;

		self.api.external.camera.pos.y += CAM_MOVE_SPEED
			* self.api.external.delta
			* (self.api.external.key(VirtualKeyCode::W).is_down() as i32
				- self.api.external.key(VirtualKeyCode::S).is_down() as i32) as f32;

		const CAM_SCALE_SPEED: f32 = 50.;

		self.api.external.camera.scale += CAM_SCALE_SPEED
			* self.api.external.delta
			* (self.api.external.key(VirtualKeyCode::Q).is_down() as i32
				- self.api.external.key(VirtualKeyCode::Z).is_down() as i32) as f32;
	}

	pub fn draw(&mut self) {
		self.api.clear();

		self.world.render(&self.api.external, &mut self.api.output);

		self.api.draw();
	}

	pub fn cleanup(&mut self) {
		self.world.cleanup()
	}
}
