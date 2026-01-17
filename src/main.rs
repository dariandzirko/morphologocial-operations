use std::{collections::binary_heap, range};

use image::{
    DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel,
    imageops::{self, BiLevel},
};

fn main() {
    let img = image::open("images/TEXT.png").unwrap();
}

fn erosion(img: DynamicImage, iterations: i32) -> DynamicImage {
    let mut return_img: DynamicImage = img;

    let kernel = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    for i in 0..iterations {
        return_img = return_img.filter3x3(&kernel);
    }

    let gray = return_img.to_luma8();

    let binary: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_fn(gray.width(), gray.height(), |x, y| {
            let pix = gray.get_pixel(x, y);
            if pix.0 > [254] {
                [255].into()
            } else {
                [0].into()
            }
        });

    return image::DynamicImage::ImageLuma8(binary);
}

fn dilation() {}
fn opening() {}
fn closing() {}
fn morphological_gradient() {}
fn black_hat() {}
fn white_hat() {}
