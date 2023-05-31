use crate::game::*;
use crate::window::*;
use cgmath::*;
use std::time::Instant;

pub struct Trail {
	pub pos: Vector2<f32>,
	pub dir: Vector2<f32>,
	pub ty: Pheromone,
	pub strength: f32,
	birth: Instant,
}

#[repr(u32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Pheromone {
	ToFood,
	ToHome,
}

impl Trail {
	const SIZE: f32 = 3.;
	const HALF_LIFE: f32 = 8.0;
	const ALIVE_THRESHOLD: f32 = 0.025;

	pub fn new(now: Instant, pos: Vector2<f32>, dir: Vector2<f32>, ty: Pheromone) -> Self {
		Self {
			birth: now,
			pos,
			ty,
			dir,
			strength: 1.0,
		}
	}
}

impl GameObject<World> for Trail {
	type Action = ();

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		let elapsed = external.now.duration_since(self.birth).as_secs_f32();
		self.strength = 0.5f32.powf(elapsed / Self::HALF_LIFE);
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		let color = match self.ty {
			Pheromone::ToFood => (0., 1., 0., self.strength),
			Pheromone::ToHome => (1., 0., 0., self.strength),
		};

		Some(
			Instance {
				position: self.pos.into(),
				rotation: 45f32.to_degrees().into(),
				color_tint: color.into(),
				..external.instance(Texture::Flat)
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
