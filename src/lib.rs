//! This crate provides native rust implementations of
//! image encoders and decoders and basic image manipulation
//! functions.

#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(missing_copy_implementations)]
#![cfg_attr(test, feature(test))]

extern crate byteorder;
extern crate num;
#[macro_use]
extern crate enum_primitive;
#[cfg(test)]
extern crate test;

pub use color::ColorType::{
    self,
    Gray,
    RGB,
    Palette,
    GrayA,
    RGBA
};

pub use color::{
    Luma,
    LumaA,
    Rgb,
    Rgba
};

pub use image::{
    ImageDecoder,
    ImageError,
    ImageResult,
    SubImage,
    GenericImage,
    // Iterators
    Pixels,
    MutPixels
};

pub use imageops::FilterType::{
    self,
    Triangle,
    Nearest,
    CatmullRom,
    Gaussian,
    Lanczos3
};

pub use image::ImageFormat::{
    self,
    PNG,
    JPEG,
    GIF,
    WEBP,
    PPM,
    BMP,
    ICO
};

pub use buffer::{
    Pixel,
    ConvertBuffer,
    // Image types
    ImageBuffer,
    RgbImage,
    RgbaImage,
    GrayImage,
    GrayAlphaImage
};

// Traits
pub use traits::Primitive;

// Opening and loading images
pub use dynimage::{
    open,
    load,
    load_from_memory,
    load_from_memory_with_format,
    save_buffer
};

#[cfg(feature = "png_codec")]
pub use dynimage::load_png;
#[cfg(feature = "gif_codec")]
pub use dynimage::load_gif;
#[cfg(feature = "jpeg")]
pub use dynimage::load_jpeg;
#[cfg(feature = "webp")]
pub use dynimage::load_webp;
#[cfg(feature = "tiff")]
pub use dynimage::load_tiff;
#[cfg(feature = "tga")]
pub use dynimage::load_tga;
#[cfg(feature = "bmp")]
pub use dynimage::load_bmp;
#[cfg(feature = "ico")]
pub use dynimage::load_ico;

pub use dynimage::DynamicImage::{
    self,
    ImageRgb8,
    ImageRgba8,
    ImageLuma8,
    ImageLumaA8
};

pub use animation::{
    Frame,
    Frames
};

// Math utils
pub mod math;

// Image processing functions
pub mod imageops;

// Image codecs
#[cfg(feature = "webp")]
pub mod webp;
#[cfg(feature = "ppm")]
pub mod ppm;
#[cfg(feature = "png_codec")]
pub mod png;
#[cfg(feature = "ico")]
pub mod ico;
#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "gif_codec")]
pub mod gif;
#[cfg(feature = "tiff")]
pub mod tiff;
#[cfg(feature = "tga")]
pub mod tga;
#[cfg(feature = "bmp")]
pub mod bmp;

mod image;
mod utils;
mod dynimage;
mod color;
mod buffer;
mod traits;
mod animation;

// Copies data from `src` to `dst`
//
// Panics if the length of `dst` is less than the length of `src`.
// NOTE: this is a copy-paste of the unstable function `std::slice::bytes::copy_memory`.
#[inline]
fn copy_memory(src: &[u8], dst: &mut [u8]) {
    let len_src = src.len();
    assert!(dst.len() >= len_src);
    // `dst` is unaliasable, so we know statically it doesn't overlap
    // with `src`.
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(),
                                      dst.as_mut_ptr(),
                                      len_src);
    }
}
