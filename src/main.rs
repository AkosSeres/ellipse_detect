use clap::Parser;
use image::{io::Reader as ImageReader, Rgb, Rgba};
use imageproc::{
    contours::{find_contours, Contour},
    drawing::{draw_polygon, draw_polygon_mut},
    point::Point,
};

mod contour;

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

    let contours: Vec<Contour<u32>> = find_contours(&img_binarized)
        .into_iter()
        .filter(|c| c.points.len() >= 50 && c.points.len() <= 2000)
        .collect();
    println!("{} contours found", contours.len());
    let mut with_contours = img.clone();
    for contour in contours {
        draw_polygon_mut(
            &mut with_contours,
            &contour
                .points
                .into_iter()
                .map(|p| Point::new(p.x as i32, p.y as i32))
                .collect::<Vec<Point<i32>>>()[..],
            Rgba([255u8, 0, 0, 255]),
        );
    }
    with_contours
        .save("contours.png")
        .expect("Failed to save image");

    println!("Hello, world!");
}
