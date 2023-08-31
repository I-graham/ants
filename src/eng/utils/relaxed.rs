//Container for objects which don't need to make
//new plans at every frame, to improve performance.

use super::*;
use std::ops::{Deref, DerefMut};
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
	last_plan: Instant,
	now: Instant,
	inner: T,
}

impl<T: Relax> Relaxed<T> {
	pub fn ready(&self) -> bool {
		self.last_plan == self.now
	}
}

impl<T: Relax> GameObject for Relaxed<T> {
	type Scene = T::Scene;
	type Action = T::Action;

	//must be called between calls to update to guarantee that planning happens at the correct times
	fn plan(&self, scene: &Self::Scene, external: &External, messenger: &Sender<Dispatch>) {
		if self.ready() || self.inner.must_plan(scene, external) {
			self.inner.plan(scene, external, messenger)
		}
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		self.now = external.now;

		let elapsed = self.now.duration_since(self.last_plan).as_secs_f32();
		let period = 1. / self.inner.plan_frequency();
		if elapsed > period {
			self.last_plan = self.now;
		}

		self.inner.update(external, messenger)
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		self.inner.render(external, out)
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		self.inner.instance(external)
	}

	fn cleanup(&mut self) {
		if self.ready() || self.inner.needs_cleanup() {
			self.inner.cleanup()
		}
	}
}

impl<T: Relax> From<T> for Relaxed<T> {
	fn from(value: T) -> Self {
		let now = Instant::now();
		Self {
			last_plan: now,
			now,
			inner: value,
		}
	}
}

impl<T: Relax> Deref for Relaxed<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T: Relax> DerefMut for Relaxed<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T: Relax + Griddable> Griddable for Relaxed<T> {
	fn pos(&self) -> (f32, f32) {
		self.inner.pos()
	}

	fn alive(&self) -> bool {
		self.inner.alive()
	}
}
