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
    typeImage: String,
    hgt: usize,
    wdt: usize,
    highPixel: usize,
    vectPixels: Vec<Pixel>,
}

impl Image {

    #[allow(dead_code)]
    pub fn readImage(path: &str){
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        for line in contents.split("\n"){
            
        }
        println!("With text:\n{}", contents);
    }
}