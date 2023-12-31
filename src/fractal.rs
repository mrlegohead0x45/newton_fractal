use image::Rgb;
use num_complex::Complex32;
use palette::{FromColor, Hsv, Srgb};

use crate::func::ComplexFunction;

pub(crate) fn find_root(z: Complex32, f: ComplexFunction<'_>) -> Option<(usize, usize)> {
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
}

pub(crate) fn calculate_colour(root_info: Option<(usize, usize)>, no_of_roots: usize) -> Rgb<u8> {
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

pub(crate) fn pixel_to_complex(
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
