use bytemuck::{Pod, Zeroable};
use spirv_std::glam;

pub mod shader;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn as_vec2(self) -> glam::Vec2 {
        glam::vec2(self.width as f32, self.height as f32)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl From<winit::dpi::PhysicalSize<u32>> for Size {
    fn from(winit::dpi::PhysicalSize { width, height }: winit::dpi::PhysicalSize<u32>) -> Self {
        Self { width, height }
    }
}

#[derive(Copy, Clone, Pod, Zeroable, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl From<glam::Vec2> for Vec2 {
    fn from(glam::Vec2 { x, y }: glam::Vec2) -> Self {
        Self { x, y }
    }
}

impl Into<glam::Vec2> for Vec2 {
    fn into(self) -> glam::Vec2 {
        glam::vec2(self.x, self.y)
    }
}

#[derive(Copy, Clone, Pod, Zeroable, PartialEq)]
#[repr(C)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl From<glam::UVec2> for UVec2 {
    fn from(glam::UVec2 { x, y }: glam::UVec2) -> Self {
        Self { x, y }
    }
}

impl Into<glam::UVec2> for UVec2 {
    fn into(self) -> glam::UVec2 {
        glam::UVec2::new(self.x, self.y)
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl From<glam::Vec3> for Vec3 {
    fn from(glam::Vec3 { x, y, z }: glam::Vec3) -> Self {
        Self { x, y, z }
    }
}

impl Into<glam::Vec3> for Vec3 {
    fn into(self) -> glam::Vec3 {
        glam::vec3(self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<glam::Vec4> for Vec4 {
    fn from(v: glam::Vec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl Into<glam::Vec4> for Vec4 {
    fn into(self) -> glam::Vec4 {
        glam::vec4(self.x, self.y, self.z, self.w)
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<glam::Quat> for Quat {
    fn from(q: glam::Quat) -> Self {
        Self {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }
}

impl Into<glam::Quat> for Quat {
    fn into(self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl From<glam::Mat4> for Mat4 {
    fn from(
        glam::Mat4 {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }: glam::Mat4,
    ) -> Self {
        Self {
            x_axis: x_axis.into(),
            y_axis: y_axis.into(),
            z_axis: z_axis.into(),
            w_axis: w_axis.into(),
        }
    }
}

impl Into<glam::Mat4> for Mat4 {
    fn into(self) -> glam::Mat4 {
        glam::Mat4::from_cols(
            self.x_axis.into(),
            self.y_axis.into(),
            self.z_axis.into(),
            self.w_axis.into(),
        )
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Bool {
    pub value: u32,
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        Self { value: b as u32 }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.value != 0
    }
}
