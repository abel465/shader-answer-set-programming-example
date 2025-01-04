#![no_std]

use push_constants::shader::*;
use shared::*;
use spirv_std::glam::*;
// use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &FragmentConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] _buffer: &mut [f32],
    output: &mut Vec4,
) {
    *output = (frag_coord.xy() / constants.size.as_vec2() / constants.zoom)
        .extend(0.0)
        .powf(2.2)
        .extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;
    *out_pos = pos.extend(0.0).extend(1.0);
}

#[spirv(compute(threads(16, 16)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] _gid: UVec3,
    #[spirv(push_constant)] _constants: &ComputeConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [f32],
) {
    buffer[0] = 1.0;
}
