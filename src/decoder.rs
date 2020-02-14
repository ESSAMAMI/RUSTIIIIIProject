
#[link (name="ppma_io")]

// Appel fonction étrangère
extern{

    fn ppma_write(file_out_name: (&str),xsize: &mut i32, ysize: &mut i32, r: &mut i32, g: &mut i32, b: &mut i32);
    fn ppma_read(file_in_name: (&str), xsize: &mut i32, ysize: &mut i32, rgb_max: &mut i32, r:&mut i32, g:&mut i32, b:&mut i32);
}
fn main() {
    let mut xsize=4;
    let mut ysize=4;
    let mut r=44;
    let mut g=44;
    let mut b=44;
    let mut rgb_max=88;

    let file_in_name = "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3.ppm";
    let file_out_name= "D:/cours/4_IABD/RUST/Projet_Rust/src/images/picture_P3_.ppm";

    let file_read= unsafe{ppma_read(file_in_name, &mut xsize,&mut ysize,&mut rgb_max,&mut r,&mut g,&mut b)};
    let file_write = unsafe{ppma_write(file_out_name,&mut xsize,&mut ysize,&mut r,&mut g,&mut b)};

}
