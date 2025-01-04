//
//     Description : Array and textureless GLSL 2D simplex noise function, converted to rust
// Original Author : Ian McEwan, Ashima Arts.
//         License : Copyright (C) 2011 Ashima Arts. All rights reserved.
//                   Distributed under the MIT License. See LICENSE file.
//                   https://github.com/ashima/webgl-noise
//                   https://github.com/stegu/webgl-noise
//

use super::*;
use spirv_std::glam::*;

pub fn noise(v: Vec2) -> f32 {
    const C: Vec4 = vec4(
        0.211324865405187,  // (3.0 - sqrt(3.0)) / 6.0
        0.366025403784439,  // 0.5 * (sqrt(3.0) - 1.0)
        -0.577350269189626, // -1.0 + 2.0 * C.x
        0.024390243902439,  // 1.0 / 41.0
    );

    // First corner
    let i = (v + v.dot(C.yy())).floor();
    let x0 = v - i + i.dot(C.xx());

    // Other corners
    let i1 = if x0.x > x0.y { Vec2::X } else { Vec2::Y };
    let mut x12 = x0.xyxy() + C.xxzz();
    x12.x -= i1.x;
    x12.y -= i1.y;

    // Permutations
    let i = i.mod289(); // Avoid truncation effects in permutation
    let p = permute(permute(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0));

    let m =
        (0.5 - vec3(x0.dot(x0), x12.xy().dot(x12.xy()), x12.zw().dot(x12.zw()))).max(Vec3::ZERO);
    let m = m * m * m * m;

    // Gradients
    let x = 2.0 * (p * C.www()).fract() - 1.0;
    let h = x.abs() - 0.5;
    let ox = (x + 0.5).floor();
    let a0 = x - ox;

    // Normalise gradients implicitly by scaling m
    let m = m * (1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h));

    // Compute final noise value at P
    let gg = a0.yz() * x12.xz() + h.yz() * x12.yw();
    let g = vec3(a0.x * x0.x + h.x * x0.y, gg.x, gg.y);
    130.0 * m.dot(g)
}
