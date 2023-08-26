use super::{worker::WorkerPlan, *};

#[derive(Clone, Copy, PartialEq)]
pub enum QueenPlan {
	Wander(Vector2<f32>),
}

impl AntPlan for QueenPlan {
	type Action = WorkerPlan;

	const TURN_SPEED: f32 = 20.;

	fn spawn(pos: Vector2<f32>, dir: Vector2<f32>) -> Self {
		Self::Wander(pos + dir)
	}

	fn next_plan(&self, ant: &Ant<Self>, _world: &World, _messenger: &Sender<Dispatch>) -> Self {
		let Self::Wander(toward) = *self;

		let state = if ant.pos.distance(toward) < Self::EXPLORATION {
			let offset = rand_in2d(-1., 1.);
			ant.pos + 2. * Self::EXPLORATION * (ant.dir + offset)
		} else {
			toward
		};

		Self::Wander(state)
	}

	fn action(
		&mut self,
		ant: &Ant<Self>,
		external: &External,
	) -> (Vector2<f32>, Option<Self::Action>) {
		match *self {
			Self::Wander(toward) => {
				use winit::event::VirtualKeyCode;

				let spawned = if external.key(VirtualKeyCode::E) == ui::ButtonState::Clicked {
					Some(WorkerPlan::spawn(ant.pos, unit_in_dir(rand_in(0., 360.))))
				} else {
					None
				};

				(toward, spawned)
			}
		}
	}

	fn texture(&self) -> Texture {
		Texture::Queen
	}
}
