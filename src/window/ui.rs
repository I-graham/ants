#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ButtonState {
	Up,
	Clicked,
	Down,
	Released,
}

impl ButtonState {
	pub fn new(down: bool) -> Self {
		if down {
			Self::Clicked
		} else {
			Self::Released
		}
	}

	pub fn update(&mut self, down: bool) {
		use ButtonState::*;
		match *self {
			Up | Released if down => *self = Clicked,
			Clicked if down => *self = Down,
			Down | Clicked if !down => *self = Released,
			Released if !down => *self = Up,
			_ => (),
		}
	}

	pub fn is_down(&self) -> bool {
		use ButtonState::*;
		match *self {
			Up | Released => false,
			Clicked | Down => true,
		}
	}
}
