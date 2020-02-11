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

    //  CIE 709 recommondation 
    pub fn convertPixelToGray(&self)->u8{
        ((self.red as f64 + self.green as f64 + self.blue as f64) / 3.0) as u8
    }



    // Getters and Setters
    #[allow(dead_code)]
    pub fn getRed(&self)-> u8{
        return self.red
    }

    #[allow(dead_code)]
    pub fn getGreen(&self)-> u8{
        return self.green
    }

    #[allow(dead_code)]
    pub fn getBlue(&self)-> u8{
        return self.blue
    }

    // ToString :p
    #[allow(dead_code)]
    pub fn display(&self)-> String{

        return format!("Red => {}, Green => {}, Blue => {}", self.red, self.green, self.blue)
    }

}

