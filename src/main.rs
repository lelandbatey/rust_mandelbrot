//use std::ops::{Add, Mul, Rem, BitXor, Not};
use std::ops::{Add, Mul};
use std::fmt;

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

fn iterate(start: f64, stop: f64, steps: i64) -> Vec<f64> {
    let len = stop - start;
    let gap = len / (steps - 1) as f64;
    let mut rv: Vec<f64> = Vec::new();
    for step in 0..steps {
        rv.push(start + (gap * step as f64))
    }
    rv
}

fn mandelbrot(z: Complex, c: Complex) -> Complex {
    let mut x = z.clone();
    for _ in 0..100 {
        x = x * x + c;
    }
    x
}


fn main() {
    let nums = vec![cmplx(1.0, 2.0), cmplx(3.0, 4.0), cmplx(5.0, 6.0)];
    for n in nums.clone() {
        for n2 in nums.clone() {
            println!("{} * {} = {}", n, n2, n * n2)
        }
    }
    //for x in iterate(-2.0, 0.52, 80) {
    for y in iterate(-1.0, 1.0, 80) {
        for x in iterate(-2.0, 0.52, 220) {
            let n = mandelbrot(cmplx(x, y), cmplx(x, y));
            //print!("{}", cmplx(x, y));
            if n.real.powi(2) < 4.0 && n.imaginary.powi(2) < 4.0 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
    println!("Hello, world!");
}
