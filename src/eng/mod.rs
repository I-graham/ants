mod messenger;
mod play;
mod state;
pub mod utils;

pub use messenger::*;
pub use play::*;
pub use std::sync::mpsc::Sender;
pub use utils::*;

use crate::window::{External, Instance};

pub trait GameObject {
	type Scene;
	type Action;

	fn plan(&self, _scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {}

	fn update(&mut self, _external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		None
	}

	//If object renders a single instance, this can be implemented instea
	//of GameObject::render
	fn instance(&self, _external: &External) -> Option<Instance> {
		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		if let Some(inst) = self.instance(external) {
			external.clip(out, inst);
		}
	}

	//not ever guaranteed to be called. Usefor for occasional but not
	//mandatory cleanup to improve performance or release unused resources.
	fn cleanup(&mut self) {}
}
