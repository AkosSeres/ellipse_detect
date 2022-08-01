use imageproc::point::Point;
use nalgebra::{Complex, ComplexField};

use crate::{fit_args::FitArgs, fit_ellipse::fit_ellipse_dls};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub a: f64,
    pub b: f64,
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    axis_a: Point<f64>,
    axis_b: Point<f64>,
}

impl Ellipse {
    pub fn new(x: f64, y: f64, a: f64, b: f64, theta: f64) -> Self {
        let (tsin, tcos) = theta.sin_cos();
        Ellipse {
            x,
            y,
            a,
            b,
            theta,
            axis_a: Point::new(tcos, tsin),
            axis_b: Point::new(-tsin, tcos),
        }
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * (self.a * self.a + self.b * self.b).sqrt()
    }

    fn is_inside(&self, x: f64, y: f64) -> bool {
        let (x, y) = (x - self.x, y - self.y);
        let (x, y) = (
            self.theta.cos() * x - self.theta.sin() * y,
            self.theta.sin() * x + self.theta.cos() * y,
        );
        let (a, b) = (self.a, self.b);
        x * x / (a * a) + y * y / (b * b) <= 1.0
    }

    /// Distance of (px, py) point from the ellipse.
    /// Taken from:
    /// Chou, C.C., 2019. A closed-form general solution for the distance of point-to-ellipse in two dimensions. Journal of Interdisciplinary Mathematics, 22(3), pp.337-351.
    pub fn distance_from_perimeter(&self, px_: f64, py_: f64) -> f64 {
        let px_ = px_ - self.x;
        let py_ = py_ - self.y;
        let (px_, py_) = (
            (-self.theta).cos() * px_ - (-self.theta).sin() * py_,
            (-self.theta).sin() * px_ + (-self.theta).cos() * py_,
        );
        let xp = Complex::new(px_, 0.0);
        let yp = Complex::new(py_, 0.0);

        let (a, b) = (Complex::from(self.a), Complex::from(self.b));
        let c = a.powi(6) - 2.0 * a.powi(4) * b * b + a * a * b.powi(4)
            - a.powi(4) * xp * xp
            - a * a * b * b * yp * yp;
        let d = a * a - b * b;
        let e = a.powi(4) - 2.0 * a * a * b * b + b.powi(4) - a * a * xp * xp - b * b * yp * yp;
        let f = -108.0 * a.powi(8) * d.powi(4) * xp * xp
            + 108.0 * a.powi(10) * d * d * xp.powi(4)
            + 108.0 * a.powi(6) * d * d * xp * xp * c
            + 2.0 * c.powi(3);
        let g = (2.0.powf(1.0 / 3.0) * e * e)
            / (3.0 * a * a * xp * xp * (f + (f * f - 4.0 * c.powi(6)).sqrt()).powf(1.0 / 3.0));
        let h = ((f + (f * f - 4.0 * c.powi(6)).sqrt()).powf(1.0 / 3.0))
            / (3.0 * 2.0.powf(1.0 / 3.0) * a.powi(6) * xp * xp);
        let i = e / (a.powi(4) * xp * xp);
        let j = c / (3.0 * a.powi(6) * xp * xp);
        let k = d / (a * a * xp);
        let m = j + g + h;

        let X1 = 0.5
            * (k - Complex::sqrt(k * k - i + m)
                - Complex::sqrt(
                    2.0 * (k * k)
                        - i
                        - m
                        - (8.0 * k * (k * k - (2.0 / (a * a)) - i)
                            / (4.0 * Complex::sqrt(k * k - i + m))),
                ));
        let X2 = 0.5
            * (k - Complex::sqrt(k * k - i + m)
                + Complex::sqrt(
                    2.0 * (k * k)
                        - i
                        - m
                        - (8.0 * k * (k * k - (2.0 / (a * a)) - i)
                            / (4.0 * Complex::sqrt(k * k - i + m))),
                ));
        let X3 = 0.5
            * (k + Complex::sqrt(k * k - i + m)
                - Complex::sqrt(
                    2.0 * (k * k) - i - m
                        + (8.0 * k * (k * k - (2.0 / (a * a)) - i)
                            / (4.0 * Complex::sqrt(k * k - i + m))),
                ));
        let X4 = 0.5
            * (k + Complex::sqrt(k * k - i + m)
                + Complex::sqrt(
                    2.0 * (k * k) - i - m
                        + (8.0 * k * (k * k - (2.0 / (a * a)) - i)
                            / (4.0 * Complex::sqrt(k * k - i + m))),
                ));
        let Y1 = ((a * a) * xp * X1 + (b * b) - (a * a)) / ((b * b) * yp);
        let Y2 = ((a * a) * xp * X2 + (b * b) - (a * a)) / ((b * b) * yp);
        let Y3 = ((a * a) * xp * X3 + (b * b) - (a * a)) / ((b * b) * yp);
        let Y4 = ((a * a) * xp * X4 + (b * b) - (a * a)) / ((b * b) * yp);
        let (xt1, yt1) = (1.0 / X1, 1.0 / Y1);
        let (xt2, yt2) = (1.0 / X2, 1.0 / Y2);
        let (xt3, yt3) = (1.0 / X3, 1.0 / Y3);
        let (xt4, yt4) = (1.0 / X4, 1.0 / Y4);
        let PT1 = ((xt1 - xp).powi(2) + (yt1 - yp).powi(2)).sqrt();
        let PT2 = ((xt2 - xp).powi(2) + (yt2 - yp).powi(2)).sqrt();
        let PT3 = ((xt3 - xp).powi(2) + (yt3 - yp).powi(2)).sqrt();
        let PT4 = ((xt4 - xp).powi(2) + (yt4 - yp).powi(2)).sqrt();
        let lengths = [PT1, PT2, PT3, PT4]
            .iter()
            .filter(|l| l.im < 0.01 && l.im > -0.01)
            .map(|l| l.re)
            .collect::<Vec<_>>();

        if lengths.len() == 0 {
            return PT1.re.min(PT4.re);
        }

        let min_len = lengths
            .iter()
            .fold(f64::INFINITY, |prev, curr| prev.min(*curr));
        min_len
    }

    /// Distance of (px, py) point from the ellipse, iteratively estimated.
    /// Taken from:
    /// Chatfield, Carl. “Simple Method for Distance to Ellipse.” Wet Robots. Wet Robots, August 28, 2017. https://blog.chatfield.io/simple-method-for-distance-to-ellipse/.
    pub fn distance_from_perimeter_estimate(&self, px_: f64, py_: f64) -> f64 {
        let px_ = px_ - self.x;
        let py_ = py_ - self.y;
        let (px_orig, py_orig) = (
            px_ * self.axis_a.x + py_ * self.axis_a.y,
            px_ * self.axis_b.x + py_ * self.axis_b.y,
        );
        let px = px_orig.abs();
        let py = py_orig.abs();

        let mut t = py.atan2(px);

        let a = self.a;
        let b = self.b;

        let mut x = 0.0;
        let mut y = 0.0;

        for _ in 0..3 {
            x = a * t.cos();
            y = b * t.sin();

            let ex = (a * a - b * b) * t.cos().powi(3) / a;
            let ey = (b * b - a * a) * t.sin().powi(3) / b;
            let rx = x - ex;
            let ry = y - ey;
            let qx = px - ex;
            let qy = py - ey;
            let r = ry.hypot(rx);
            let q = qy.hypot(qx);
            let delta_c = r * ((rx * qy - ry * qx) / (r * q)).asin();
            let delta_t = delta_c / (a * a + b * b - x * x - y * y).sqrt();
            t += delta_t;
            t = (std::f64::consts::PI / 2.0).min(t.max(0.0));
        }

        let dx = x - px;
        let dy = y - py;

        (dx * dx + dy * dy).sqrt()
    }
}

/// Robust ellipse fit on noisy data, based on
/// Kaewapichai, W. and Kaewtrakulpong, P., 2008. Robust ellipse detection by fitting randomly selected edge patches. World Academy of Science, Engineering, and Technology, 48, pp.30-33.
pub fn robust_fit_ellipse(cont: &Vec<Point<f64>>, args: &FitArgs) -> Vec<Ellipse> {
    let mut center_of_mass = cont.iter().fold(Point::new(0.0, 0.0), |acc, p| acc + *p);
    center_of_mass.x /= cont.len() as f64;
    center_of_mass.y /= cont.len() as f64;
    center_of_mass.x -= args.rotation_center_x;
    center_of_mass.y -= args.rotation_center_y;
    if center_of_mass.norm() < args.detect_radius_min
        || center_of_mass.norm() > args.detect_radius_max
    {
        return vec![];
    }

    let mut cont = cont.clone();
    let err: f64 = 0.6;
    let d = args.dist_threshold;
    let pvalue = 1. - err.powf(5.0);
    let k = ((1. - pvalue).log2() / (1. - (1. - err).powf(5.0)).log2() * 2.) as usize;
    let min_r = args.radius_threshold;
    let min_fittness = args.min_fitness;
    let mut best_ellipses: Vec<Ellipse> = vec![];

    let mut prev_cont_len = 0;

    loop {
        if cont.len() < 30 || prev_cont_len == cont.len() {
            break;
        }
        prev_cont_len = cont.len();
        let mut samples: Vec<Vec<Point<f64>>> = Vec::with_capacity(k);
        for _ in 0..k {
            let mut sample: Vec<Point<f64>> = vec![];
            let mut added = 0;
            while added < 5 {
                let p1 = cont.get(fastrand::usize(..cont.len())).unwrap();
                let p2 = cont.get(fastrand::usize(..cont.len())).unwrap();
                let distance = (*p1 - *p2).norm();
                if distance > min_r * 2.0 && distance < min_r * 10.0 {
                    sample.extend(
                        cont.iter()
                            .filter(|&p| (*p - *p1).norm() <= min_r || (*p - *p2).norm() <= min_r)
                            .copied(),
                    );
                    added += 2;
                }
            }
            samples.push(sample);
        }

        let ellipse_filter = |e: &Ellipse| {
            let length = e.a.max(e.b) * 2.0;
            let width = e.a.min(e.b) * 2.0;
            let pred11 = length >= args.min_length;
            let pred12 = length <= args.max_length;
            let pred13 = width >= args.min_width;
            let pred14 = width <= args.max_width;
            let aspect = length / width;
            let pred21 = aspect >= args.min_aspect_ratio;
            let pred22 = aspect <= args.max_aspect_ratio;
            return pred11 && pred12 && pred13 && pred14 && pred21 && pred22;
        };

        let ellipses = samples
            .iter()
            .filter_map(|s| fit_ellipse_dls(&s[..]))
            .filter(ellipse_filter)
            .collect::<Vec<_>>();

        let fitnesses = ellipses
            .iter()
            .map(|e| {
                cont.iter()
                    .filter(|point| e.distance_from_perimeter_estimate(point.x, point.y) <= d)
                    .count() as f64
                    / e.perimeter()
            })
            .collect::<Vec<_>>();

        if fitnesses.len() == 0 || !fitnesses.iter().any(|&f| f >= min_fittness) {
            break;
        }

        let argmax = fitnesses
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap_or(0);
        let best_ellipse = ellipses[argmax].clone();
        best_ellipses.push(best_ellipse);

        cont = cont
            .iter()
            .filter(|point| {
                let distance = best_ellipse.distance_from_perimeter_estimate(point.x, point.y);
                distance >= d
            })
            .copied()
            .collect();
    }

    best_ellipses
}

trait Norm {
    fn norm(&self) -> f64;
}

impl Norm for Point<f64> {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
