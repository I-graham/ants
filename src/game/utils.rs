use cgmath::*;

pub fn probability(p: f32) -> bool {
    random() < p
}

pub fn random() -> f32 {
    rand_in(0., 1.)
}

pub fn rand_in(lo: f32, hi: f32) -> f32 {
    lo + (hi - lo) * rand::random::<f32>()
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
