#![feature(test)]
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::iter::Iterator;
use std::io::prelude::*;
use crate::pixel::Pixel;

#[repr(C, packed)]
#[derive(Debug, Clone)]

// Struct Image [image_type, height, width, high_pixel, vect_pixel]
pub struct Image {
    image_type: String,
    height: usize,
    width: usize,
    high_pixel: usize,
    vect_pixels: Vec<Pixel>,
}
// Implement Image:: new_with_file(String), Image::invert(), Image::grayScale(), Image::save(String)
impl Image {

    #[allow(dead_code)]

    pub fn new_with_file(path: &str)-> Image{

        // Variabels nedded
        // Read file
        let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
        // Fetch all lines
        let mut file_lines = file.lines();
        // Get Format from file
        let format = file_lines.next().unwrap().unwrap().to_string();
        // Create a Vic  of String with width and height
        let width_height: Vec<String> = file_lines.next().unwrap().unwrap().split(' ').map(|s| s.to_string()).collect();
        // Get element at position 0 => width (parsing to requsted type usize)
        let width = width_height[0].parse::<usize>().unwrap();
        // Get element at position 0 => height (parsing to requsted type usize)
        let height = width_height[1].parse::<usize>().unwrap();
        // Get max Pixel
        let max_pixel = file_lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        // Split and Parse color to u8 (PS it will be created as a Flatten image one dim)
        let mut split: Vec<u8> = Vec::new();
        for line in file_lines{
            
            split.extend(line.unwrap()
                .split_whitespace()
                .map(|i| i.parse::<u8>().unwrap()));
                
        }   
        
        println!("=>Len of vect {}", split.iter().count());
        let mut i:u8 = 1;
        let mut cnt:u8 = 0;
        // ===================== DECLA ===================== \\

        // Create a Pixel from old vect named 'split' [r, g, b]
        let mut pixels_vict = Vec::<Pixel>::new();
        let mut tmp = vec![];
        for pixel in split{
            if  i == 4{
                cnt += 1;
                println!("================ - Created Pixel {:} ================", cnt);
                println!("{:?}", tmp);
                pixels_vict.push(Pixel::new(tmp[0], tmp[1], tmp[2]));
                tmp.remove(2);
                tmp.remove(1);
                tmp.remove(0);
                i = 0;

            }else{
                tmp.push(pixel);
            }
            i+= 1;
        }
        // Here we goooooooooooooooooooo Image will be enable to create !!
        Image{
            image_type: format,
            height: height,
            width: width,
            high_pixel: max_pixel,
            vect_pixels: pixels_vict,
        }
        
    }

    #[allow(dead_code)]
    pub fn invert(&mut self){ // invert all of pixel using function decalred in Pixel struct
        for pixel in self.vect_pixels.iter_mut(){
            pixel.invert();
        }
    }

    #[allow(dead_code)]
    pub fn grayscale(&mut self) {
        for pixel in unsafe{self.vect_pixels.iter_mut()} {
            pixel.grayscale();
        }
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: &Path){

        if(self.image_type == "P3"){
            let header = unsafe{format!("{}\n{} {}\n{}\n", self.image_type, self.width, self.height, self.high_pixel)};
            
            let mut new_file = match File::create(filename) {
                Err(why) => panic!("Unable to write file..."),
                Ok(new_file) => new_file,
            };

            new_file.write_all(header.as_bytes()).expect("Unable to write header");
            let mut line = 1;
            for pixel in &self.vect_pixels{        
                if line % 5 == 0{
                    line = 1;
                    new_file.write_all("\n".as_bytes());
                }
                line += 1;
                new_file.write_all(pixel.to_string().as_bytes()).expect("Unable to write line...");
                new_file.write_all(" ".as_bytes())
                .expect("Unable to write pixels...");
            }
        }else{

            panic!("Unable to invert file Sorry try with a P3 format :*")
        }

    
    }

}

#[cfg(test)]

mod tests {
    use super::*;
    use test::Bencher;


    #[test]
    fn image_header() {
        let image = Image::new_with_file("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm");
        assert_eq!(image.image_type, "P3")

    }

    fn image_width() {
        let image = Image::new_with_file("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm");
        assert_eq!(image.width, 34)
    }
    
    fn image_height() {
        let image = Image::new_with_file("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm");
        assert_eq!(image.height, 7)

    }

    #[test]
    fn max_pixel() {
        let image = Image::new_with_file("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm");
        assert_eq!(image.high_pixel, 255)

    }

    #[bench]
    fn bench_new_with_file(b: &mut Bencher) {
        b.iter(|| Image::new_with_file("D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm"))
    }

}