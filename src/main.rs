use std::task::ready;

use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Pixel, error,
    imageops::{self, BiLevel},
};

fn main() {
    let img = image::open("images/TEXT.png").unwrap();
}

fn erosion(img: DynamicImage, iterations: i32) -> DynamicImage {
    let mut return_img: DynamicImage = img;

    let kernel = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    for _ in 0..iterations {
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

fn dilation(img: DynamicImage, iterations: i32) -> DynamicImage {
    let mut return_img: DynamicImage = img;

    let kernel = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    for _ in 0..iterations {
        return_img = return_img.filter3x3(&kernel);
    }

    let gray = return_img.to_luma8();

    let binary = ImageBuffer::from_fn(gray.width(), gray.height(), |x, y| {
        let pix = gray.get_pixel(x, y);
        if pix.0 > [1] {
            [255].into()
        } else {
            [0].into()
        }
    });

    return image::DynamicImage::ImageLuma8(binary);
}

fn opening(img: DynamicImage, iterations: i32) -> DynamicImage {
    let mut return_img: DynamicImage = img;

    return_img = erosion(return_img, iterations);
    return_img = dilation(return_img, iterations);

    return return_img;
}

fn closing(img: DynamicImage, iterations: i32) -> DynamicImage {
    let mut return_img: DynamicImage = img;

    return_img = dilation(return_img, iterations);
    return_img = erosion(return_img, iterations);

    return_img
}

fn morphological_gradient(img: DynamicImage, iterations: i32) {
    let return_img: DynamicImage = img;

    let erosion_img = erosion(return_img, iterations);
    let dilation_img = dilation(return_img, iterations);

    return dilation_img - erosion_img;
}

fn black_hat() {}
fn white_hat() {}
