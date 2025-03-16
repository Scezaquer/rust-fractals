use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use num::complex::Complex;
use rayon::prelude::*;
use std::time::Instant;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const RED: u32 = 0xFF0000;
const GREEN: u32 = 0x00FF00;
const BLUE: u32 = 0x0000FF;
const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xFFFFFF;

const ITER: usize = 100;    // Max number of iteration of mandelbrot formula
const ZOOM: f64 = 1.005;    // Zoom factor (applied 60 times/s while right click is pressed)
const FRAMERATE: usize = 60;

fn draw_square(buffer: &mut Vec<u32>, top_left: (usize, usize), bottom_right: (usize, usize), color: u32) {
    let (top, left) = top_left;
    let (bot, right) = bottom_right;

    (top..bot).for_each(|i| {
        for j in left..right{
            buffer[i * WIDTH + j] = color;
        }
    });
}


fn is_point_in_mandelbrot(x: f64, y: f64, max_iter: usize) -> Option<usize> {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..max_iter{
        z = z*z + c;
        if z.norm_sqr() > 4.0 { return Some(i) }
    }

    return None;
}

fn draw_mandelbrot(buffer: &mut Vec<u32>, top: f64, left: f64, right: f64, bot: f64, max_iter: usize) {
    let hstep = (right - left) / WIDTH as f64;
    let vstep = (top - bot) / HEIGHT as f64;

    buffer.par_chunks_mut(WIDTH).enumerate().for_each(|(i, row)| {
       let y = top - i as f64 * vstep;

       for (j, pixel) in row.iter_mut().enumerate() {
       
            let x = left + j as f64 * hstep;
            *pixel = if let Some(k) = is_point_in_mandelbrot(x, y, max_iter) {
                let t = k as f64 / max_iter as f64;
                ((WHITE as f64) * (1.0 - t) + ((BLACK as f64) * t)) as u32
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
    window.set_target_fps(FRAMERATE);

    let mut top = 1.5;
    let mut left = -2.0;
    let mut right = 1.0;
    let mut bot = -1.5;

    let mut center_x = (left + right) / 2.0;
    let mut center_y = (top + bot) / 2.0;

    let t = 0.1;

    draw_mandelbrot(&mut buffer, top, left, right, bot, ITER);

    let mut now = Instant::now();
    let mut framerate = FRAMERATE as f64;
    let mut zoom = ZOOM;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*
        let mut color: u32 = 0;
        for i in buffer.iter_mut() {
            *i = color;
            color += 1;
        }
        */
        
        if window.get_mouse_down(MouseButton::Left) {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                // Zoom in
                let last_height = top - bot;
                let last_width = right - left;

                center_x = center_x * (1.0 - t) + (left + (mouse.0 as f64) / (WIDTH as f64) * last_width) * t;
                center_y = center_y * (1.0 - t) + (top - (mouse.1 as f64) / (HEIGHT as f64) * last_height) * t;
                top = center_y + last_height / 2.0 / zoom;
                bot = center_y - last_height / 2.0 / zoom;
                left = center_x - last_width / 2.0 / zoom;
                right = center_x + last_width / 2.0 / zoom;

                draw_mandelbrot(&mut buffer, top, left, right, bot, ITER);
            });
        } else if window.get_mouse_down(MouseButton::Right) {
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                // Zoom out
                let last_height = top - bot;
                let last_width = right - left;

                center_x = center_x * (1.0 - t) + (left + (mouse.0 as f64) / (WIDTH as f64) * last_width) * t;
                center_y = center_y * (1.0 - t) + (top - (mouse.1 as f64) / (HEIGHT as f64) * last_height) * t;
                top = center_y + last_height / 2.0 * zoom;
                bot = center_y - last_height / 2.0 * zoom;
                left = center_x - last_width / 2.0 * zoom;
                right = center_x + last_width / 2.0 * zoom;

                draw_mandelbrot(&mut buffer, top, left, right, bot, ITER);
            });
        }  
        
        //draw_square(&mut buffer, (50, 50), (100, 100), RED);  

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        // Adjust zoom speed to be consistent even if framerate drops
        framerate = 1.0 / now.elapsed().as_secs_f64();
        now = Instant::now();
        zoom = ZOOM.powf(FRAMERATE as f64 / framerate);
    }
}
