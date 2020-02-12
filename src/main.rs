mod pixel;
mod images;

fn main() {
    // println!("Hello, world!");

    let path = "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm";

    images::Image::new_with_file(path)
    
}
