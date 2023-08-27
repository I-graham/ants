use super::*;

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

	pub fn screen_to_world(&self, p: Vector2<f32>) -> Vector2<f32> {
		self.scale * p + self.pos
	}
}
