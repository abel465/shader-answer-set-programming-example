pub mod simplex;

use spirv_std::glam::*;

trait Mod289 {
    fn mod289(self) -> Self;
}

impl Mod289 for Vec2 {
    fn mod289(self) -> Self {
        self - (self * (1.0 / 289.0)).floor() * 289.0
    }
}

impl Mod289 for Vec3 {
    fn mod289(self) -> Self {
        self - (self * (1.0 / 289.0)).floor() * 289.0
    }
}

fn permute(x: Vec3) -> Vec3 {
    (((x * 34.0) + 10.0) * x).mod289()
}
