mod ants;
mod food;
mod interface;
mod pheromones;

pub use ants::*;
pub use food::*;
pub use interface::*;
pub use pheromones::*;

use super::*;
use crate::window::*;
use cgmath::*;
use rayon::prelude::*;
use tracing::trace_span;
use utils::Grid;

pub struct World {
	interface: Interface,
	queen: Queen,
	ants: Grid<Relaxed<Worker>>,
	food: Grid<Relaxed<Food>>,
	trails: Grid<Relaxed<Trail>>,
}

impl World {
	pub fn new() -> Self {
		const NUM_FOOD: usize = 100;

		Self {
			interface: Default::default(),
			queen: Queen::new((0., 0.).into(), 0.),
			ants: Grid::new(200.),
			food: Grid::from_iter(
				32.,
				std::iter::repeat_with(|| Food::new(utils::rand_in2d(-1000., 1000.)).into())
					.take(NUM_FOOD),
			),

			trails: Grid::new(3. * WorkerPlan::TRAIL_SEP),
		}
	}
}

impl GameObject for World {
	type Scene = ();
	type Action = ();

	fn plan(&self, _: &(), external: &External, messenger: &Sender<Dispatch>) {
		let span = trace_span!("Planning");
		let _guard = span.enter();

		self.queen.plan(self, external, messenger);

		self.ants
			.par_iter()
			.for_each_with(messenger.clone(), |sender, ant| {
				ant.plan(self, external, sender);
			});
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let span = trace_span!("Updating");
		let _guard = span.enter();

		{
			let span = trace_span!("Interfacing");
			let _guard = span.enter();
			if let Some(trail) = self.interface.update(external, messenger) {
				self.trails.insert(trail.into())
			}
		}

		{
			let span = trace_span!("Food");
			let _guard = span.enter();
			for food in self.food.iter_mut() {
				food.update(external, messenger);
			}
		}

		{
			let span = trace_span!("Ants");
			let _guard = span.enter();

			if let Some(plan) = self.queen.update(external, messenger) {
				let worker =
					Ant::<_>::from_plan(self.queen.pos, rand_in(0., std::f32::consts::TAU), plan);
				self.ants.insert(worker.into());
			}

			for ant in self.ants.iter_mut() {
				if let Some(trail) = ant.update(external, messenger) {
					self.trails.insert(trail.into())
				}
			}
		}

		{
			let span = trace_span!("Trails");
			let _guard = span.enter();
			self.trails.par_iter_mut().for_each(|trail| {
				trail.update(external, messenger);
			});
		}

		rayon::in_place_scope(|s| {
			s.spawn(|_| self.ants.maintain());
			s.spawn(|_| self.food.maintain());
			s.spawn(|_| self.trails.maintain());
		});

		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		let span = trace_span!("Rendering");
		let _guard = span.enter();

		for food in self.food.iter() {
			food.render(external, out);
		}

		for pher in self.trails.iter() {
			pher.render(external, out);
		}

		for ant in self.ants.iter() {
			ant.render(external, out);
		}

		self.queen.render(external, out);
	}

	fn cleanup(&mut self) {
		let span = trace_span!("Debug info");
		let _guard = span.enter();

		self.ants.cleanup();
		self.food.cleanup();
		self.trails.cleanup();

		//self.ants.dbg_analytics();
		//self.trails.dbg_analytics();
	}
}
