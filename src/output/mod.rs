// pub fn render(width: u32, height: u32) {
//     let mut image = image::ImageBuffer::new(width, height);

//     for (x, y, pixel) in image.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//     }

//     image.save("output.png").unwrap();
// }
