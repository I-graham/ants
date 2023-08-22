mod animation;

use super::{glsl::*, ui};

use cgmath::*;
use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};
use ui::*;
use winit::event::*;

pub use animation::Animation;

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

pub struct External {
	pub scroll: f32,
	pub mouse_pos: (f32, f32),
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

	pub fn instance(&self, texture: Texture) -> Instance {
		self.texture_map[&texture]
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
		self.mouse_pos = (
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

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Debug, Eq, Clone, Copy)]
pub enum Texture {
	Ant,
	Swirl,
	Flat,
	Queen,
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		1
	}
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Instance {
	pub color_tint: GLvec4,
	pub texture: GLvec4,
	pub scale: GLvec2,
	pub position: GLvec2,
	pub rotation: GLfloat,
	pub screen_relative: GLbool,
}

impl Instance {
	pub fn scale(self, r: f32) -> Self {
		self.scale2(r, r)
	}

	pub fn scale2(self, x: f32, y: f32) -> Self {
		Self {
			scale: GLvec2(x * self.scale.0, y * self.scale.1),
			..self
		}
	}

	pub fn nth_frame(self, n: u32, out_of: u32) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.texture;
		let shift = (lry - uly) / out_of as f32;
		let starty = uly + n as f32 * shift;

		const ANTI_BLEED_MULTIPLIER: f32 = 10. * f32::EPSILON;
		let anti_bleed = shift * ANTI_BLEED_MULTIPLIER;

		Self {
			texture: GLvec4(ulx, starty + anti_bleed, lrx, starty + shift - anti_bleed),
			..self
		}
	}
}

impl Default for Instance {
	fn default() -> Self {
		Instance {
			color_tint: GLvec4(1.0, 1.0, 1.0, 1.0),
			texture: GLvec4(0.0, 0.0, 1.0, 1.0),
			scale: GLvec2(1.0, 1.0),
			position: GLvec2(0.0, 0.0),
			rotation: GLfloat(0.0),
			screen_relative: GLbool::False,
		}
	}
}

#[derive(Clone, Copy)]
pub struct Camera {
	pub pos: Vector2<f32>,
	pub scale: f32,
}

impl Camera {
	pub fn proj(&self, aspect: f32) -> Matrix4<f32> {
		ortho(
			self.pos.x - aspect * self.scale,
			self.pos.x + aspect * self.scale,
			self.pos.y - self.scale,
			self.pos.y + self.scale,
			-100.,
			100.,
		)
	}
}
