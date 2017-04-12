//use std::ops::{Add, Mul, Rem, BitXor, Not};

extern crate argparse;

use argparse::{ArgumentParser, Store};

use std::ops::{Add, Mul};
use std::fmt;
use std::cmp;

#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, _rhs: Complex) -> Complex {
        Complex {
            real: (self.real * _rhs.real) + (-1.0 * self.imaginary * _rhs.imaginary),
            imaginary: (self.real * _rhs.imaginary) + (self.imaginary * _rhs.real),
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, _rhs: Complex) -> Complex {
        Complex {
            real: self.real + _rhs.real,
            imaginary: self.imaginary + _rhs.imaginary,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:5.1}+{:5.1}j)", self.real, self.imaginary)
    }
}

fn cmplx(x: f64, y: f64) -> Complex {
    Complex {
        real: x,
        imaginary: y,
    }
}

struct Img {
    height: i64,
    width: i64,
    maximum: i64,
    pixels: Vec<i64>,
}

impl Img {
    pub fn new(h: i64, w: i64) -> Img {
        Img {
            height: h,
            width: w,
            maximum: 1,
            pixels: vec![0; (h*w) as usize],
        }
    }
    pub fn set_px(&mut self, x: i64, y: i64, val: i64) {
        if x < self.width && x >= 0 {
            if y < self.height && y >= 0 {
                if val > self.maximum {
                    self.maximum = val
                }
                self.pixels[((self.height * y) + x) as usize] = val
            }
        }
    }
}
impl fmt::Display for Img {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "P2\n# Created by leland batey RustPGM\n{} {}\n{}\n",
               self.height,
               self.width,
               self.maximum)
            .unwrap();
        let pixvals: Vec<String> = self.pixels
            .iter()
            .map(|pix| format!("{}", pix))
            .collect();
        write!(f, "{}\n", pixvals.join("\n"))
    }
}

// Maps a coordinate on a number line from x to y onto a different extent from 0 to z (intbound)
fn map_real_img(start: f64, end: f64, point: f64, intbound: i64) -> i64 {
    let extent = end - start;
    let scaled = (point - start) / extent;
    ((intbound as f64) * scaled) as i64
}

fn iterate(start: f64, stop: f64, steps: i64) -> Vec<f64> {
    let len = stop - start;
    let gap = len / (steps - 1) as f64;
    let mut rv: Vec<f64> = Vec::new();
    for step in 0..steps {
        rv.push(start + (gap * step as f64))
    }
    rv
}

fn mandelbrot(z: Complex, c: Complex, max_iters: i64) -> (Complex, i64) {
    let mut x = z.clone();
    for itr in 0..max_iters {
        x = x * x + c;
        if x.real.abs() > 4.0 && x.imaginary.abs() > 4.0 {
            return (x, itr);
        }
    }
    (x, max_iters)
}


fn main() {
    let mut resolution = 2048;
    let mut max_iters = 120;
    // x width = 2.52
    // y height = 2.4

    let (mut centerx, mut centery) = (-0.74, 0.0);
    let startzoom = 1.26;

    let mut zoomlevel = 1.0;

    centerx = -0.7010733;
    centery = 0.352329458;

    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Render a mandelbrot set as PGM");
        argparse.refer(&mut resolution)
            .add_option(&["-r", "--resolution"],
                        Store,
                        "Horizontal and vertical resolution.");
        argparse.refer(&mut max_iters)
            .add_option(&["--max_iters"],
                        Store,
                        "Maximum number of allowed iterations.");
        argparse.refer(&mut centerx)
            .add_option(&["-x"], Store, "The center X coordinate");
        argparse.refer(&mut centery)
            .add_option(&["-y"], Store, "The center Y coordinate");
        argparse.refer(&mut zoomlevel)
            .add_option(&["-z", "--zoom"], Store, "Amount of zoom in render");
        argparse.parse_args_or_exit();
    }
    //centerx = -0.694871946;
    //centery = 0.356592915764728;
    //zoomlevel = 16.0;

    let (startx, stopx) = (centerx - (startzoom / (2.0 as f64).powf(zoomlevel)),
                           centerx + (startzoom / (2.0 as f64).powf(zoomlevel)));
    let (starty, stopy) = (centery - (startzoom / (2.0 as f64).powf(zoomlevel)),
                           centery + (startzoom / (2.0 as f64).powf(zoomlevel)));
    //stopy = 0.0;
    //stopx = startx + (stopx - startx) / 2.0;
    //starty += 0.6;
    //stopy += 0.6;
    //startx += 0.1;
    //stopx += 0.1;

    let mut img = Img::new(resolution, resolution);
    for y in iterate(starty, stopy, img.height) {
        for x in iterate(startx, stopx, img.width) {
            let (n, itrs) = mandelbrot(cmplx(x, y), cmplx(x, y), max_iters);
            let ix = map_real_img(startx, stopx, x, img.width);
            let iy = map_real_img(starty, stopy, y, img.height);
            img.set_px(ix, iy, cmp::min(itrs, 55));
            //if n.real.powi(2) < 4.0 && n.imaginary.powi(2) < 4.0 {}
        }
    }
    print!("{}", img)
}
