use super::*;
use cgmath::*;

pub mod ui;
mod freelist;
mod grid;
mod relaxed;

pub use freelist::*;
pub use grid::*;
pub use relaxed::*;

pub fn unit_in_dir(deg: f32) -> Vector2<f32> {
	vec2(deg.sin(), deg.cos())
}

pub fn angle(v: Vector2<f32>) -> f32 {
	std::f32::consts::FRAC_PI_2 - v.y.atan2(v.x)
}

pub fn dist((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
	(x1 - x2).hypot(y1 - y2)
}

pub fn unit_toward(from: Vector2<f32>, to: Vector2<f32>) -> Vector2<f32> {
	if to != from {
		(to - from).normalize()
	} else {
		vec2(0., 0.)
	}
}

pub fn probability(p: f32) -> bool {
	random() < p
}

pub fn random() -> f32 {
	rand::random::<f32>()
}

pub fn rand_in(lo: f32, hi: f32) -> f32 {
	lo + (hi - lo) * random()
}

pub fn rand_in2d(lo: f32, hi: f32) -> Vector2<f32> {
	vec2(rand_in(lo, hi), rand_in(lo, hi))
}

pub fn snap_to_grid(p: Vector2<f32>, (cellx, celly): (f32, f32)) -> Vector2<i32> {
	vec2(
		(cellx * (p.x / cellx).round()) as i32,
		(celly * (p.y / celly).round()) as i32,
	)
}
