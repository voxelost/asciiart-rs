extern crate bmp;

use bmp::Image;
use std::env;

const GRAYSCALE_CHARS: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^'. ";

const IMG_SCALE: u32 = 1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No filename given!");
    }

    let img = bmp::open(args[1].clone()).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    let output_str = str_from_image(img);

    println!("{}", output_str);
}

fn str_from_image(img: Image) -> String {
    let (size_x, size_y) = img.coordinates().last().unwrap();

    let step_x: u32 = 4 * IMG_SCALE;
    let step_y: u32 = 9 * IMG_SCALE;

    let mut output_str: String = String::new();

    for y in (0..size_y).step_by(step_y as usize) {
        for x in (0..size_x).step_by(step_x as usize) {
            let gs = get_average_brightness(&img, x, y, step_x, step_y);

            let idx = gs as i32 * GRAYSCALE_CHARS.len() as i32 / (std::u8::MAX as i32 + 1);

            output_str.push(GRAYSCALE_CHARS.chars().nth(idx as usize).unwrap());
        }

        output_str.push('\n');
    }

    output_str
}

fn get_average_brightness(img: &Image, coord_x: u32, coord_y: u32, step_x: u32, step_y: u32) -> u8 {
    let mut accumulator: u64 = 0;
    let mut counter: u64 = 0;
    let (size_x, size_y) = img.coordinates().last().unwrap();

    for y in coord_y..(coord_y + step_y) {
        for x in coord_x..(coord_x + step_x) {
            if x > size_x || y > size_y {
                continue;
            }

            let px = img.get_pixel(x, y);

            accumulator += rgb_to_grayscale(px.r, px.g, px.b) as u64;
            counter += 1;
        }
    }

    if counter == 0 {
        return 0;
    }

    (accumulator / counter) as u8
}

fn rgb_to_grayscale(r: u8, g: u8, b: u8) -> u8 {
    (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u8
}
