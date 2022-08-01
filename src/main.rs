use crate::fit_args::FitArgs;
use clap::Parser;
use image::{io::Reader as ImageReader, Rgba};
use imageproc::{
    contours::{find_contours_with_threshold, Contour},
    drawing::draw_hollow_polygon_mut,
    point::Point,
};
use particle_detect::fit_args::CliArgs;
use rayon::prelude::*;

use crate::robust_fit::robust_fit_ellipse;

mod contour;
mod fit_args;
mod fit_ellipse;
mod robust_fit;

fn main() {
    let cli_args = CliArgs::parse();
    let config_file = std::fs::read_to_string(cli_args.config).unwrap();
    let fit_args = serde_yaml::from_str::<FitArgs>(&config_file).unwrap();

    let img = ImageReader::open(cli_args.file.clone())
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    let img_flat = img.to_luma8();

    let contours: Vec<Vec<Point<f64>>> =
        find_contours_with_threshold(&img_flat, fit_args.threshold)
            .into_iter()
            .filter(|c| {
                c.points.len() >= fit_args.min_contour_points
                    && c.points.len() <= fit_args.max_contour_points
            })
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
        .map(|ps| robust_fit_ellipse(ps, &fit_args))
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
        .save(
            cli_args
                .file
                .to_str()
                .unwrap()
                .to_owned()
                .replace(".bmp", ".jpg"),
        )
        .expect("Failed to save image");
}
