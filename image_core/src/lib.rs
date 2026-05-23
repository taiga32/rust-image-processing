use image::{DynamicImage, ImageBuffer, ImageReader};
use minifb::{Key, Window, WindowOptions};

pub fn read_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

pub fn resize_image(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize(width, height, image::imageops::FilterType::Nearest)
}

pub fn image_to_grayscale(img: &DynamicImage) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    img.to_luma8()
}

pub fn display_image(img: &DynamicImage, img_gray: &ImageBuffer<image::Luma<u8>, Vec<u8>>) {
    let resized_img = resize_image(img, 400, 400);
    let rgb = resized_img.to_rgb8();
    let (w, h) = (resized_img.width() as usize, resized_img.height() as usize);
    let resized_gray =
        resize_image(&DynamicImage::ImageLuma8(img_gray.clone()), 400, 400).to_luma8();

    let mut buffer: Vec<u32> = rgb
        .pixels()
        .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32))
        .collect();

    buffer.extend(
        resized_gray
            .pixels()
            .map(|p| {
                let v = p[0] as u32;
                (v << 16) | (v << 8) | v
            })
            .collect::<Vec<u32>>(),
    );

    let total_h = h * 2;
    let mut window: Window = Window::new(
        "Preview - ESC to close",
        w,
        total_h,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, w, total_h).unwrap();
    }
}
