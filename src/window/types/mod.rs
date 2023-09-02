mod camera;
mod input;
mod instance;
mod texture;

pub use camera::*;
pub use input::*;
pub use instance::*;
pub use texture::*;

use super::glsl::*;
use cgmath::*;
use std::time::Instant;
use winit::event::*;

pub type TextureMap = fnv::FnvHashMap<&'static str, Instance>;

pub struct External {
	pub scroll: f32,
	pub mouse_pos: Vector2<f32>,
	pub left_mouse: ButtonState,
	pub right_mouse: ButtonState,
	pub keymap: fnv::FnvHashMap<VirtualKeyCode, ButtonState>,

	pub texture_map: TextureMap,
	pub win_size: (u32, u32),
	pub camera: Camera,
	pub now: Instant,
	pub delta: f32,
}

impl External {
	pub fn update(&mut self, now: Instant) {
		self.delta = now.duration_since(self.now).as_secs_f32();
		self.now = now;

		self.update_mouse();

		for state in self.keymap.values_mut() {
			state.update(state.is_down());
		}
	}

	pub fn view_dims(&self) -> Vector2<f32> {
		let k = 2. * self.camera.scale;

		vec2(k * self.aspect(), k)
	}

	pub fn point_in_view(&self, p: Vector2<f32>) -> bool {
		let diff = self.camera.pos - p;
		let k = self.camera.scale;
		diff.x.abs() < k * self.aspect() && diff.y.abs() < k
	}

	pub fn visible(&self, instance: Instance) -> bool {
		let (cx, cy) = self.camera.pos.into();
		let GLvec2(px, py) = instance.position;
		let GLvec2(sx, sy) = instance.scale;

		//maximal possible distance, since instances may be rotated
		let max = sx.hypot(sy);

		let (dx, dy) = self.view_dims().into();

		instance.screen_relative == GLbool::True
			|| ((px - cx).abs() < max + dx / 2. && (py - cy).abs() < max + dy / 2.)
	}

	pub fn clip(&self, out: &mut Vec<Instance>, instance: Instance) {
		//clip unseen instances
		if self.visible(instance) {
			out.push(instance);
		}
	}

	pub fn instance<T: TextureType>(&self, texture: T) -> Instance {
		self.texture_map[&texture.name()]
	}

	pub fn aspect(&self) -> f32 {
		self.win_size.0 as f32 / self.win_size.1 as f32
	}

	pub fn mouse_button(&mut self, button: &winit::event::MouseButton, down: bool) {
		use winit::event::MouseButton::{Left, Right};
		match button {
			Left => self.left_mouse.update(down),
			Right => self.right_mouse.update(down),
			_ => (),
		}
	}

	pub fn update_mouse(&mut self) {
		self.left_mouse.update(self.left_mouse.is_down());
		self.right_mouse.update(self.right_mouse.is_down());
	}

	pub fn capture_mouse(&mut self, pos: &winit::dpi::PhysicalPosition<f64>, size: (u32, u32)) {
		let (sx, sy) = (size.0 as f32, size.1 as f32);
		self.mouse_pos = cgmath::vec2(
			(2.0 * pos.x as f32 / sx - 1.0) * sx / sy,
			-2.0 * pos.y as f32 / sy + 1.0,
		);
	}

	pub fn capture_key(&mut self, input: KeyboardInput) {
		let KeyboardInput {
			virtual_keycode: key,
			state,
			..
		} = input;
		match key {
			Some(key) if (VirtualKeyCode::A..VirtualKeyCode::F12).contains(&key) => {
				let down = state == ElementState::Pressed;

				if let Some(button) = self.keymap.get_mut(&key) {
					button.update(down);
				} else {
					self.keymap.insert(key, ButtonState::new(down));
				}
			}
			_ => {}
		}
	}

	pub fn key(&self, key: VirtualKeyCode) -> ButtonState {
		*self.keymap.get(&key).unwrap_or(&ButtonState::Up)
	}
}
