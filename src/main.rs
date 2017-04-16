
extern crate argparse;

use argparse::{ArgumentParser, Store};

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::ops::{Add, Mul};
use std::thread;
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
    minimum: i64,
    pixels: Vec<i64>,
}

impl Img {
    pub fn new(h: i64, w: i64) -> Img {
        Img {
            height: h,
            width: w,
            maximum: 1,
            minimum: 1000,
            pixels: vec![0; (h*w) as usize],
        }
    }
    pub fn set_px(&mut self, x: i64, y: i64, val: i64) {
        if x < self.width && x >= 0 {
            if y < self.height && y >= 0 {
                if val > self.maximum {
                    self.maximum = val
                }
                if val < self.minimum {
                    self.minimum = val
                }
                self.pixels[((self.height * y) + x) as usize] = val
            }
        }
    }
}

// Print the contents of our "Img" struct according to the "Plane PGM" variant of the PGM standard:
// http://netpbm.sourceforge.net/doc/pgm.html#plainpgm
impl fmt::Display for Img {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "P2\n# Created by leland batey RustPGM\n{} {}\n{}\n",
               self.height,
               self.width,
               self.maximum - self.minimum)
            .unwrap();
        let pixvals: Vec<String> = self.pixels
            .iter()
            .map(|pix| if pix >= &self.minimum {
                format!("{}", pix - self.minimum)
            } else {
                format!("{}", pix)
            })
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

// Returns a Vector of "steps" length with values starting at "start" and ending at "stop",
// inclusively. The first value of the returned vector (rv[0]) will always be equal to "start", and
// the last value of the returned vector (rv[-1]) will always be equal to "stop".
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
        if x.real.abs() > 2.0 && x.imaginary.abs() > 4.0 {
            return (x, itr);
        }
    }
    (x, max_iters)
}

#[derive(Copy, Clone)]
struct Pixel {
    // x and y represent the coordinate of this pixel on an actual grid of pixels, with (0, 0) as
    // the upper left and (height, width) as the lower right pixel.
    x: i64,
    y: i64,
    // Val is any positive integer and is the brightness, with 0 being black, and the highest pixel
    // value of all being pure white.
    val: i64,
    // rx and ry represent the coordinate of the pixel on the imaginary plane.
    rx: f64,
    ry: f64,
}


fn main() {
    let mut thread_count = 4;
    let mut resolution = 2048;
    let mut max_iters = 120;

    // We render square images centered on (-0.74, 0.0), that are by default 2.52 in real-number
    // space on either side.
    // This means we sample from (-2.0, -1.26) in the top left to (0.52, 1.26) in the bottom right.
    let (mut centerx, mut centery) = (-0.74, 0.0);
    let startzoom = 1.26;

    let mut zoomlevel = 1.0;

    //centerx = -0.7010733;
    //centery = 0.352329458;

    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Render a mandelbrot set as PGM");
        argparse.refer(&mut thread_count)
            .add_option(&["-t", "--threads"],
                        Store,
                        "Number of threads to use (default 4)");
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

    let (startx, stopx) = (centerx - (startzoom / (2.0 as f64).powf(zoomlevel)),
                           centerx + (startzoom / (2.0 as f64).powf(zoomlevel)));
    let (starty, stopy) = (centery - (startzoom / (2.0 as f64).powf(zoomlevel)),
                           centery + (startzoom / (2.0 as f64).powf(zoomlevel)));

    let mut img = Img::new(resolution, resolution);
    // Here we create a vector of Pixels for each thread, and populate each of those "pools of
    // work" with ~equal numbers of pixels to be calculated.
    let mut thread_work: Vec<Vec<Pixel>> = vec![Vec::new(); thread_count as usize];
    {
        for y in iterate(starty, stopy, img.height) {
            for x in iterate(startx, stopx, img.width) {
                // Calculate the coordinates in the image where this pixel will be located, based
                // on it's x and y coordinates in the imaginary plane.
                let ix = map_real_img(startx, stopx, x, img.width);
                let iy = map_real_img(starty, stopy, y, img.height);
                let ref mut work = thread_work[(ix % thread_count) as usize];
                work.push(Pixel {
                    x: ix,
                    y: iy,
                    val: 0,
                    rx: x,
                    ry: y,
                });
            }
        }
    }
    // Use Arc to allow us to have multiple references to our vector of vectors of Pixels, as none
    // of those vectors need to be modified, and each thread will be iterating only over separate
    // vectors at one time.
    let tw = Arc::new(thread_work);
    let (tx, rx) = channel();
    let mut children = vec![];
    for idx in 0..thread_count {
        let child_tw = tw.clone();
        let child_tx = tx.clone();
        // Spawn one thread for each thread in `thread_count`, with that thread in charge of only a
        // single division of work. All Pixels are copied and sent back to the parent thread via a
        // channel.
        let child = thread::spawn(move || {
            let ref work = child_tw[idx as usize];
            for pix in work {
                let (_, itrs) = mandelbrot(cmplx(pix.rx, pix.ry), cmplx(pix.rx, pix.ry), max_iters);
                child_tx.send(Pixel {
                        x: pix.x,
                        y: pix.y,
                        val: itrs,
                        rx: pix.rx,
                        ry: pix.ry,
                    })
                    .unwrap();
            }
        });
        children.push(child);
    }

    // Collect each Pixel and set it's corresponding location to the value of that Pixel in the
    // Image.
    for _ in 0..img.pixels.len() {
        match rx.recv() {
            Ok(pix) => {
                img.set_px(pix.x, pix.y, pix.val);
            }
            Err(_) => break,
        }
    }
    for child in children {
        child.join().unwrap();
    }

    print!("{}", img)
}
