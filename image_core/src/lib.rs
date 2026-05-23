use image::{DynamicImage, ImageReader};
use minifb::{Key, Window, WindowOptions};

pub fn read_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, image::imageops::FilterType::Nearest)
}

pub fn image_to_grayscale(img: &DynamicImage) -> DynamicImage {
    let rgb_image = img.to_rgb8();
    let width = rgb_image.width();
    let height = rgb_image.height();
    let mut gray_image = image::GrayImage::new(width, height);
    // gray scale``
    for i in 0..width {
        for j in 0..height {
            let pixel = rgb_image.get_pixel(i, j);
            let gray_pixel = ((pixel[0] as f32 * 0.2126)
                + (pixel[1] as f32 * 0.7152)
                + (pixel[2] as f32 * 0.0722)) as u8;
            gray_image.put_pixel(i, j, image::Luma([gray_pixel]));
        }
    }
    DynamicImage::ImageLuma8(gray_image)
}

pub fn display_images(images: &[&DynamicImage]) {
    let w = images[0].width();
    let h_total: u32 = images.iter().map(|img| img.height()).sum();
    let mut buffer: Vec<u32> = Vec::new();
    for img in images {
        buffer.extend(
            img.to_rgb8()
                .pixels()
                .map(|p| {
                    let v = p[0] as u32;
                    (v << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
                })
                .collect::<Vec<u32>>(),
        );
    }
    let mut window: Window = Window::new(
        "Preview - ESC to close",
        w as usize,
        h_total as usize,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, w as usize, h_total as usize)
            .unwrap();
    }
}
