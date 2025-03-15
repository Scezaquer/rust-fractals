use minifb::{Key, Window, WindowOptions};
use num::complex::Complex;
use rayon::prelude::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const RED: u32 = 0xFF0000;
const GREEN: u32 = 0x00FF00;
const BLUE: u32 = 0x0000FF;
const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

fn draw_square(buffer: &mut Vec<u32>, top_left: (usize, usize), bottom_right: (usize, usize), color: u32) {
    let (top, left) = top_left;
    let (bot, right) = bottom_right;

    for i in top..bot{
        for j in left..right{
            buffer[i * WIDTH + j] = color;
        }
    }
}

fn is_point_in_mandelbrot(x: f32, y: f32, max_iter: usize) -> Option<usize> {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..max_iter{
        let z_square = z*z;
        if (z+z_square).norm_sqr() > 4.0 { return Some(i) }
        z = z_square + c;
    }

    return None;
}

fn draw_mandelbrot(buffer: &mut Vec<u32>, top: f32, left: f32, right: f32, bot: f32, max_iter: usize) {
    let hstep = (right - left) / WIDTH as f32;
    let vstep = (top - bot) / HEIGHT as f32;

    buffer.par_chunks_mut(WIDTH).enumerate().for_each(|(i, row)| {
       let y = top - i as f32 * vstep;

       for (j, pixel) in row.iter_mut().enumerate() {
       
            let x = left + j as f32 * hstep;
            *pixel = if let Some(k) = is_point_in_mandelbrot(x, y, max_iter) {
                let t = k as f32 / max_iter as f32;
                ((WHITE as f32) * (1.0 - t) + ((BLACK as f32) * t)) as u32
            } else {
                BLACK
            }

       }
    });
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions::default();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);
    draw_mandelbrot(&mut buffer, 1.5, -2.0, 1.0, -1.5, 100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*
        let mut color: u32 = 0;
        for i in buffer.iter_mut() {
            *i = color;
            color += 1;
        }
        */
        
        
        // draw_square(&mut buffer, (50, 50), (100, 100), RED);  

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
