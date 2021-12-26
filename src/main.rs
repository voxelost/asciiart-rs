extern crate image;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use std::env;
use std::io;
use std::io::ErrorKind;

const GRAYSCALE_CHARS: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^'. ";

const IMG_SCALE: u32 = 1;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "no filename given"));
    }

    let img = ImageReader::open(args[1].clone())?.decode().unwrap();
    let output_str = str_from_image(img);

    println!("{}", output_str);
    Ok(())
}

fn str_from_image(img: DynamicImage) -> String {
    let width = img.width();
    let height = img.height();

    let step_x: u32 = 4 * IMG_SCALE;
    let step_y: u32 = 9 * IMG_SCALE;

    let mut output_str: String = String::new();

    for y in (0..height).step_by(step_y as usize) {
        for x in (0..width).step_by(step_x as usize) {
            let gs = get_average_brightness(&img, x, y, step_x, step_y);
            output_str.push(get_grayscale_char_for_u8_brightness(gs));
        }

        output_str.push('\n');
    }

    output_str
}

fn get_grayscale_char_for_u8_brightness(brightness: u8) -> char {
    let idx = brightness as i32 * GRAYSCALE_CHARS.len() as i32 / (std::u8::MAX as i32 + 1);
    GRAYSCALE_CHARS.chars().nth(idx as usize).unwrap()
}

fn get_average_brightness(
    img: &DynamicImage,
    coord_x: u32,
    coord_y: u32,
    offset_x: u32,
    offset_y: u32,
) -> u8 {
    let mut accumulator: u64 = 0;
    let mut counter: u64 = 0;

    let width = img.width();
    let height = img.height();

    let grayscale_img = img.grayscale();

    for y in coord_y..(coord_y + offset_y) {
        for x in coord_x..(coord_x + offset_x) {
            if x >= width || y >= height {
                continue;
            }

            let px = grayscale_img.get_pixel(x, y);

            accumulator += px.0[0] as u64;
            counter += 1;
        }
    }

    if counter == 0 {
        return 0;
    }

    (accumulator / counter) as u8
}
