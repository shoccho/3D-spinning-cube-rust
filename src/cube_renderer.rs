use crate::math::*;
use crate::renderer::*;
use std::{thread, time, time::Instant};
pub struct CubeRenderer {
    cube_width: usize,
    renderer: Renderer,
    angle: Vec3,
    dt: f32,
    fps: i32,
}
impl CubeRenderer {
    pub fn new(width: usize, renderer: Renderer, fps: i32) -> Self {
        let angle = Vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        };
        Self {
            cube_width: width,
            renderer,
            angle,
            dt: 0.2f32,
            fps,
        }
    }
    pub fn render(&mut self) {
        let bg = ' ' as u8;
        let z = -1f32 * self.cube_width as f32;
        loop {
            self.renderer.buffer.fill(bg);
            self.renderer.z_buffer.fill(0f32);
            let start = Instant::now();

            let mut cube_x = -1f32 * self.cube_width as f32;
            while cube_x < self.cube_width as f32 {
                let mut cube_y = -1f32 * self.cube_width as f32;
                while cube_y < self.cube_width as f32 {
                    cube_y += self.dt;
                    let surface = &Vec3 {
                        x: -cube_x,
                        y: cube_y,
                        z,
                    };
                    let ch = '^' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);

                    let surface = &Vec3 {
                        x: cube_x,
                        y: cube_y,
                        z: -z,
                    };
                    let ch = '$' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);

                    let surface = &Vec3 {
                        x: -z,
                        y: cube_y,
                        z: -cube_x,
                    };
                    let ch = ',' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);

                    let surface = &Vec3 {
                        x: z,
                        y: cube_y,
                        z: cube_x,
                    };
                    let ch = '%' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);

                    let surface = &Vec3 {
                        x: cube_x,
                        y: -z,
                        z: -cube_y,
                    };
                    let ch = '/' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);

                    let surface = &Vec3 {
                        x: cube_x,
                        y: z,
                        z: cube_y,
                    };
                    let ch = '#' as u8;
                    self.renderer
                        .calculate_surface_pixel(surface, &self.angle, &ch);
                }
                cube_x += self.dt;
            }
            self.renderer.render();
            self.angle.x += 0.05;
            self.angle.y += 0.05;
            self.angle.z += 0.01;

            let elapsed = start.elapsed();
            let frame_time = 1000 / self.fps;

            let wait_time =
                time::Duration::from_millis((frame_time - elapsed.as_millis() as i32) as u64);
            thread::sleep(wait_time);
        }
    }
}
