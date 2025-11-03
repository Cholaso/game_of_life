use crate::miniquad::conf::Icon;
    
use image::ImageReader as ImageReader;
use image::imageops::FilterType;

pub fn load_icon(path: &str) -> Icon {
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    
    let big: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::imageops::resize(&img, 64, 64, FilterType::Triangle);
    let mut big_bytes: [u8; 16384] = [0u8; 16384];
    big_bytes.copy_from_slice(&big.into_raw());

    let medium: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::imageops::resize(&img, 32, 32, FilterType::Triangle);
    let mut medium_bytes: [u8; 4096] = [0u8; 4096];
    medium_bytes.copy_from_slice(&medium.into_raw());

    let small: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::imageops::resize(&img, 16, 16, FilterType::Triangle);
    let mut small_bytes: [u8; 1024] = [0u8; 1024];
    small_bytes.copy_from_slice(&small.into_raw());

    Icon {
        small: small_bytes,
        medium: medium_bytes,
        big: big_bytes,
    }
}