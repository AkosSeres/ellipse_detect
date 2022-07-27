use image::GrayImage;

pub fn keep_contours(img: &mut GrayImage) {
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

pub struct ContourFinder {
    img: GrayImage,
    black_finder_iter_num: usize,
}

impl ContourFinder {
    fn new(img: GrayImage) -> ContourFinder {
        ContourFinder {
            img,
            black_finder_iter_num: 0,
        }
    }

    fn find_black_pixel(&mut self) -> Option<(u32, u32)> {
        self.img
            .enumerate_pixels()
            .skip(self.black_finder_iter_num)
            .find(|(_i, _j, p)| {
                self.black_finder_iter_num += 1;
                p.0[0] == 0
            })
            .map(|(i, j, _)| (i, j))
    }

    fn find_black_pixel_next_to(&self, pos: (u32, u32)) -> (u32, u32) {
        let (i, j) = pos;
        let mut next_to = (i, j);
        if self.img.get_pixel(i, j).0[0] == 0 {
            if i < self.img.width() - 1 && self.img.get_pixel(i + 1, j).0[0] == 0 {
                next_to = (i + 1, j);
            } else if i > 0 && self.img.get_pixel(i - 1, j).0[0] == 0 {
                next_to = (i - 1, j);
            } else if j < self.img.height() - 1 && self.img.get_pixel(i, j + 1).0[0] == 0 {
                next_to = (i, j + 1);
            } else if j > 0 && self.img.get_pixel(i, j - 1).0[0] == 0 {
                next_to = (i, j - 1);
            } else if j > 0 && i > 0 && self.img.get_pixel(i - 1, j - 1).0[0] == 0 {
                next_to = (i - 1, j - 1);
            } else if j > 0
                && i < self.img.width() - 1
                && self.img.get_pixel(i + 1, j - 1).0[0] == 0
            {
                next_to = (i + 1, j - 1);
            } else if i < self.img.width() - 1
                && j < self.img.height() - 1
                && self.img.get_pixel(i + 1, j + 1).0[0] == 0
            {
                next_to = (i + 1, j + 1);
            } else if i > 0
                && j < self.img.height() - 1
                && self.img.get_pixel(i - 1, j + 1).0[0] == 0
            {
                next_to = (i - 1, j + 1);
            }
        }
        next_to
    }

    fn trace_contour(&mut self, start: (u32, u32)) -> Vec<(u32, u32)> {
        let mut current = start;
        let mut next;
        let mut contour = Vec::new();
        contour.push(current);
        loop {
            next = self.find_black_pixel_next_to(current);
            self.img.get_pixel_mut(current.0, current.1).0[0] = 255;
            if next == current {
                break;
            }
            contour.push(next);
            current = next;
        }
        contour
    }
}

impl Iterator for ContourFinder {
    type Item = Vec<(u32, u32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.find_black_pixel()?;
        Some(self.trace_contour(start))
    }
}

pub trait ToContourFinder {
    fn to_contour_finder(self) -> ContourFinder;
}

impl ToContourFinder for GrayImage {
    fn to_contour_finder(self) -> ContourFinder {
        ContourFinder::new(self)
    }
}
