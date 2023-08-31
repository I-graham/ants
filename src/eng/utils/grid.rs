use super::*;
use fnv::*;
use rayon::prelude::*;

pub struct Grid<T: Griddable> {
	scale: f32,
	grid: FnvHashMap<(i32, i32), Vec<usize>>,
	elems: FreeList<T>,
}

pub trait Griddable {
	fn alive(&self) -> bool {
		true
	}

	fn pos(&self) -> (f32, f32);
	fn x(&self) -> f32 {
		self.pos().0
	}
	fn y(&self) -> f32 {
		self.pos().1
	}
}

impl<T: Griddable> Grid<T> {
	pub fn new(scale: f32) -> Self {
		Self {
			scale,
			grid: Default::default(),
			elems: FreeList::new(),
		}
	}

	pub fn count(&self) -> usize {
		self.elems.count()
	}

	pub fn insert(&mut self, item: T) {
		let cell = Self::grid_cell(self.scale, item.pos());
		let index = self.elems.insert(item);
		self.grid.entry(cell).or_insert(vec![]).push(index);
	}

	pub fn get(&self, pos: (f32, f32)) -> Option<&T> {
		let cell = self.grid.get(&Self::grid_cell(self.scale, pos));
		cell.and_then(|v| {
			v.iter()
				.find_map(|&index| Some(&self.elems[index]).filter(|e| e.pos() == pos))
		})
	}

	pub fn remove(&mut self, pos: (f32, f32)) -> Option<T> {
		let cell = self.grid.get_mut(&Self::grid_cell(self.scale, pos));

		if let Some(v) = cell {
			if let Some((cell_i, &elem_i)) = v
				.iter()
				.enumerate()
				.find(|(_, &index)| self.elems[index].pos() == pos)
			{
				v.swap_remove(cell_i);
				self.elems.remove(elem_i)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn nearest_by<P>(&self, pos: (f32, f32), radius: f32, mut predicate: P) -> Option<(f32, &T)>
	where
		P: FnMut(f32, &T) -> Option<f32>,
	{
		self.query_with_dist(pos, radius)
			.filter_map(|(d, t)| predicate(d, t).zip(Some(t)))
			.min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
	}

	pub fn nearest(&self, pos: (f32, f32), radius: f32) -> Option<&T> {
		self.nearest_dist(pos, radius).map(|d| d.1)
	}

	pub fn nearest_dist(&self, pos: (f32, f32), radius: f32) -> Option<(f32, &T)> {
		self.query_with_dist(pos, radius)
			.min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap())
	}

	pub fn query_at(&self, pos: (f32, f32), radius: f32) -> impl Iterator<Item = &T> {
		self.query_with_dist(pos, radius).map(|(_, item)| item)
	}

	pub fn query_with_dist(
		&self,
		(x, y): (f32, f32),
		radius: f32,
	) -> impl Iterator<Item = (f32, &T)> {
		let (hi_x, hi_y) = Self::grid_cell(self.scale, (x + radius, y + radius));
		let (lo_x, lo_y) = Self::grid_cell(self.scale, (x - radius, y - radius));

		(lo_x..=hi_x)
			.flat_map(move |i| (lo_y..=hi_y).map(move |j| (i, j)))
			.filter_map(|cell| self.grid.get(&cell))
			.flatten()
			.map(move |&index| {
				let item = &self.elems[index];
				(dist(item.pos(), (x, y)), item)
			})
			.filter(move |(d, i)| *d <= radius && i.alive())
	}

	//pairs not guaranteed to come out in any particular order.
	//all pairs are unordered and distinctly located.
	pub fn nearby_pairs(&self, distance: f32) -> impl Iterator<Item = (&T, &T)> {
		let radius = (distance / self.scale).ceil() as i32;

		let is_nearby =
			move |a: &T, b: &T| -> bool { a.pos() < b.pos() && dist(a.pos(), b.pos()) <= distance };

		self.grid
			.iter()
			.flat_map(move |(&(cx, cy), ids)| {
				(cx..=cx + radius)
					.flat_map(move |ix| (cy..=cy + radius).map(move |iy| (ix, iy)))
					.filter_map(move |cell2| self.grid.get(&cell2))
					.map(move |jds| (ids, jds))
			})
			.flat_map(move |(ids, jds)| {
				let ielems = ids.iter().map(|&i| &self.elems[i]);
				let jelems = jds.iter().map(|&j| &self.elems[j]);
				ielems.flat_map(move |i| {
					jelems
						.clone()
						.filter(move |j| is_nearby(i, j))
						.map(move |j| (i, j))
				})
			})
	}

	pub fn retain<P: FnMut(&T) -> bool>(&mut self, mut predicate: P) {
		for vec in self.grid.values_mut() {
			let mut i = 0;
			while i < vec.len() {
				if !predicate(&self.elems[vec[i]]) {
					self.elems.remove(vec[i]);
					vec.swap_remove(i);
				} else {
					i += 1;
				}
			}
		}
	}

	pub fn maintain(&mut self) {
		self.elems.sort_frees();

		let mut moved = vec![];

		for (&bucket, vec) in &mut self.grid {
			let mut i = 0;
			while i < vec.len() {
				let elem = &self.elems[vec[i]];
				let alive = elem.alive();
				let cell = Self::grid_cell(self.scale, elem.pos());

				if !alive {
					self.elems.remove(vec[i]);
					vec.swap_remove(i);
				} else if cell != bucket {
					moved.push((cell, vec[i]));
					vec.swap_remove(i);
				} else {
					i += 1;
				}
			}
		}

		for (cell, index) in moved {
			self.grid.entry(cell).or_insert(vec![]).push(index);
		}
	}

	pub fn cleanup(&mut self) {
		self.grid.retain(|_, v| !v.is_empty());
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.elems.iter().filter(|e| e.alive())
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.elems.iter_mut().filter(|e| e.alive())
	}

	pub fn from_iter<I: Iterator<Item = T>>(scale: f32, iter: I) -> Self {
		let mut grid = Grid::new(scale);
		for i in iter {
			grid.insert(i);
		}
		grid
	}

	fn grid_cell(scale: f32, (x, y): (f32, f32)) -> (i32, i32) {
		debug_assert!(!x.is_nan() && !y.is_nan());
		((x / scale).floor() as i32, (y / scale).floor() as i32)
	}

	pub fn dbg_analytics(&self) {
		let cells = self.grid.len();
		let len = self.elems.count();
		let max = self.grid.values().map(|v| v.len()).max();
		let mean = self.grid.values().map(|v| v.len()).sum::<usize>() as f32 / cells as f32;
		dbg!(cells);
		dbg!(len);
		dbg!(max);
		dbg!(mean);
	}
}

impl<T: Griddable + Send + Sync> Grid<T> {
	pub fn par_maintain(&mut self) {
		use std::sync::*;

		self.elems.sort_frees();

		let (moved_s, moved_r) = mpsc::channel();
		let elems = RwLock::new(&mut self.elems);

		self.grid
			.par_iter_mut()
			.for_each_with(moved_s, |moved_s, (&bucket, vec)| {
				let mut i = 0;
				while i < vec.len() {
					let lock = elems.read().unwrap();
					let elem = &lock[vec[i]];
					let alive = elem.alive();
					let cell = Self::grid_cell(self.scale, elem.pos());
					std::mem::drop(lock);

					if !alive {
						let mut lock = elems.write().unwrap();
						lock.remove(vec[i]);
						vec.swap_remove(i);
					} else if cell != bucket {
						moved_s.send((cell, vec[i])).unwrap();
						vec.swap_remove(i);
					} else {
						i += 1;
					}
				}
			});

		for (cell, index) in moved_r.iter() {
			self.grid.entry(cell).or_insert(vec![]).push(index);
		}
	}

	pub fn par_iter(&self) -> impl ParallelIterator<Item = &T> {
		self.elems.par_iter()
	}

	pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T> {
		self.elems.par_iter_mut()
	}
}
