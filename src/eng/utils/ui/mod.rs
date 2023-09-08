mod display;
mod global;
mod rect;

pub use display::*;
pub use global::*;
pub use rect::*;

use super::*;
use crate::window::*;

pub enum UIAction {}

pub trait UIElement: GameObject<Scene = (), Action = UIAction> {
	fn rect(&self) -> &UIRect;
	fn rect_mut(&mut self) -> &mut UIRect;
	fn propagate_global(&mut self, _parent: &UIRect);
}
