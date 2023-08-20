use super::*;
use crate::game::*;
use crate::window::*;
use cgmath::*;
use plans::*;
use std::cell::Cell;

#[derive(Clone)]
pub struct Ant<Plan: AntPlan> {
	pub pos: Vector2<f32>,
	pub dir: Vector2<f32>,
	pub behavior: Cell<Plan>,
}

impl<Plan: AntPlan> Ant<Plan> {
	pub fn new(pos: Vector2<f32>, dir: f32) -> Self {
		Self {
			pos,
			dir: unit_in_dir(dir),
			behavior: Cell::new(Plan::default()),
		}
	}

	pub fn new_with_plan(pos: Vector2<f32>, dir: f32, plan: Plan) -> Self {
		Self {
			pos,
			dir: unit_in_dir(dir),
			behavior: Cell::new(plan),
		}
	}
}

impl<Plan: AntPlan> GameObject<World> for Ant<Plan> {
	type Action = Plan::Action;

	fn plan(&self, world: &World, _external: &External, messenger: &Sender<Dispatch>) {
		let next_plan = self.behavior.get().next_plan(self, world, messenger);

		self.behavior.set(next_plan);
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Plan::Action> {
		let (next_dir, action) = self.behavior.get().action(self, external);

		let next_dir = if next_dir.magnitude() > 0. {
			next_dir.normalize()
		} else {
			self.dir
		};

		//gradual turning
		let curr_ang = angle(self.dir);
		let Rad(diff) = next_dir.angle(self.dir);
		let new_ang = curr_ang + Plan::TURN_SPEED * diff * external.delta;

		//slow down on wide turns
		let slow_down = self.dir.dot(next_dir).abs();

		self.dir = unit_in_dir(new_ang);
		self.pos += self
			.dir
			.normalize_to(Plan::SPEED * external.delta * slow_down);

		action
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.pos.into(),
			rotation: angle(self.dir).to_degrees().into(),
			..external.instance(self.behavior.get().texture())
		})
	}
}

impl<Plan: AntPlan> utils::Griddable for Ant<Plan> {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}

unsafe impl<Plan: AntPlan + Send + Sync> Send for Ant<Plan> {}
unsafe impl<Plan: AntPlan + Send + Sync> Sync for Ant<Plan> {}
