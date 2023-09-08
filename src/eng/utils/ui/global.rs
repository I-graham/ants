use super::*;

#[derive(Default)]
pub struct Parent {
	children: Vec<Box<dyn UIElement>>,
	rect: UIRect,
}

impl Parent {
	pub fn plus(&mut self, ui: impl UIElement + 'static) -> &mut Self {
		self.children.push(Box::new(ui));
		self
	}
}

impl GameObject for Parent {
	type Scene = ();
	type Action = UIAction;

	fn plan(&self, scene: &Self::Scene, external: &External, messenger: &Sender<Dispatch>) {
		for child in &self.children {
			child.plan(scene, external, messenger);
		}
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		for child in &mut self.children {
			child.update(external, messenger);
		}
		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		for child in &self.children {
			child.render(external, out);
		}
	}

	fn cleanup(&mut self) {
		for child in &mut self.children {
			child.cleanup();
		}
	}
}

impl UIElement for Parent {
	fn rect(&self) -> &UIRect {
		&self.rect
	}

	fn rect_mut(&mut self) -> &mut UIRect {
		&mut self.rect
	}

	fn propagate_global(&mut self, parent: &UIRect) {
		for child in &mut self.children {
			child.propagate_global(&self.rect.globalize(parent));
		}
	}
}
