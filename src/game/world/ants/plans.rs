use super::*;

pub trait AntPlan: Sized + Copy + PartialEq {
	const SMELL_RAD: f32 = 50.;
	const TRAIL_SMELL_RAD: f32 = 5. * Self::TRAIL_SEP;
	const SPEED: f32 = 15.0;
	const TURN_SPEED: f32 = 180. * std::f32::consts::TAU / 360.;
	const EXPLORATION: f32 = 10.;
	const TRAIL_SEP: f32 = 15.0;
	const PREFER_STRAIGHT: f32 = 0.6;

	type Action;

	fn spawn(pos: Vector2<f32>, dir: Vector2<f32>) -> Self;
	fn next_plan(&self, ant: &Ant<Self>, world: &World, messenger: &Sender<Dispatch>) -> Self;
	fn action(&mut self, ant: &Ant<Self>, external: &External) -> (Vector2<f32>, Option<Self::Action>);
	fn texture(&self) -> Texture;
}
