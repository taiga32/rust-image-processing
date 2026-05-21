use image::{DynamicImage, ImageBuffer, ImageReader};
use minifb::{Key, Window, WindowOptions};

pub fn read_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, image::imageops::FilterType::Nearest)
}

pub fn display_image(img: &DynamicImage) {
    let resized_img = resize_image(img, 800, 800);
    let rgb = resized_img.to_rgb8();
    let (w, h) = (resized_img.width() as usize, resized_img.height() as usize);

    let buffer: Vec<u32> = rgb
        .pixels()
        .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32))
        .collect();

    let mut window: Window = Window::new("Preview - ESC to close", w, h, WindowOptions::default())
        .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, w, h).unwrap();
    }
}

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
