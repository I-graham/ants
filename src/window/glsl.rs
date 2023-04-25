use cgmath::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uniform {
    pub ortho: Matrix4<f32>,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLint(pub i32);

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLbool {
    False,
    True,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLfloat(pub f32);

impl From<f32> for GLfloat {
    fn from(f: f32) -> Self {
        GLfloat(f)
    }
}

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec2(pub f32, pub f32);

impl From<(f32, f32)> for GLvec2 {
    fn from((f1, f2): (f32, f32)) -> Self {
        GLvec2(f1, f2)
    }
}

impl From<(i32, i32)> for GLvec2 {
    fn from((i1, i2): (i32, i32)) -> Self {
        GLvec2(i1 as f32, i2 as f32)
    }
}

impl From<Vector2<f32>> for GLvec2 {
    fn from(v: Vector2<f32>) -> Self {
        GLvec2(v.x, v.y)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec3(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for GLvec3 {
    fn from((f1, f2, f3): (f32, f32, f32)) -> Self {
        GLvec3(f1, f2, f3)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec4(pub f32, pub f32, pub f32, pub f32);

impl GLvec4 {
    pub fn rgba(&self) -> Self {
        let GLvec4(r, g, b, a) = *self;
        let f = |i| i / 255.;
        Self(f(r), f(g), f(b), f(a))
    }
}

impl From<(f32, f32, f32, f32)> for GLvec4 {
    fn from((f1, f2, f3, f4): (f32, f32, f32, f32)) -> Self {
        GLvec4(f1, f2, f3, f4)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLdvec4(pub f64, pub f64, pub f64, pub f64);

impl From<(f64, f64, f64, f64)> for GLdvec4 {
    fn from((f1, f2, f3, f4): (f64, f64, f64, f64)) -> Self {
        GLdvec4(f1, f2, f3, f4)
    }
}
