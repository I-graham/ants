use super::*;

use cgmath::*;

#[derive(Clone, Copy, PartialEq)]
pub struct WorkerPlan {
	state: WorkerState,
	last_trail: Vector2<f32>,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum WorkerState {
	#[default]
	Wander,
	GoToFood(Vector2<f32>),
	GoToHome(Vector2<f32>),
}

impl AntPlan for WorkerPlan {
	type Action = Trail;

	fn next_plan(&self, ant: &Ant<Self>, world: &World, messenger: &Sender<Dispatch>) -> Self {
		use WorkerState::*;

		let state = match self.state {
			Wander => {
				if let Some(food) = world.food.nearest(ant.pos(), Self::SMELL_RAD) {
					GoToFood(food.pos)
				} else {
					self.state
				}
			}

			GoToFood(food) => {
				if world.food.get(food.into()).is_none() {
					Wander
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
					sum_dir -= trail.dir.normalize_to(trail.strength / (0.5 + d));
				}

				let normal = sum_dir.normalize();

				if normal != curr {
					GoToHome(normal)
				} else {
					Wander
				}
			}
		};

		Self {
			state,
			..*self
		}
	}

	fn action(
		&mut self,
		ant: &Ant<Self>,
		external: &External,
	) -> (Vector2<f32>, Option<Self::Action>) {
		use WorkerState::*;
		let (dir, pheromone) = match self.state {
			Wander => {
				let offset =
					external.delta * rand_in(-Self::EXPLORATION, Self::EXPLORATION).powi(3);
				(unit_in_dir(angle(ant.dir) + offset), Pheromone::ToHome)
			}

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

impl Default for WorkerPlan {
	fn default() -> Self {
		Self {
			state: Default::default(),
			last_trail: vec2(0., 0.),
		}
	}
}
