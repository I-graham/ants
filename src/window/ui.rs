#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MouseState {
    Up,
    Drag,
    Click,
    Release,
}

impl MouseState {
    pub fn update(&mut self, down: bool) {
        use MouseState::*;
        match *self {
            Up | Release if down => *self = Click,
            Click if down => *self = Drag,
            Drag | Click if !down => *self = Release,
            Release if !down => *self = Up,
            _ => (),
        }
    }

    pub fn is_down(&self) -> bool {
        use MouseState::*;
        match *self {
            Up | Release => false,
            Click | Drag => true,
        }
    }
}
