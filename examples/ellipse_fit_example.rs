use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_hollow_polygon_mut},
    point::Point,
};

use opencv::{
    core::{Point_, RotatedRect, Vector},
    imgproc::fit_ellipse_direct,
    prelude::RotatedRectTraitConst,
};
use particle_detect::{fit_ellipse::fit_ellipse_dls, robust_fit::Ellipse};

fn main() {
    let mut figure = RgbImage::new(256, 256);

    let points: Vector<Point_<i32>> = Vector::from_iter([
        Point_::new(34, 53),
        Point_::new(53, 10),
        Point_::new(100, 180),
        Point_::new(100, 172),
        Point_::new(102, 164),
        Point_::new(105, 187),
        Point_::new(110, 190),
        Point_::new(34, 65),
        Point_::new(39, 85),
    ]);
    let ellipse =
        Ellipse::from_rotated_rect(fit_ellipse_direct(&points).expect("Failed to fit ellipse"));

    figure.pixels_mut().for_each(|p| p.0 = [255, 255, 255]);

    let points_f64 = &points
        .iter()
        .map(|p| Point::new(p.x as f64, p.y as f64))
        .collect::<Vec<_>>()[..];
    let ellipse = fit_ellipse_dls(points_f64).expect("Failed to fit ellipse");
    let res = 50;
    let ellipse_poly = (0..res)
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
    draw_hollow_polygon_mut(&mut figure, &ellipse_poly[..], Rgb([0, 0, 255]));

    points
        .iter()
        .for_each(|p| draw_filled_circle_mut(&mut figure, (p.x, p.y), 2, Rgb([255, 0, 0])));

    figure.save("figure.png").expect("Failed to save image");

    println!("Hello, world!");
}

trait FromRotatedRect {
    fn from_rotated_rect(rect: RotatedRect) -> Self;
}

impl FromRotatedRect for Ellipse {
    fn from_rotated_rect(rect: RotatedRect) -> Self {
        let size = rect.size();
        let center = rect.center();
        let angle = rect.angle();
        Ellipse {
            a: size.width as f64 / 2.0,
            b: size.height as f64 / 2.0,
            x: center.x as f64,
            y: center.y as f64,
            theta: (angle / 180.0 * std::f32::consts::PI) as f64,
        }
    }
}
