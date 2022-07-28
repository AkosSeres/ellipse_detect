use image::{GrayImage, Luma};
use imageproc::{drawing::draw_hollow_polygon_mut, point::Point};

use particle_detect::robust_fit::Ellipse;

fn main() {
    let mut figure = GrayImage::new(512, 512);
    let ellipse = Ellipse::new(256., 256., 30., 70.0, 1.3);

    figure.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let x = x as f64;
        let y = y as f64;
        let d = ellipse.distance_from_perimeter(x, y);
        p.0[0] = (255.0 * ((d / 5.).tanh())) as u8;
    });

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
    draw_hollow_polygon_mut(&mut figure, &ellipse_poly[..], Luma([0u8]));

    figure.save("figure.png").expect("Failed to save image");

    println!("Hello, world!");
}
