mod animation;

use super::glsl::*;

use cgmath::*;
use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

pub use animation::Animation;

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

pub struct External {
    pub texture_map: TextureMap,
    pub size: (u32, u32),
    pub camera: Camera,
    pub now: Instant,
    pub delta: f32,
}

impl External {
    pub fn refresh(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.now).as_secs_f32();
        self.now = now;
    }

    pub fn view_dims(&self) -> Vector2<f32> {
        let k = 2. * self.camera.scale;

        vec2(k * self.aspect(), k)
    }

    pub fn point_in_view(&self, p: Vector2<f32>) -> bool {
        let diff = self.camera.pos - p;
        let k = self.camera.scale;
        diff.x.abs() < k * self.aspect() && diff.y.abs() < k
    }

    pub fn visible(&self, instance: Instance) -> bool {
        let (cx, cy) = self.camera.pos.into();
        let GLvec2(px, py) = instance.position;
        let GLvec2(sx, sy) = instance.scale;

        //maximal possible distance, since instances may be rotated
        let max = sx.hypot(sy);

        let (dx, dy) = self.view_dims().into();

        instance.screen_relative == GLbool::True
            || ((px - cx).abs() < max + dx / 2. && (py - cy).abs() < max + dy / 2.)
    }

    pub fn clip(&self, out: &mut Vec<Instance>, instance: Instance) {
        //clip unseen instances
        if self.visible(instance) {
            out.push(instance);
        }
    }

    pub fn instance(&self, texture: Texture) -> Instance {
        self.texture_map[&texture]
    }

    pub fn aspect(&self) -> f32 {
        self.size.0 as f32 / self.size.1 as f32
    }
}

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Debug, Eq, Clone, Copy)]
pub enum Texture {
    Ant
}

impl Texture {
    pub fn frame_count(&self) -> u32 {
        match self {
            _ => 1,
        }
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Instance {
    pub color_tint: GLvec4,
    pub texture: GLvec4,
    pub scale: GLvec2,
    pub position: GLvec2,
    pub rotation: GLfloat,
    pub screen_relative: GLbool,
}

impl Instance {
    pub fn scale(self, x: f32, y: f32) -> Self {
        Self {
            scale: GLvec2(x * self.scale.0, y * self.scale.1),
            ..self
        }
    }

    pub fn nth_frame(self, n: u32, out_of: u32) -> Self {
        let GLvec4(ulx, uly, lrx, lry) = self.texture;
        let shift = (lry - uly) / out_of as f32;
        let starty = uly + n as f32 * shift;

        const ANTI_BLEED_MULTIPLIER: f32 = 10. * f32::EPSILON;
        let anti_bleed = shift * ANTI_BLEED_MULTIPLIER;

        Self {
            texture: GLvec4(ulx, starty + anti_bleed, lrx, starty + shift - anti_bleed),
            ..self
        }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            color_tint: GLvec4(1.0, 1.0, 1.0, 1.0),
            texture: GLvec4(0.0, 0.0, 1.0, 1.0),
            scale: GLvec2(1.0, 1.0),
            position: GLvec2(0.0, 0.0),
            rotation: GLfloat(0.0),
            screen_relative: GLbool::False,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Vector2<f32>,
    pub scale: f32,
}

impl Camera {
    pub fn proj(&self, aspect: f32) -> Matrix4<f32> {
        ortho(
            self.pos.x - aspect * self.scale,
            self.pos.x + aspect * self.scale,
            self.pos.y - self.scale,
            self.pos.y + self.scale,
            -100.,
            100.,
        )
    }
}
