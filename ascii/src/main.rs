// Cargo.toml: image = "0.25"
use image::{GenericImageView, imageops::FilterType};

fn main() {
    // <-- change this to any image file on your machine
    let path = "src/spidery.jpg";

    // Load the image from disk
    let img = image::open(path).expect("Failed to open image");

    // Shrink it down — ASCII art needs far fewer "pixels" than a real photo,
    // and terminal characters are roughly twice as tall as they are wide,
    // so we squash the height a bit to keep proportions looking right.
    let (width, height) = img.dimensions();
    let new_width = 100;
    let new_height = (height as f32 / width as f32 * new_width as f32 * 0.5) as u32;
    let resized = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    // Characters ordered from "darkest look" to "lightest look"
    let ramp: Vec<char> = " .:-=+*#%@".chars().collect();

    // Convert to grayscale so we only need one brightness value per pixel
    let gray = resized.to_luma8();

    for y in 0..gray.height() {
        let mut line = String::new();
        for x in 0..gray.width() {
            let brightness = gray.get_pixel(x, y)[0]; // 0 (black) to 255 (white)
            let index = (brightness as usize * (ramp.len() - 1)) / 255;
            line.push(ramp[index]);
        }
        println!("{}", line);
    }
}
