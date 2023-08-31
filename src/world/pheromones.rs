use super::*;
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
	pub const DECAY_RATE: f32 = -std::f32::consts::LN_2 / Self::HALF_LIFE;

	pub fn new(pos: Vector2<f32>, dir: Vector2<f32>, ty: Pheromone) -> Self {
		Self {
			pos,
			ty,
			dir,
			strength: 1.0,
		}
	}
}

impl GameObject for Trail {
	type Scene = World;
	type Action = ();

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		//Integrates to
		self.strength += external.delta * Self::DECAY_RATE * self.strength;
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		let color = match self.ty {
			Pheromone::ToFood => (0., 1., 0., self.strength),
			Pheromone::ToHome => (1., 0., 0., self.strength),
		};

		//Some math shows this is the time elapsed
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

impl utils::Relax for Trail {
	fn plan_frequency(&self) -> f32 {
		60.
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
