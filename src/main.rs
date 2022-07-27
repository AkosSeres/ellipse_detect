use clap::Parser;
use image::{io::Reader as ImageReader, Rgb, Rgba};
use imageproc::{
    contours::{find_contours, BorderType, Contour},
    drawing::{draw_polygon, draw_polygon_mut},
    point::Point,
};
use opencv::{
    core::{Point_, Vector},
    imgproc::fit_ellipse_direct,
};

use crate::fit_ellipse::fit_ellipse_dls;

mod contour;
mod fit_ellipse;

// Program to detect elongated particles on images
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Pathname of the image to open
    #[clap(short, long, value_parser)]
    file: String,

    /// Threshold for binarization
    #[clap(short, long, value_parser, default_value = "35")]
    threshold: u8,
}

fn main() {
    let args = Args::parse();

    let img = ImageReader::open(args.file)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut img_binarized = img.to_luma8();
    img_binarized
        .pixels_mut()
        .for_each(|p| p.0[0] = if p.0[0] > args.threshold { 255 } else { 0 });
    img_binarized
        .save("binarized.png")
        .expect("Failed to save image");

    let contours: Vec<Vector<Point_<i32>>> = find_contours(&img_binarized)
        .into_iter()
        .filter(|c| c.points.len() >= 50 && c.points.len() <= 2000)
        .filter(|c| c.border_type == BorderType::Hole)
        .map(|c: Contour<i32>| {
            let ps = c
                .points
                .clone()
                .into_iter()
                .map(|p: Point<_>| Point_::new(p.x as i32, p.y as i32))
                .collect();
            ps
        })
        .collect();

    let mut with_contours = img.clone();
    for contour in contours.iter() {
        draw_polygon_mut(
            &mut with_contours,
            &contour
                .into_iter()
                .map(|p| Point::new(p.x, p.y))
                .collect::<Vec<_>>()[..],
            Rgba([255u8, 0, 0, 255]),
        );
    }
    with_contours
        .save("contours.png")
        .expect("Failed to save image");

    let fit_res = fit_ellipse_direct(&contours[0]).expect("Failed to fit ellipse");

    println!("Hello, world!");
}
