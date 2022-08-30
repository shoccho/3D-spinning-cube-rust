const CUBE_WIDTH: usize = 20;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;

const INC: f32 = 0.5;
const HORIZONTAL_OFFSET: isize = -2 * CUBE_WIDTH as isize;
const K1: f32 = 40f32;
use std::{thread, time};


fn calculateX(i: &f32, j: &f32, k: &f32, A: &f32, B: &f32, C: &f32) -> f32 {
    return *j * A.sin() * B.sin() * C.cos() - *k * A.cos() * B.sin() * C.cos()
        + *j * A.cos() * C.sin()
        + *k * A.sin() * C.sin()
        + *i * B.cos() * C.cos();
}

fn calculateY(i: &f32, j: &f32, k: &f32, A: &f32, B: &f32, C: &f32) -> f32 {
    return *j * A.cos() * C.cos() + *k * A.sin() * C.cos() - *j * A.sin() * B.sin() * C.sin()
        + *k * A.cos() * B.sin() * C.sin()
        - *i * A.cos() * C.sin();
}

fn calculateZ(i: &f32, j: &f32, k: &f32, A: &f32, B: &f32, C: &f32) -> f32 {
    return *k * A.cos() * B.cos() - *j * A.sin() * B.cos() + *i * B.sin();
}
fn calculateSurfaceChar(
    x: &f32,
    y: &f32,
    z: &f32,
    ch: &u8,
    A: &f32,
    B: &f32,
    C: &f32,
    zBuffer: &mut [f32],
    buffer: &mut [u8],
) {
    let mx = calculateX(x, y, z, A, B, C);
    let my = calculateY(x, y, z, A, B, C);
    let mz = calculateZ(x, y, z, A, B, C)+100f32;
    let ooz = 1f32 / mz;
    let xp = WIDTH as isize / 2 + HORIZONTAL_OFFSET + (K1 * ooz * mx * 2f32) as isize ;
    let yp = HEIGHT as isize / 2 + (K1 * ooz * my) as isize;
    let idx = xp as usize + yp as usize * WIDTH;
    if idx >= 0 && idx < WIDTH * HEIGHT {
        if ooz > zBuffer[idx] {
            zBuffer[idx] = ooz;
            buffer[idx] = *ch;
        }
    }
}
fn main() {
    let mut zBuffer = [0f32; WIDTH * HEIGHT];
    let mut buffer = [0u8; WIDTH * HEIGHT];

    let bg = ' ' as u8;
    let mut A: f32 = 0f32;
    let mut B: f32 = 0f32;
    let mut C: f32 = 0f32;
    print!("\x1b[2J");
    let z = -1f32 * CUBE_WIDTH as f32;
    loop {
        buffer.fill(bg);
        zBuffer.fill(0f32);
        let mut cubeX = -1f32 * CUBE_WIDTH as f32;
        while cubeX < CUBE_WIDTH as f32 {
            let mut cubeY = -1f32 * CUBE_WIDTH as f32;
            while cubeY < CUBE_WIDTH as f32 {
                cubeY += INC;

                calculateSurfaceChar(
                    &cubeX,
                    &cubeY,
                    &z,
                    &('#' as u8),
                    &A,
                    &B,
                    &C,
                    &mut zBuffer,
                    &mut buffer,
                );
                calculateSurfaceChar(
                    &z,
                    &cubeY,
                    &cubeX,
                    &('$' as u8),
                    &A,
                    &B,
                    &C,
                    &mut zBuffer,
                    &mut buffer,
                );
               calculateSurfaceChar(
                     &-z,
                    &cubeY,
                    &-cubeX,
                    &('#' as u8),
                    &A,
                    &B,
                    &C,
                    &mut zBuffer,
                    &mut buffer,
                )
            }
            cubeX += INC;
        }
        print!("\x1b[H");
        for k in 0..WIDTH * HEIGHT {
            if k % WIDTH == 0 {
                println!();
            } else {
                print!("{}", buffer[k] as char)
            }
        }
        A += 0.05;
        B += 0.05;
        C += 0.01;
        let ten_millis = time::Duration::from_millis(80*2);
thread::sleep(ten_millis);
    }
}
