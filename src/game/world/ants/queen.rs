use tracing_subscriber::util;

use super::{worker::WorkerPlan, *};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum QueenPlan {
	#[default]
	Wander,
}

impl AntPlan for QueenPlan {
	type Action = WorkerPlan;

	fn next_plan(&self, _ant: &Ant<Self>, _world: &World, _messenger: &Sender<Dispatch>) -> Self {
		*self
	}

	fn action(
		&mut self,
		ant: &Ant<Self>,
		external: &External,
	) -> (Vector2<f32>, Option<Self::Action>) {
		match *self {
			Self::Wander => {
				let offset =
					external.delta * rand_in(-Self::EXPLORATION, Self::EXPLORATION).powi(3);
				(
					unit_in_dir(angle(ant.dir) + offset),
					Some(Default::default()).filter(|_| utils::probability(0.0005)),
				)
			}
		}
	}

	fn texture(&self) -> Texture {
		Texture::Queen
	}
}
