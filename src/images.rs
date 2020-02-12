use std::fmt::{Display, Formatter, Error};
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::iter::Iterator;
use std::io::Read;
use std::io::Write;
use std::env;
use std::fs;
use crate::pixel::Pixel;

pub struct Image {

    image_type: String,
    height: usize,
    width: usize,
    high_pixel: usize,
    vect_pixels: Vec<Pixel>,
}

impl Image {

    #[allow(dead_code)]
    pub fn new_with_file(path: &str){

        let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
        let mut file_lines = file.lines();
        let format = file_lines.next().unwrap();
        let mut height_width = file_lines.next().unwrap();
        let max_pixel = file_lines.next().unwrap();
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
        let mut pixels_vict = Vec::<Pixel>::new();
        let mut tmp = vec![];
        for pixel in split{
            if  i == 4{
                cnt += 1;
                // println!("================ - Created Pixel {:} ================", cnt);
                // println!("{:?}", tmp);
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

        // println!("=>Len of vecteur {}", split.iter().count());
        println!("=>Format {:}", format.unwrap());
        println!("=>HW {:}", height_width.unwrap());
        println!("=>MaxPixel {:}", max_pixel.unwrap());

        // Image{
        //     image_type: format,


        // }
        
    }

    pub fn invert(&mut self){ //inversion de tout les pixels ==> utilisation de la fonction Pixel.invert()
        for pixel in self.vect_pixels.iter_mut(){
            pixel.invert();
        }
    }
}