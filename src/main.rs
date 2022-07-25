use clap::Parser;
use image::io::Reader as ImageReader;

use crate::contour::ToContourFinder;

mod contour;

// Program to detect elongated particles on images
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Pathname of the image to open
    #[clap(short, long, value_parser)]
    file: String,

    /// Threshold for binarization
    #[clap(short, long, value_parser, default_value = "55")]
    threshold: u8,
}

fn main() {
    let args = Args::parse();

    let img = ImageReader::open(args.file)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut img = img.to_luma8();
    img.pixels_mut()
        .for_each(|p| p.0[0] = if p.0[0] > args.threshold { 255 } else { 0 });

    let mut from_left = img.clone();
    from_left.rows_mut().for_each(|row| {
        let mut is_inner = false;
        for p in row.into_iter() {
            if p.0[0] == 0 {
                if !is_inner {
                    is_inner = true;
                } else {
                    p.0[0] = 255;
                }
            } else {
                if is_inner {
                    p.0[0] = 0;
                }
                is_inner = false;
            }
        }
    });

    contour::keep_contours(&mut img);

    let contours: Vec<_> = img
        .clone()
        .to_contour_finder()
        .filter(|contour| contour.len() >= 5)
        .collect();

    println!("{:?}", contours);

    img.save("out.png").expect("Failed to save image");

    println!("Hello, world!");
}
