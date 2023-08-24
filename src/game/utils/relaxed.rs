//Container for objects which don't need to make
//new plans at every frame, to improve performance.

use super::*;
use std::cell::Cell;
use std::time::Instant;

pub trait Relax: GameObject {
	//Number of updates per second
	fn plan_frequency(&self) -> f32;
	fn must_plan(&self, _scene: &Self::Scene, _external: &External) -> bool {
		false
	}
	fn needs_cleanup(&self) -> bool {
		false
	}
}

pub struct Relaxed<T> {
	last_plan: Cell<Instant>,
	now: Instant,
	inner: T,
}

impl<T> Relaxed<T> {
	pub fn new(value: T) -> Self {
		let now = Instant::now();
		Self {
			last_plan: now.into(),
			now,
			inner: value,
		}
	}
}

impl<T: Relax> GameObject for Relaxed<T> {
	type Scene = T::Scene;
	type Action = T::Action;

	fn plan(&self, scene: &Self::Scene, external: &External, messenger: &Sender<Dispatch>) {
		let elapsed = external
			.now
			.duration_since(self.last_plan.get())
			.as_secs_f32();
		let period = 1. / self.inner.plan_frequency();

		if elapsed > period || self.inner.must_plan(scene, external) {
			self.last_plan.set(external.now);
			self.inner.plan(scene, external, messenger)
		}
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		self.now = external.now;
		self.inner.update(external, messenger)
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		self.inner.render(external, out)
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		self.inner.instance(external)
	}

	fn cleanup(&mut self) {
		let elapsed = self.now.duration_since(self.last_plan.get()).as_secs_f32();
		let period = 1. / self.inner.plan_frequency();

		if elapsed > period || self.inner.needs_cleanup() {
			self.inner.cleanup()
		}
	}
}
