extern crate image;
use num::complex::Complex;
use std::env;

fn escape_time(c: Complex<f64>, max_iter: u32, fractal_type: &str) -> Option<u32> {
    let mut z = match fractal_type {
        "mandelbrot" => Complex { re: 0.0, im: 0.0 },
        "julia" => c,
        "burning_ship" => Complex { re: 0.0, im: 0.0 },
        "tricorn" => Complex { re: 0.0, im: 0.0 },
        "dede" => Complex { re: 0.0, im: 0.0 },
        "mandelbrot_4th" => Complex { re: 0.0, im: 0.0 },
        "mandelbrot_5th" => Complex { re: 0.0, im: 0.0 },
        _ => panic!("Unsupported fractal type"),
    };
    let k = Complex::new(-0.4, 0.6); // for Julia set

    for i in 0..max_iter {
        if z.norm() > 2.0 {
            return Some(i);
        }
        z = match fractal_type {
            "mandelbrot" => z * z + c,
            "julia" => z * z + k,
            "burning_ship" => Complex { re: z.re.abs(), im: z.im.abs() } * Complex { re: z.re.abs(), im: z.im.abs() } + c,
            "tricorn" => Complex { re: z.re, im: -z.im } * Complex { re: z.re, im: -z.im } + c,
            "dede" => Complex { re: z.re, im: z.im } * Complex { re: z.re, im: z.im } + c / Complex { re: 2.0, im: 0.0 },
            "mandelbrot_4th" => z * z * z * z + c,
            "mandelbrot_5th" => z * z * z * z * z + c,
            _ => panic!("Unsupported fractal type"),
        };
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Not enough arguments. Usage: <program> <fractal_type> <max_iter>");
    }
    let fractal_type = &args[1];
    let max_iter: u32 = args[2].parse().expect("Max iteration should be a number");

    let width = 800;
    let height = 800;

    let xmin = -2.0;
    let xmax = 2.0;
    let ymin = -2.0;
    let ymax = 2.0;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = xmin + (x as f64 / width as f64) * (xmax - xmin);
        let cy = ymin + (y as f64 / height as f64) * (ymax - ymin);
        let c = Complex::new(cx, cy);

        if let Some(i) = escape_time(c, max_iter, fractal_type) {
            let pixel_value = ((i as f64 / max_iter as f64) * 255.0) as u8;
            *pixel = image::Rgb([pixel_value, pixel_value, pixel_value]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    let filename = format!("images/{}/{}.png", fractal_type, max_iter);

    std::fs::create_dir_all("images").unwrap_or_default();
    std::fs::create_dir_all(format!("images/{}", fractal_type)).unwrap_or_default();

    imgbuf.save(filename).unwrap();
}