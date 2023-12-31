mod fractal;
mod func;

use std::f32::consts::{PI, SQRT_2, TAU};

use image::RgbImage;
use num_complex::Complex32;

use crate::fractal::{calculate_colour, find_root, pixel_to_complex};
use crate::func::ComplexFunction;
// use image::{Rgb, RgbImage};
// use num_complex::Complex32;
// use palette::{FromColor, Hsv, Srgb};
// use rayon::{prelude::*, ThreadPoolBuilder};

// use func::ComplexFunction;

// use gtk::{glib, Application, ApplicationWindow};
// use gtk::{prelude::*, Button};

// const APP_ID: &str = "com.github.mrlegohead0x45.newton_fractal";

// fn main() -> glib::ExitCode {
//     // Create a new application
//     let app = Application::builder().application_id(APP_ID).build();

//     // Connect to "activate" signal of `app`
//     app.connect_activate(build_ui);

//     // Run the application
//     app.run()
// }

// fn build_ui(app: &Application) {
//     let button = Button::builder()
//         .label("Press me!")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     // Connect to "clicked" signal of `button`
//     button.connect_clicked(|button| {
//         // Set the label to "Hello World!" after the button has been clicked on
//         button.set_label("Hello World!");
//     });

//     // Create a window
//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("Newton Fractal Generator")
//         .child(&button)
//         .build();

//     // Present window
//     window.present();
// }

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

    let h = 7000;
    let w = 7000;
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
                                pixel_to_complex(Complex32::new(0.0, 0.0), 1000.0, (w, h), (x, y)),
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
