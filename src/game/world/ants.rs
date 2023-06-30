use cgmath::*;

use super::*;
use crate::game::*;
use crate::window::*;
use std::cell::Cell;
use std::f32::consts::*;

#[derive(Clone)]
pub struct Ant {
	pos: Vector2<f32>,
	dir: Vector2<f32>,
	goal: Cell<Goal>,
	last_trail: Vector2<f32>,
}

unsafe impl Send for Ant {}
unsafe impl Sync for Ant {}

#[derive(Clone, Copy)]
pub enum Goal {
	Wander,
	GoToFood(Vector2<f32>),
	GoToHome(Vector2<f32>),
}

impl Ant {
	pub const SMELL_RAD: f32 = 50.;
	pub const TRAIL_SMELL_RAD: f32 = 5. * Self::TRAIL_SEP;
	pub const SPEED: f32 = 15.0;
	pub const TURN_SPEED: f32 = 180. * TAU / 360.;
	pub const EXPLORATION: f32 = Self::TURN_SPEED;
	pub const TRAIL_SEP: f32 = 15.0;
	pub const PREFER_STRAIGHT: f32 = 0.6;

	pub fn new(pos: Vector2<f32>, dir: f32) -> Self {
		Self {
			pos,
			dir: unit_in_dir(dir),
			goal: Cell::new(Goal::Wander),
			last_trail: pos,
		}
	}
}

impl GameObject<World> for Ant {
	type Action = Trail;

	fn plan(&self, world: &World, _external: &External, messenger: &Sender<Dispatch>) {
		match self.goal.get() {
			Goal::Wander => {
				if let Some(food) = world.food.nearest(self.pos(), Self::SMELL_RAD) {
					self.goal.set(Goal::GoToFood(food.pos))
				}
			}

			Goal::GoToFood(food) => {
				if world.food.get(food.into()).is_none() {
					self.goal.set(Goal::Wander)
				} else if food.distance(self.pos) < Self::TRAIL_SEP {
					let message = Dispatch::new(Some(food.into()), Signal::ConsumeFood, 0.);
					messenger.send(message).expect("Unable to send message.");
					self.goal.set(Goal::GoToHome(self.last_trail - self.pos))
				}
			}

			Goal::GoToHome(curr) => {
				let mut sum_dir = curr * Self::PREFER_STRAIGHT;

				for (d, trail) in world
					.trails
					.query_with_dist(self.pos.into(), Self::TRAIL_SMELL_RAD)
					.filter(|(_, t)| t.ty == Pheromone::ToHome)
				{
					sum_dir -= trail.dir.normalize_to(trail.strength / (0.5 + d));
				}

				let normal = sum_dir.normalize();

				self.goal.set(if normal != curr {
					Goal::GoToHome(normal)
				} else {
					Goal::Wander
				});
			}
		}
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		let (next_dir, ty) = match self.goal.get() {
			Goal::Wander => {
				let offset =
					external.delta * rand_in(-Self::EXPLORATION, Self::EXPLORATION).powi(3);
				(unit_in_dir(angle(self.dir) + offset), Pheromone::ToHome)
			}

			Goal::GoToFood(pos) => (pos - self.pos, Pheromone::ToHome),

			Goal::GoToHome(dir) => (dir, Pheromone::ToFood),
		};

		let next_dir = if next_dir.magnitude() > 0. {
			next_dir.normalize()
		} else {
			self.dir
		};

		//gradual turning
		let curr_ang = angle(self.dir);
		let Rad(diff) = next_dir.angle(self.dir);
		let new_ang = curr_ang + Self::TURN_SPEED * diff * external.delta;

		//slow down on wide turns
		let slow_down = self.dir.dot(next_dir).abs();

		self.dir = unit_in_dir(new_ang);
		self.pos += self
			.dir
			.normalize_to(Self::SPEED * external.delta * slow_down);

		if self.pos.distance2(self.last_trail) > Self::TRAIL_SEP.powf(2.) {
			self.last_trail = self.pos;
			Some(Trail::new(self.pos, self.dir, ty))
		} else {
			None
		}
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.pos.into(),
			rotation: angle(self.dir).to_degrees().into(),
			..external.instance(Texture::Ant)
		})
	}
}

impl utils::Griddable for Ant {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}
