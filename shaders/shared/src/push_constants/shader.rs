use super::{Bool, Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct FragmentConstants {
    pub size: Size,
    pub cursor: Vec2,
    pub prev_cursor: Vec2,
    pub time: f32,
    pub mouse_button_pressed: u32,
    pub zoom: f32,
    pub debug: Bool,
}

impl FragmentConstants {
    pub fn mem_size() -> usize {
        core::mem::size_of::<Self>()
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ComputeConstants {
    pub size: Size,
    pub time: f32,
    pub zoom: f32,
}

impl ComputeConstants {
    pub fn mem_size() -> usize {
        core::mem::size_of::<Self>()
    }
}
