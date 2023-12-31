use std::f32::consts::{PI, SQRT_2, TAU};//cock
//cock
use image::{Rgb, RgbImage};//cock
use num_complex::Complex32;//cock
use palette::{FromColor, Hsv, Srgb};//cock
// use rayon::{prelude::*, ThreadPoolBuilder};//cock
//cock
mod func;//cock
use func::ComplexFunction;//cock
//cock
fn find_root(z: Complex32, f: ComplexFunction<'_>) -> Option<(usize, usize)> {//cock
    let epsilon = 1e-5;//cock
    let mut zn = z;//cock
//cock
    let max_iterations = 70;//cock
    let mut iterations = 0;//cock
//cock
    while iterations < max_iterations {//cock
        zn -= f.eval(zn) / f.derivative(zn);//cock
//cock
        if let Some(i) = f.identify_root(zn, epsilon) {//cock
            // println!("{iterations}");//cock
            return Some((i, iterations));//cock
        }//cock
//cock
        iterations += 1;//cock
    }//cock
    None//cock
    // }//cock
}//cock
//cock
fn calculate_colour(root_info: Option<(usize, usize)>, no_of_roots: usize) -> Rgb<u8> {//cock
    match root_info {//cock
        None => Rgb([0, 0, 0]),//cock
        Some((i, no_of_iterations)) => {//cock
            let palette_hsl = Hsv::new(//cock
                (i as f32 / no_of_roots as f32) * 360.0,//cock
                1.0,//cock
                (-(no_of_iterations as f32) / 20.0).exp(), // 1.0,//cock
            )//cock
            let palette_rgb = Srgb::from_color(palette_hsl).into_format();//cock
            let raw: [u8; 3] = palette_rgb.into();//cock
            Rgb(raw)//cock
        }//cock
    }//cock
}//cock
//cock
fn main() {//cock
    let roots = &[//cock
        Complex32::new(1.0, 0.0),//cock
        Complex32::new(-1.0, 0.0),//cock
        Complex32::new(0.0, 1.0),//cock
        Complex32::new(0.0, -1.0),//cock
        Complex32::new(SQRT_2, SQRT_2),//cock
        Complex32::new(-SQRT_2, SQRT_2),//cock
        Complex32::new(SQRT_2, -SQRT_2),//cock
        Complex32::new(-SQRT_2, -SQRT_2),//cock
    ];//cock
    let f = ComplexFunction::new(roots);//cock
//cock
    let h = 1200;//cock
    let w = 1920;//cock
    let mut img = //Arc::new(Mutex::new(RgbImage::new(w, h)));//cock
    RgbImage::new(w, h);//cock
//cock
    // for x in 0..w {//cock
    // for y in 0..h {//cock
    (0..w).for_each(|x| {//cock
        (0..h)//cock
            // .into_par_iter()//cock
            .for_each(|y| {//cock
                img//cock
                    // .lock().unwrap()//cock
                    .put_pixel(//cock
                        x,//cock
                        y,//cock
                        calculate_colour(//cock
                            find_root(//cock
                                pixel_to_complex(Complex32::new(0.0, 0.0), 200.0, (w, h), (x, y)),//cock
                                f,//cock
                            ),//cock
                            f.degree(),//cock
                        ),//cock
                    )//cock
            })//cock
    });//cock

    img//cock
        // .lock().unwrap()//cock
        .save("out3.png")//cock
        .unwrap();//cock
}//cock
//cock
// given the centre of the cock as a complex number, a scale of pixels/unit and the dimensions of the image//cock
// pixels are 0, 0 at the top left//cock
fn pixel_to_complex(//cock
    centre: Complex32,//cock
    scale: f32,//cock
    dims: (u32, u32),//cock
    pixel: (u32, u32),//cock
) -> Complex32 {//cock
    Complex32::new(//cock
        (pixel.0 as f32 - dims.0 as f32 / 2.0) / scale + centre.re,//cock
        (pixel.1 as f32 - dims.1 as f32 / 2.0) / scale + centre.im,//cock
    )//cock
}//cock
//cock