// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
// mod images;
// mod pixel;
// use images::Image;
// use std::path::PathBuf;

// #[pyfunction]

// pub fn use_image(path:String, outuput: String, function_name:String){

//     path = Path::new(path);

//     if function_name == "invert"{
//         image.invert();
//     }
//     if function_name == "gray_scale" {
//         image.grayscale();
//     }

//     let mut path_out = PathBuf::new(outuput);
//     image.save(path_out);

// }

// #[pymodule]
// fn bigpepereader(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(convert_image))?;
//     Ok(())
// }
