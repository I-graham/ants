use super::*;
use crate::game::*;
use crate::window::*;
use cgmath::*;

pub struct Trail {
	pub pos: Vector2<f32>,
	pub dir: Vector2<f32>,
	pub ty: Pheromone,
	pub strength: f32,
}

#[repr(u32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Pheromone {
	ToFood,
	ToHome,
}

impl Trail {
	pub const SIZE: f32 = 3.;
	pub const HALF_LIFE: f32 = 7.0;
	pub const ALIVE_THRESHOLD: f32 = 0.0125;
	pub const MERGE_RADIUS: f32 = 10.;
	pub const MERGE_DIR_TOL: f32 = 0.5;
	pub const DECAY_RATE: f32 = -std::f32::consts::LN_2 / Self::HALF_LIFE;

	pub fn new(pos: Vector2<f32>, dir: Vector2<f32>, ty: Pheromone) -> Self {
		Self {
			pos,
			ty,
			dir,
			strength: 1.0,
		}
	}

	pub fn clump(a: &Self, b: &Self) -> Option<Self> {
		if a.pos.distance2(b.pos) < Self::MERGE_RADIUS.powi(2)
			&& a.dir.dot(b.dir) > Self::MERGE_DIR_TOL
			&& a.ty == b.ty
		{
			Some(Self {
				pos: (a.pos + b.pos) / 2.,
				dir: (a.dir + b.dir) / 2.,
				ty: a.ty,
				strength: a.strength + b.strength,
			})
		} else {
			None
		}
	}
}

impl GameObject<World> for Trail {
	type Action = ();

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		self.strength += external.delta * Self::DECAY_RATE * self.strength;
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		let color = match self.ty {
			Pheromone::ToFood => (0., 1., 0., self.strength),
			Pheromone::ToHome => (1., 0., 0., self.strength),
		};

		let elapsed = f32::ln(self.strength) / Self::DECAY_RATE;

		Some(
			Instance {
				position: self.pos.into(),
				rotation: (90. * elapsed).into(),
				color_tint: color.into(),
				scale: (2, 2).into(),
				..external.instance(Texture::Swirl)
			}
			.scale(Self::SIZE),
		)
	}
}

impl Griddable for Trail {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}

	fn alive(&self) -> bool {
		self.strength > Self::ALIVE_THRESHOLD
	}
}
