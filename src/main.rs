use clap::Parser;
use image::{io::Reader as ImageReader, Rgba};
use imageproc::{
    contours::{find_contours_with_threshold, BorderType, Contour},
    drawing::draw_hollow_polygon_mut,
    point::Point,
};
use rayon::prelude::*;

use crate::robust_fit::robust_fit_ellipse;

mod contour;
mod fit_ellipse;
mod robust_fit;

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
    let img_flat = img.to_luma8();

    let contours: Vec<Vec<Point<f64>>> = find_contours_with_threshold(&img_flat, args.threshold)
        .into_iter()
        .filter(|c| c.points.len() >= 50 && c.points.len() <= 2000)
        .filter(|c| c.border_type == BorderType::Hole)
        .map(|c: Contour<i32>| {
            let ps = c
                .points
                .iter()
                .copied()
                .map(|p: Point<_>| Point::new(p.x.into(), p.y.into()))
                .collect::<Vec<_>>();
            ps
        })
        .collect::<Vec<_>>();

    let fit_results = contours[..]
        .par_iter()
        .map(robust_fit_ellipse)
        .collect::<Vec<_>>();

    let mut img_with_fits = img.clone();
    for fit_res in fit_results.iter() {
        for ellipse in fit_res.iter() {
            let res = 40;
            let ellipse_poly = (0..40)
                .into_iter()
                .map(|i| {
                    let angle = (i as f32 / res as f32) * 2.0 * std::f32::consts::PI;
                    let x = ellipse.a as f32 * angle.cos();
                    let y = ellipse.b as f32 * angle.sin();
                    let rotangle = ellipse.theta as f32;
                    let x_rot = x * rotangle.cos() - y * rotangle.sin();
                    let y_rot = x * rotangle.sin() + y * rotangle.cos();
                    Point::new(x_rot + ellipse.x as f32, y_rot + ellipse.y as f32)
                })
                .collect::<Vec<Point<f32>>>();
            draw_hollow_polygon_mut(
                &mut img_with_fits,
                &ellipse_poly[..],
                Rgba([0u8, 0, 255, 255]),
            );
        }
    }
    img_with_fits
        .save("fits.png")
        .expect("Failed to save image");
}
