use cgmath::Vector2;

use crate::game::*;
use crate::window::*;

#[derive(Clone)]
pub struct Food {
	pub pos: Vector2<f32>,
	pub amount: usize,
}

impl Food {
	pub fn new(pos: Vector2<f32>) -> Self {
		Self { pos, amount: 10 }
	}
}

impl GameObject for Food {
	type Scene = World;
	type Action = ();

	fn update(&mut self, _external: &External, messenger: &Messenger) -> Option<Self::Action> {
		for _ in messenger.local_receive(self.pos(), 0., &[MessageTypes::ConsumeFood]) {
			if self.amount > 0 {
				self.amount -= 1;
			}
		}
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(
			Instance {
				position: self.pos.into(),
				color_tint: (0., 0., 1., 1.).into(),
				..external.instance(Texture::Flat)
			}
			.scale(self.amount as f32),
		)
	}
}

impl utils::Griddable for Food {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}

	fn alive(&self) -> bool {
		self.amount > 0
	}
}
