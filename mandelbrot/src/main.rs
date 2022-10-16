
extern crate num;
extern crate clap;
extern crate image;
extern crate crossbeam;

use std::str::FromStr;
use std::vec;
use clap::Parser;

use num::Complex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    frame: String,
    #[arg(short, long)]
    upper_left: String,
    #[arg(short, long)]
    lower_right: String,
    #[arg(short, long)]
    n_worker: usize
}

fn escape_time(c: Complex<f64>, hard_limit: u32, crit_r: f64) -> Option<u32> {
    let mut z = Complex { re: 0., im: 0. };
    for i in 0..hard_limit {
        z = z * z + c;
        if z.norm_sqr() > crit_r {
            return Some(i);
        }
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator:char) -> Option<(T,T)> {
    match s.find(separator) {
        Some(i) => {
            match (T::from_str(&s[..i]),T::from_str(&s[i+1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
        _ => None
    }
}

fn parse_complex(s:&str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re,im)) => Some(Complex { re, im }),
        _ => None
    }
}


fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>, 
                  lower_right: Complex<f64>) -> Complex<f64> {

                    let (w, h) = bounds;
                    let (x, y) = pixel;
                    let (cw, ch) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
                    Complex { 
                        re: upper_left.re + x as f64 / w as f64 * cw, 
                        im: upper_left.im - y as f64 / h as f64 * ch 
                    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let mut imgbuf = image::GrayImage::new(bounds.0 as u32, bounds.1 as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([pixels[y as usize * bounds.0 + x as usize]])
    }
    imgbuf.save(filename).unwrap();
    Ok(())
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>, 
          lower_right: Complex<f64>) {
            println!("render {:?}", bounds);
            assert_eq!(pixels.len(), bounds.0 * bounds.1);
            let (w, h) = bounds;
            for y in 0..h {
                for x in 0..w {
                    let c = pixel_to_point(bounds, (x, y), upper_left, lower_right);
                    pixels[y * w + x] = match escape_time(c, 255, 8.0) {
                        Some(count) => 255 - count as u8,
                        _ => 0
                    };
                }
            }
}

fn render_parallel(pixels: &mut [u8],
                   bounds: (usize, usize),
                   upper_left: Complex<f64>,
                   lower_right: Complex<f64>,
                   nworkers: usize) {

                    let (width, _) = bounds;
                    crossbeam::scope(|spawner| {
                        let n_height = bounds.1 / nworkers;

                        for (idx, band) in pixels.chunks_mut(n_height * width).enumerate() {
                            let band_top = n_height * idx;
                            let band_bottom = band_top + band.len() / width;
                            let band_upper_left = pixel_to_point(bounds, (0, band_top), upper_left, lower_right);
                            let band_lower_right = pixel_to_point(bounds, (width, band_bottom), upper_left, lower_right);
                            spawner.spawn(move |_| {
                                render(band, (width, band.len() / width), band_upper_left, band_lower_right)
                            });
                        }
                    }).expect("fail to create scope");
}



fn main() {
    let args = Args::parse();
    let bounds = parse_pair::<usize>(&args.frame.to_lowercase(), 'x').expect("error parsing frame size");
    let upper_left = parse_complex(&args.upper_left).expect("error parsing upper_left point");
    let lower_right = parse_complex(&args.lower_right).expect("error parsing lower right point");
    let mut pixels = vec![0u8; bounds.0 * bounds.1];
    render_parallel(&mut pixels, bounds, upper_left, lower_right, args.n_worker);

    write_image(&args.output, &pixels, bounds).expect("error writing image");
}


#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75), 
                              Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex { re: -0.5, im: -0.5});
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<u32>("100,", ','), None);
    assert_eq!(parse_pair::<u32>("100,200", ','), Some((100,200)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("20,20"), Some(Complex { re: 20.0, im: 20.0 }));
    assert_eq!(parse_complex("20,"), None);
}