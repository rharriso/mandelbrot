extern crate num;
use num::Complex;

use std::str::FromStr;

/// Parse the strings `s` as a coordinate pare, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the charater given by the `separator argument, and <left> and <right> are both strings
/// that can be parsed from `T::from_str`
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse correctly, return `None`
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("20,10", ','), Some((20, 10)));
    assert_eq!(parse_pair::<i32>("20,10xy", ','), None);
    assert_eq!(parse_pair::<i32>("20x10xy", 'x'), None);
    assert_eq!(parse_pair::<i32>("20x10", 'x'), Some((20, 10)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
}

///
/// Give a Complex number for the given point
///
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // why subtract? pixel.1 increases as we go down....
    }
}

#[test]
fn test_pixe_to_point() {
    assert_eq!(
        pixel_to_point((100, 100), (25, 75), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }),
        Complex { re: -0.5, im: -0.5 }
    )
}

fn main() {
    println!("Hello, world!");
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit{
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
    }

    None
}
