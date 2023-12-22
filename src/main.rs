use std::f32::consts::{PI, SQRT_2, TAU};

use image::{Rgb, RgbImage};
use num_complex::Complex32;
use palette::{FromColor, Hsv, Srgb};
// use rayon::{prelude::*, ThreadPoolBuilder};

mod func;
use func::ComplexFunction;

fn find_root(z: Complex32, f: ComplexFunction<'_>) -> Option<(usize, usize)> {
    let epsilon = 1e-5;
    let mut zn = z;

    let max_iterations = 70;
    let mut iterations = 0;

    while iterations < max_iterations {
        zn -= f.eval(zn) / f.derivative(zn);

        if let Some(i) = f.identify_root(zn, epsilon) {
            // println!("{iterations}");
            return Some((i, iterations));
        }

        iterations += 1;
    }
    None
    // }
}

fn calculate_colour(root_info: Option<(usize, usize)>, no_of_roots: usize) -> Rgb<u8> {
    match root_info {
        None => Rgb([0, 0, 0]),
        Some((i, no_of_iterations)) => {
            let palette_hsl = Hsv::new(
                (i as f32 / no_of_roots as f32) * 360.0,
                1.0,
                (-(no_of_iterations as f32) / 20.0).exp(), // 1.0,
            );
            let palette_rgb = Srgb::from_color(palette_hsl).into_format();
            let raw: [u8; 3] = palette_rgb.into();
            Rgb(raw)
        }
    }
}

fn main() {
    let roots = &[
        Complex32::new(1.0, 0.0),
        Complex32::new(-1.0, 0.0),
        Complex32::new(0.0, 1.0),
        Complex32::new(0.0, -1.0),
        Complex32::new(SQRT_2, SQRT_2),
        Complex32::new(-SQRT_2, SQRT_2),
        Complex32::new(SQRT_2, -SQRT_2),
        Complex32::new(-SQRT_2, -SQRT_2),
    ];
    let f = ComplexFunction::new(roots);

    let h = 1200;
    let w = 1920;
    let mut img = //Arc::new(Mutex::new(RgbImage::new(w, h)));
    RgbImage::new(w, h);

    // for x in 0..w {
    // for y in 0..h {
    (0..w).for_each(|x| {
        (0..h)
            // .into_par_iter()
            .for_each(|y| {
                img
                    // .lock().unwrap()
                    .put_pixel(
                        x,
                        y,
                        calculate_colour(
                            find_root(
                                pixel_to_complex(Complex32::new(0.0, 0.0), 200.0, (w, h), (x, y)),
                                f,
                            ),
                            f.degree(),
                        ),
                    )
            })
    });

    img
        // .lock().unwrap()
        .save("out3.png")
        .unwrap();
}

// given the centre of the image as a complex number, a scale of pixels/unit and the dimensions of the image
// pixels are 0, 0 at the top left
fn pixel_to_complex(
    centre: Complex32,
    scale: f32,
    dims: (u32, u32),
    pixel: (u32, u32),
) -> Complex32 {
    Complex32::new(
        (pixel.0 as f32 - dims.0 as f32 / 2.0) / scale + centre.re,
        (pixel.1 as f32 - dims.1 as f32 / 2.0) / scale + centre.im,
    )
}
