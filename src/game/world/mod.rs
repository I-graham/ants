mod ants;
mod food;
mod pheromones;

use super::*;
use ants::*;
use food::*;
use pheromones::*;
use rayon::prelude::*;
use utils::Grid;

pub struct World {
	ants: Grid<Ant>,
	food: Grid<Food>,
	trails: Grid<Trail>,
}

impl World {
	pub fn new() -> Self {
		const NUM_ANTS: usize = 150;
		const NUM_FOOD: usize = 100;
		const GRID_SCALE: f32 = 256.;

		Self {
			ants: Grid::from_iter(
				GRID_SCALE,
				(0..NUM_ANTS)
					.map(|i| Ant::new((0., 0.).into(), 360.0 * i as f32 / NUM_ANTS as f32)),
			),

			food: Grid::from_iter(
				GRID_SCALE,
				std::iter::repeat_with(|| Food::new(utils::rand_in2d(-1000., 1000.)))
					.take(NUM_FOOD),
			),
			trails: Grid::new(GRID_SCALE),
		}
	}
}

impl GameObject<()> for World {
	type Action = ();

	fn plan(&self, _: &(), external: &External, messenger: &Sender<Dispatch>) {
		self.ants
			.par_iter()
			.for_each_with(messenger.clone(), |sender, ant| {
				ant.plan(self, external, sender);
			});
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		self.food.par_iter_mut().for_each(|food| {
			food.update(external, messenger);
		});

		self.trails.par_iter_mut().for_each(|trail| {
			trail.update(external, messenger);
		});

		for trail in self
			.ants
			.iter_mut()
			.filter_map(|ant| ant.update(external, messenger))
		{
			self.trails.insert(trail)
		}

		rayon::scope(|s| {
			s.spawn(|_| self.ants.par_maintain());
			s.spawn(|_| self.food.par_maintain());
			s.spawn(|_| self.trails.par_maintain());
		});

		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		for food in self.food.iter() {
			food.render(external, out);
		}

		for pher in self.trails.iter() {
			pher.render(external, out);
		}

		for ant in self.ants.iter() {
			ant.render(external, out);
		}
	}

	fn cleanup(&mut self) {
		self.ants.cleanup();
		self.food.cleanup();
		self.trails.cleanup();
	}
}
