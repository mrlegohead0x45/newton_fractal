use std::{
    f32::consts::{PI, TAU},
    sync::{Arc, Mutex},
};

use image::{Rgb, RgbImage};
use num_complex::Complex32;
use palette::{FromColor, Hsv, Srgb};
use rayon::{prelude::*, ThreadPoolBuilder};

#[derive(Debug, Clone, Copy)]
struct ComplexFunction<'a> {
    roots: &'a [Complex32],
}

impl<'a> ComplexFunction<'a> {
    pub(crate) fn new(roots: &'a [Complex32]) -> Self {
        Self { roots }
    }

    pub(crate) fn eval(&self, z: Complex32) -> Complex32 {
        self.roots.iter().map(|r| z - r).product()
    }

    pub(crate) fn derivative(&self, z: Complex32) -> Complex32 {
        // generalised form of product rule
        // sum of (d(this one) * prod(all the others))

        (0..self.degree())
            .map(|i| {
                self.roots
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, r)| z - r)
                    .product::<Complex32>()
            })
            .sum::<Complex32>()
    }

    pub(crate) fn is_root(&self, z: Complex32, epsilon: f32) -> bool {
        // self.roots.iter().any()
        self.eval(z).norm() < epsilon
    }

    pub(crate) fn degree(&self) -> usize {
        self.roots.len()
    }

    pub(crate) fn identify_root(&self, z: Complex32, epsilon: f32) -> Option<usize> {
        // for i in 0..self.degree() {
        //     if (z - self.roots[i]).norm() < epsilon {
        //         return Some(i);
        //     }
        // }
        // None
        (0..self.degree()).find(|&i| (z - self.roots[i]).norm() < epsilon)
    }
}

fn find_root(z: Complex32, f: ComplexFunction<'_>) -> Option<usize> {
    let epsilon = 1e-5;
    let mut zn = z;

    let max_iterations = 200;
    let mut iterations = 0;

    while iterations < max_iterations {
        zn -= f.eval(zn) / f.derivative(zn);
        if let Some(i) = f.identify_root(zn, epsilon) {
            return Some(i);
        }
        iterations += 1;
    }
    None
    // }
}

fn pixel_to_complex(
    x: u32,
    y: u32,
    x_offset: f32,
    y_offset: f32,
    width: f32,
    height: f32,
    x_factor: f32,
    y_factor: f32,
) -> Complex32 {
    // let w = 600.0;
    // let h = 600.0;
    Complex32::new(
        1.0 / x_factor * (x as f32 - width / 2.0 - x_offset),
        1.0 / -y_factor * (y as f32 - height / 2.0 - y_offset),
    )
}

fn calculate_colour(root_no: Option<usize>, no_of_roots: usize) -> Rgb<u8> {
    match root_no {
        None => Rgb([0, 0, 0]),
        Some(i) => {
            let palette_hsv = Hsv::new((i as f32 / no_of_roots as f32) * 360.0, 1.0, 1.0);
            let palette_rgb = Srgb::from_color(palette_hsv).into_format();
            let raw: [u8; 3] = palette_rgb.into();
            Rgb(raw)
        }
    }
}

fn main() {
    let roots = &[
        Complex32::cis(TAU / 5.0),
        Complex32::cis(2.0 * TAU / 5.0),
        Complex32::cis(3.0 * TAU / 5.0),
        Complex32::cis(4.0 * TAU / 5.0),
        Complex32::cis(5.0 * TAU / 5.0),
    ];
    let f = ComplexFunction::new(roots);

    let h = 2000;
    let w = 2000;
    let mut img = Arc::new(Mutex::new(RgbImage::new(w, h)));
    // RgbImage::new(w, h);
    // img.par_iter_mut().ma
    // println!("{}", pixel_to_complex(300, 300));
    println!("{} {}", f.eval(roots[0]), f.is_root(roots[0], 1e-4));

    // let pool = ThreadPoolBuilder::new().build().unwrap();

    // for x in 0..w {
    // for y in 0..h {
    (0..w).into_par_iter().for_each(|x| {
        (0..h).for_each(|y| {
            // let _ = ((0..w), (0..h))
            // .into_par_iter()
            // .map(|(x, y)| {
            // println!("({w}, {h})");
            // pool.spawn(move || {
            img.lock().unwrap().put_pixel(
                x,
                y,
                calculate_colour(
                    find_root(
                        pixel_to_complex(x, y, 0.0, 0.0, w as f32, h as f32, 100.0, 100.0),
                        f,
                    ),
                    f.degree(),
                ),
            )
        })
        // x + y
    });
    // .sum::<u32>();
    // }
    // }
    // })
    // });
    // img.enumerate_pixels_mut()
    // img.put_pixel(300, 300, )
    img.lock().unwrap().save("out.png").unwrap();

    println!("Hello, world!");
}
