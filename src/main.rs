mod pixel;
mod images;

fn main() {
    // println!("Hello, world!");

    let path = "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm";

    // Test Image
    #[allow(dead_code)]
    let mut get_image = images::Image::new_with_file(path);
    // println!("{:}", get_image.);    
}
