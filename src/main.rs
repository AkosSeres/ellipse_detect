use clap::Parser;
use image::{io::Reader as ImageReader, GrayImage};

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

    keep_contours(&mut img);

    img.save("out.png").expect("Failed to save image");

    println!("Hello, world!");
}

fn keep_contours(img: &mut GrayImage) {
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
                is_inner = false;
            }
        }
    });

    let mut from_right = img.clone();
    from_right.rows_mut().for_each(|row| {
        let mut is_inner = false;
        for p in row.into_iter().rev() {
            if p.0[0] == 0 {
                if !is_inner {
                    is_inner = true;
                } else {
                    p.0[0] = 255;
                }
            } else {
                is_inner = false;
            }
        }
    });

    let mut from_top = img.clone();
    let w = from_top.width();
    let h = from_top.height();
    for i in 0..w {
        let mut is_inner = false;
        for j in 0..h {
            let mut p = from_top.get_pixel_mut(i, j);
            if p.0[0] == 0 {
                if !is_inner {
                    is_inner = true;
                } else {
                    p.0[0] = 255;
                }
            } else {
                is_inner = false;
            }
        }
    }

    let mut from_bottom = img.clone();
    for i in 0..w {
        let mut is_inner = false;
        for j in (0..h).rev() {
            let mut p = from_bottom.get_pixel_mut(i, j);
            if p.0[0] == 0 {
                if !is_inner {
                    is_inner = true;
                } else {
                    p.0[0] = 255;
                }
            } else {
                is_inner = false;
            }
        }
    }

    img.enumerate_pixels_mut().for_each(|(i, j, p)| {
        if from_left.get_pixel(i, j).0[0] == 0
            || from_right.get_pixel(i, j).0[0] == 0
            || from_top.get_pixel(i, j).0[0] == 0
            || from_bottom.get_pixel(i, j).0[0] == 0
        {
            p.0[0] = 0;
        } else {
            p.0[0] = 255;
        }
    });
}

fn keep_contours_thick(img: &mut GrayImage) {
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

    let mut from_right = img.clone();
    from_right.rows_mut().for_each(|row| {
        let mut is_inner = false;
        for p in row.into_iter().rev() {
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

    let mut from_top = img.clone();
    let w = from_top.width();
    let h = from_top.height();
    for i in 0..w {
        let mut is_inner = false;
        for j in 0..h {
            let mut p = from_top.get_pixel_mut(i, j);
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
    }

    let mut from_bottom = img.clone();
    for i in 0..w {
        let mut is_inner = false;
        for j in (0..h).rev() {
            let mut p = from_bottom.get_pixel_mut(i, j);
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
    }

    img.enumerate_pixels_mut().for_each(|(i, j, p)| {
        if from_left.get_pixel(i, j).0[0] == 0
            || from_right.get_pixel(i, j).0[0] == 0
            || from_top.get_pixel(i, j).0[0] == 0
            || from_bottom.get_pixel(i, j).0[0] == 0
        {
            p.0[0] = 0;
        } else {
            p.0[0] = 255;
        }
    });
}
