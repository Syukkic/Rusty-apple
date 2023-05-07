use std::fs;

use image::DynamicImage;

fn read_image(image_path: &str) -> DynamicImage {
    image::open(image_path).expect("Failed to open image")
}

fn convert_to_ascii(image: &DynamicImage, art_width: u32, art_height: u32, ascii_chars: &str) -> String {
    // Convert the image to grayscale
    let grayscale_image = image.to_luma8();

    // Calculate the scale factor for converting pixel values to ASCII characters
    let scale_factor = 255.0 / (ascii_chars.len() - 1) as f32;

    // Convert the image to ASCII art
    let mut ascii_art = String::new();
    for y in 0..art_height {
        for x in 0..art_width {
            // Map the grayscale pixel value to an ASCII character
            let pixel_value = grayscale_image.get_pixel(
                (x * grayscale_image.width() / art_width) as u32,
                (y * grayscale_image.height() / art_height) as u32,
            )[0] as f32;
            let ascii_index = (pixel_value / scale_factor) as usize;
            let ascii_char = ascii_chars.chars().nth(ascii_index).unwrap();

            ascii_art.push(ascii_char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}

fn main() {
    let frames_folder = "frames";
    let ascii_chars = "@%#*+=-:. ";

    let art_width = 100;
    fs::read_dir(frames_folder)
        .expect("Failed to read frames folder")
        .filter_map(Result::ok)
        .filter(|entry| {
            if let Some(extension) = entry.path().extension() {
                extension == "png"
            } else {
                false
            }
        }).for_each(|entry| {
            let entry_path = entry.path();
            let image = read_image(entry_path.to_str().unwrap());
            let art_height = (image.height() * art_width) / image.width();
            let ascii_art = convert_to_ascii(&image, art_width, art_height, &ascii_chars);
            println!("{}", ascii_art);
        });
}
