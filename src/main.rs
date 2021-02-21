use image;
use image::GenericImageView;
use std::env;

mod setting;
use setting::Settings;

fn main() {
    let mut args = env::args();
    // TODO: This is ugly FIXME
    let settings = match Settings::new(&mut args) {
        Some(val) => val,
        None => return,
    };

    uncolored_img_to_ascii(settings);
}

fn uncolored_img_to_ascii(x: Settings) {
    let img = if let Ok(val) = image::open(x.image_file) {
        val
    } else {
        return println!("Error: File Not Fount OR File Type Not Supported!");
    };

    //converting the image to greyscale
    let img = img
        .resize_exact(
            (img.width() / x.image_size) as u32,
            (img.height() / (x.image_size * 2) as u32) as u32,
            image::imageops::FilterType::Nearest,
        )
        .grayscale();

    //loop on every pixel in y and x of the image and calculate the brightness.
    for y in 0..img.height() {
        for x in 0..img.width() {
            let [r, g, b, _] = img.get_pixel(x, y).0;
            print_char(get_luminance(r, g, b));
        }
        println!();
    }
    println!();
}

fn print_char(lumi: f32) {
    // TODO: Move this to Settings module and accept a character array from the user.
    let table = [
        ' ', '.', ',', ':', ';', '\'', '"', '<', '>', 'i', '!', '(', ')', '[', ']', '(', ')', '{',
        '}', '*', '8', 'B', '%', '$', '#', '@',
    ];
    print!("{}", table[(lumi * (table.len() - 1) as f32) as usize]);
}

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get brightness of the pixel
fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    (r + g + b) / 255.0
}
