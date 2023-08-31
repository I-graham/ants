use super::*;

#[derive(Default)]
pub struct Interface {
	last_trail: Option<Vector2<f32>>,
}

impl GameObject for Interface {
	type Scene = World;
	type Action = Trail;

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		use ButtonState::*;
		let pos = external.camera.screen_to_world(external.mouse_pos);

		match (external.left_mouse, self.last_trail) {
			(Pressed, _) => {
				self.last_trail = Some(pos);
				None
			}
			(Down, Some(last))
				if external.point_in_view(pos) && pos.distance(last) > WorkerPlan::TRAIL_SEP =>
			{
				self.last_trail = Some(pos);
				let trail = Trail::new(pos, unit_toward(pos, last), Pheromone::ToHome);
				Some(trail)
			}
			_ => None,
		}
	}
}
