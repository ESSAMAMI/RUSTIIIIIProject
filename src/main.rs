#![feature(test)]

extern crate test;

// =========== MOD ============
mod pixel;
pub mod images;

// ========== USE =============
use images::Image;
use std::path::Path;

fn main() {
    
    let path = "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm";

    // Test Image
    #[allow(dead_code)]
    
    let mut image:Image = Image::new_with_file(path);
    println!("=>Image Before {:?}", image);

    image.invert();
    println!("=>Image After Invert {:?}", image);

    let save_image_invert = Path::new("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3_inverted.ppm");
    image.save(save_image_invert);

    image.grayscale();
    println!("=>Image After GrayScale {:?}", image);
    let save_image_grayscale = Path::new("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3_gray_scale.ppm");
    image.save(save_image_grayscale);

}
