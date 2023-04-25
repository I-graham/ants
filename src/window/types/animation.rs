use super::*;

type Curve = fn(f32) -> f32;

#[derive(Clone)]
pub struct Animation {
    pub start: Instant,
    pub texture: Texture,
    pub duration: f32,
    pub curve: Curve,
    pub repeat: Option<f32>, //None means repeat forever
}

use std::f32::consts::*;
impl Animation {
    pub const LINEAR: Curve = |f| f;
    pub const FIRST: Curve = |_| 0.;
    pub const LAST: Curve = |_| 1.;
    pub const REVERSE: Curve = |f| 1. - f;
    pub const SIN: Curve = |f| (1. - (f * PI).cos()) / 2.;
    pub const SIN_SQ: Curve = |f| Self::SIN(f).powf(2.);
    pub const REV_SIN_SQ: Curve = |f| Self::SIN(1.0 - f).powf(2.);
    pub const SIN_BOUNCE: Curve = |f| Self::SIN(2. * f);

    pub fn new(
        texture: Texture,
        duration: f32,
        curve: fn(f32) -> f32,
        repeat: Option<f32>,
    ) -> Self {
        Self {
            start: Instant::now(),
            texture,
            duration,
            curve,
            repeat,
        }
    }

    pub fn frame(&self, external: &External) -> Instance {
        let elapsed = self.age(external.now);
        let frames = self.texture.frame_count();

        let reps_elapsed = elapsed / self.duration;

        let proportion = match self.repeat {
            Some(reps) if elapsed > reps * self.duration => reps - f32::EPSILON,
            _ => self.repeat.unwrap_or(reps_elapsed).min(reps_elapsed),
        };

        let frame = (frames as f32 * (self.curve)(proportion.fract())) as u32;

        external
            .instance(self.texture)
            .nth_frame(frame.clamp(0, frames - 1), frames)
    }

    pub fn finished(&self, now: Instant) -> bool {
        matches!(self.repeat, Some(reps) if self.age(now) > reps * self.duration)
    }

    pub fn age(&self, now: Instant) -> f32 {
        now.duration_since(self.start).as_secs_f32()
    }

    pub fn restart(&mut self) {
        self.start = Instant::now()
    }
}
