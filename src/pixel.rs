#[derive(Clone, Debug, Copy)]
pub struct Pixel{
    red: u8,
    green: u8,
    blue: u8
}

impl Pixel{

    pub fn new (red: u8, green: u8, blue: u8) -> Pixel{
        Pixel{
            red: red,
            green: green,
            blue: blue,
        }
    }

    // Getters and Setters
    #[allow(dead_code)]
    pub fn get_red(&self)-> u8{
        return self.red
    }

    #[allow(dead_code)]
    pub fn get_green(&self)-> u8{
        return self.green
    }

    #[allow(dead_code)]
    pub fn get_blue(&self)-> u8{
        return self.blue
    }

    // ToString :p
    #[allow(dead_code)]
    pub fn display(&self)-> String{

        return format!("Red => {}, Green => {}, Blue => {}", self.red, self.green, self.blue)
    }

    #[allow(dead_code)]
    pub fn invert(&mut self){

        self.red = 255 - self.red;
        self.green = 255 - self.green;
        self.blue = 255 - self.blue;
    }

    //  CIE 709 recommondation 
    #[allow(dead_code)]
    pub fn grayscale(&self)->u8{
        ((self.red as f64 + self.green as f64 + self.blue as f64) / 3.0) as u8
    }

    
}
// Source => https://doc.rust-lang.org/std/cmp/trait.Eq.html
#[allow(dead_code)]
impl PartialEq for Pixel{
    fn eq(&self, other:&Self) ->bool{
        self.get_red() == other.get_red() && self.get_green() == other.get_green() && self.get_blue() == other.get_blue()
    }
}
