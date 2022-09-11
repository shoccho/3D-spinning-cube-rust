pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub fn calculate_x(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.sin() * angle.y.sin() * angle.z.cos()
        - pos.z * angle.x.cos() * angle.y.sin() * angle.z.cos()
        + pos.y * angle.x.cos() * angle.z.sin()
        + pos.z * angle.x.sin() * angle.z.sin()
        + pos.x * angle.y.cos() * angle.z.cos();
}

pub fn calculate_y(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.cos() * angle.z.cos() + pos.z * angle.x.sin() * angle.z.cos()
        - pos.y * angle.x.sin() * angle.y.sin() * angle.z.sin()
        + pos.z * angle.x.cos() * angle.y.sin() * angle.z.sin()
        - pos.x * angle.x.cos() * angle.z.sin();
}

pub fn calculate_z(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.z * angle.x.cos() * angle.y.cos() - pos.y * angle.x.sin() * angle.y.cos()
        + pos.x * angle.y.sin();
}
