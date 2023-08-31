use std::sync::mpsc;
use std::time::*;
use strum::*;
use strum_macros::*;

use super::{Grid, Griddable};

#[derive(Clone, Copy, EnumDiscriminants, EnumCount)]
#[strum_discriminants(name(MessageTypes))]
pub enum Signal {
	ConsumeFood,
}

pub struct Messenger {
	now: Instant,
	global: Buckets,
	locals: Grid<(Instant, Dispatch)>,
	sender: mpsc::Sender<Dispatch>,
	receiver: mpsc::Receiver<Dispatch>,
}

unsafe impl Send for Messenger {}
unsafe impl Sync for Messenger {}

type Buckets = [Vec<(Instant, Dispatch)>; Signal::COUNT];

#[derive(Clone, Copy)]
pub struct Dispatch {
	pos: Option<(f32, f32)>,
	signal: Signal,
	delay: f32,
}

impl Messenger {
	pub fn new() -> Self {
		let (sender, receiver) = mpsc::channel();
		Self {
			now: Instant::now(),
			global: Buckets::default(),
			locals: Grid::new(128.),
			sender,
			receiver,
		}
	}

	pub fn sender(&self) -> mpsc::Sender<Dispatch> {
		self.sender.clone()
	}

	pub fn update(&mut self, now: Instant) {
		self.now = now;

		let alive = |&(time, dispatch): &(Instant, Dispatch)| {
			now < (time + Duration::from_secs_f32(dispatch.delay))
		};

		for bucket in &mut self.global {
			bucket.retain(alive)
		}

		self.locals.retain(alive);
		self.locals.maintain();

		while let Ok(dispatch) = self.receiver.try_recv() {
			if dispatch.pos.is_some() {
				self.locals.insert((self.now, dispatch));
			} else {
				let ty = MessageTypes::from(dispatch.signal);
				self.global[ty as usize].push((self.now, dispatch));
			}
		}
	}

	pub fn global_receive<'a>(
		&'a self,
		types: &'a [MessageTypes],
	) -> impl Iterator<Item = Signal> + 'a {
		types
			.iter()
			.flat_map(|&ty| self.global[ty as usize].iter())
			.map(|(_, dispatch)| dispatch.signal)
	}

	pub fn local_receive<'a>(
		&'a self,
		pos: (f32, f32),
		radius: f32,
		types: &'a [MessageTypes],
	) -> impl Iterator<Item = ((f32, f32), Signal)> + 'a {
		self.locals
			.query_at(pos, radius)
			.map(|&dispatch| (dispatch.pos(), dispatch.1.signal))
			.filter(|(_, signal)| types.contains(&MessageTypes::from(signal)))
	}
}

impl Dispatch {
	pub fn new(pos: Option<(f32, f32)>, signal: Signal, delay: f32) -> Self {
		Self { pos, signal, delay }
	}
}

impl Griddable for (Instant, Dispatch) {
	fn pos(&self) -> (f32, f32) {
		self.1.pos.expect("Global dispatch.")
	}
}
