use super::*;

pub trait Automaton<Scene> {
	type Action;
	type State: Copy + Eq;

	fn state(&self) -> Self::State;
	fn state_mut(&mut self) -> &mut Self::State;
	fn enter_from(&mut self, _old: Self::State) {}
	fn exit_to(&mut self, _new: Self::State) {}
	fn next_state(&self, external: &External) -> Self::State;

	fn by_probability(&self, probability_table: &[(Self::State, f32)]) -> Self::State {
		let mut rng = random();
		for &(state, prob) in probability_table {
			if rng < prob {
				return state;
			}
			rng -= prob;
		}
		self.state()
	}

	fn plan(&self, _scene: &Scene, _external: &External, _messenger: &Sender<Dispatch>) {}

	fn update(&mut self, _external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		None
	}

	fn instance(&self, _external: &External) -> Option<Instance> {
		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		if let Some(inst) = self.instance(external) {
			external.clip(out, inst);
		}
	}

	fn cleanup(&mut self) {}
}

impl<Scene, T: Automaton<Scene>> GameObject<Scene> for T {
	type Action = <T as Automaton<Scene>>::Action;

	fn plan(&self, scene: &Scene, external: &External, messenger: &Sender<Dispatch>) {
		Automaton::plan(self, scene, external, messenger)
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let old = self.state();
		let new = self.next_state(external);

		if new != old {
			self.exit_to(new);
			*self.state_mut() = new;
			self.enter_from(old);
		}

		Automaton::update(self, external, messenger)
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		Automaton::render(self, context, out)
	}

	fn cleanup(&mut self) {
		Automaton::cleanup(self)
	}
}
