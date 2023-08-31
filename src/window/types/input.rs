#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

impl ButtonState {
	pub fn new(down: bool) -> Self {
		if down {
			Self::Pressed
		} else {
			Self::Released
		}
	}

	pub fn update(&mut self, down: bool) {
		use ButtonState::*;
		match *self {
			Up | Released if down => *self = Pressed,
			Pressed if down => *self = Down,
			Down | Pressed if !down => *self = Released,
			Released if !down => *self = Up,
			_ => (),
		}
	}

	pub fn pressed(&self) -> bool {
		*self == ButtonState::Pressed
	}
	
	pub fn released(&self) -> bool {
		*self == ButtonState::Released		
	}

	pub fn is_down(&self) -> bool {
		use ButtonState::*;
		match *self {
			Up | Released => false,
			Pressed | Down => true,
		}
	}
}
