extern crate image;

use image::GenericImageView;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("img/input/progressive.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img
    .resize(160, 90, image::imageops::Lanczos3)
    .save("img/output/progressive.jpg")
    .unwrap();
}