use super::*;

use cgmath::*;

#[derive(Clone, Copy, PartialEq)]
pub struct WorkerPlan {
	state: WorkerState,
	last_trail: Vector2<f32>,
}

#[derive(Clone, Copy, PartialEq)]
enum WorkerState {
	Wander(Vector2<f32>),
	GoToFood(Vector2<f32>),
	GoToHome(Vector2<f32>),
}

impl AntPlan for WorkerPlan {
	type Action = Trail;

	fn spawn(pos: Vector2<f32>, dir: Vector2<f32>) -> Self {
		Self {
			state: WorkerState::Wander(pos + dir),
			last_trail: pos,
		}
	}

	fn next_plan(&self, ant: &Ant<Self>, world: &World, messenger: &Sender<Dispatch>) -> Self {
		use WorkerState::*;

		let state = match self.state {
			Wander(toward) => {
				if let Some(food) = world.food.nearest(ant.pos(), Self::SMELL_RAD) {
					GoToFood(food.pos)
				} else if ant.pos.distance(toward) < Self::EXPLORATION {
					let offset = rand_in2d(-1., 1.);
					Wander(ant.pos + 2. * Self::EXPLORATION * (ant.dir + offset))
				} else {
					Wander(toward)
				}
			}

			GoToFood(food) => {
				if world.food.get(food.into()).is_none() {
					Wander(food)
				} else if food.distance(ant.pos) < Self::TRAIL_SEP {
					let message = Dispatch::new(Some(food.into()), Signal::ConsumeFood, 0.);
					messenger.send(message).expect("Unable to send message.");
					GoToHome(self.last_trail - ant.pos)
				} else {
					self.state
				}
			}

			GoToHome(curr) => {
				let mut sum_dir = curr * Self::PREFER_STRAIGHT;

				for (d, trail) in world
					.trails
					.query_with_dist(ant.pos.into(), Self::TRAIL_SMELL_RAD)
					.filter(|(_, t)| t.ty == Pheromone::ToHome)
				{
					let toward_trail = unit_toward(trail.pos, ant.pos);
					let direction = 0.5 * toward_trail + trail.dir;
					sum_dir -= direction.normalize_to(trail.strength / (0.1 + d));
				}

				let normal = sum_dir.normalize();

				if normal != curr {
					GoToHome(normal)
				} else {
					Wander(ant.pos + curr)
				}
			}
		};

		Self { state, ..*self }
	}

	fn action(
		&mut self,
		ant: &Ant<Self>,
		_external: &External,
	) -> (Vector2<f32>, Option<Self::Action>) {
		use WorkerState::*;
		let (dir, pheromone) = match self.state {
			Wander(toward) => (toward - ant.pos, Pheromone::ToHome),

			GoToFood(pos) => (pos - ant.pos, Pheromone::ToHome),

			GoToHome(dir) => (dir, Pheromone::ToFood),
		};

		let trail = if ant.pos.distance2(self.last_trail) > Self::TRAIL_SEP.powf(2.) {
			self.last_trail = ant.pos;
			Some(Trail::new(ant.pos, ant.dir, pheromone))
		} else {
			None
		};

		(dir, trail)
	}

	fn texture(&self) -> Texture {
		Texture::Ant
	}
}
