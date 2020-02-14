// =========== MOD ============
mod pixel;
pub mod images;

// ========== USE =============
use images::Image;
use std::path::Path;

fn main() {
    // println!("Hello, world!");

    let path = "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm";

    // Test Image
    #[allow(dead_code)]
    
    let mut image:Image = Image::new_with_file(path);
    println!("=>Image Before {:?}", image);

    image.invert();

    println!("=>Image After Invert {:?}", image);

    let save_image = Path::new("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3_inverted.ppm");
    image.save(save_image);
    
}
