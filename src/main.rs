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
            (img.width() / 9) as u32,
            (img.height() / 16) as u32,
            image::imageops::FilterType::Nearest,
        )
        .grayscale();

    //loop on every pixel in y and x of the image and calculate the brightness.
    for y in 0..img.height() {
        for x in 0..img.width() {
            let [a, b, c, _] = img.get_pixel(x, y).0;
            print_char(get_brightness(a, b, c));
        }
        println!();
    }
    println!();
}

fn print_char(x: u16) {
    //NOTE: im lazy asshole, i need to orginize the table, because the commented ugly 'match' below gives better details.
    let table = [
        ' ', '.', ',', ':', ';', '\'', '"', '<', '>', 'i', '!', '(', ')', '[', ']', '(', ')', '{',
        '}', '*', '8', 'B', '%', '$', '#', '@',
    ];
    print!(
        "{}",
        table[(x as usize / ((255 * 4) as usize / table.len() - 1)) as usize]
    );

    // match x {
    //     0..=10 => print!(" "),
    //     11..=20 => print!("."),
    //     21..=30 => print!(","),
    //     31..=40 => print!(":"),
    //     41..=50 => print!("_"),
    //     51..=60 => print!("-"),
    //     61..=70 => print!(";"),
    //     71..=80 => print!(":"),
    //     81..=90 => print!("!"),
    //     91..=100 => print!("i"),
    //     101..=110 => print!("("),
    //     111..=120 => print!(")"),
    //     121..=130 => print!("{{"),
    //     131..=140 => print!("}}"),
    //     141..=150 => print!("*"),
    //     151..=160 => print!("J"),
    //     161..=170 => print!("L"),
    //     171..=180 => print!("M"),
    //     181..=190 => print!("K"),
    //     191..=200 => print!("E"),
    //     201..=210 => print!("R"),
    //     211..=220 => print!("H"),
    //     221..=230 => print!("$"),
    //     231..=240 => print!("8"),
    //     _ => print!("@"),
    // }
}

// calculate RGB values to get brightness of the pixel
fn get_brightness(a: u8, b: u8, c: u8) -> u16 {
    ((a as u16 + b as u16 + c as u16) / 3) as u16
}
