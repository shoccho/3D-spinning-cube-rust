use crate::math::*;
pub struct Renderer {
    pub z_buffer: Vec<f32>,
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub offset: isize,
}
impl Renderer {
    pub fn new(height: usize, width: usize, offset: isize) -> Self {
        Self {
            z_buffer: vec![0f32; height * width],
            buffer: vec![' ' as u8; height * width],
            width,
            height,
            offset,
        }
    }
    pub fn render(&self) {
        print!("{}[1;1H", 27 as char);
        for k in 0..&self.width * &self.height {
            if k % &self.width == 0 {
                println!();
            } else {
                print!("{}", *self.buffer.get(k).unwrap() as char)
            }
        }
    }
    pub fn calculate_surface_pixel(&mut self, pos: &Vec3, angle: &Vec3, ch: &u8) {
        let mx = calculate_x(pos, angle);
        let my = calculate_y(pos, angle);
        let mz = calculate_z(pos, angle) + 100f32;
        let ooz = 1f32 / mz;
        let xp = self.width as isize / 2 + self.offset + (50f32 * ooz * mx * 2f32) as isize;
        let yp = self.height as isize / 2 + (50f32 * ooz * my) as isize;
        let idx = xp as usize + (yp as usize * self.width);
        if idx < (self.width * self.height) {
            if ooz > self.z_buffer[idx] {
                self.z_buffer[idx] = ooz;
                self.buffer[idx] = *ch;
            }
        }
    }
}
