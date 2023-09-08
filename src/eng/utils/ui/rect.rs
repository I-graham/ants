pub use super::*;

#[derive(Clone)]
pub struct UIRect {
	pub offset: Vector2<f32>,
	pub center: Vector2<f32>,
	pub size: Vector2<f32>,
}

impl UIRect {
	pub fn screen(external: &External) -> Self {
		let u32_size = Vector2::<u32>::from(external.win_size);

		Self {
			size: u32_size.cast().unwrap() / 2.,
			..Self::default()
		}
	}

	pub fn globalize(&self, parent_gbl: &UIRect) -> Self {
		Self {
			size: self.size.mul_element_wise(parent_gbl.size),
			offset: self.offset - self.center + parent_gbl.center,
			center: vec2(0., 0.),
		}
	}
}

impl Default for UIRect {
	fn default() -> Self {
		Self {
			offset: vec2(0., 0.),
			center: vec2(0., 0.),
			size: vec2(1., 1.),
		}
	}
}
