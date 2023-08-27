use strum_macros::{EnumIter, IntoStaticStr};
use std::hash::Hash;


#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Debug, Eq, Clone, Copy)]
pub enum Texture {
	Ant,
	Swirl,
	Flat,
	Queen,
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		1
	}
}
