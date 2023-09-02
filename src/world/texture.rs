use std::hash::Hash;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::window::TextureType;

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Debug, Eq, Clone, Copy)]
pub enum Texture {
	Ant,
	Swirl,
	Flat,
	Queen,
}

impl TextureType for Texture {
	fn list() -> Vec<Self> {
		use strum::IntoEnumIterator;
		Self::iter().collect()
	}

	fn name(&self) -> &'static str {
		self.into()
	}

	fn frame_count(&self) -> u32 {
		1
	}
}
