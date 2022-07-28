use opencv::{
    core::{Point_, RotatedRect, Vector},
    imgproc::fit_ellipse_direct,
    prelude::RotatedRectTraitConst,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub a: f64,
    pub b: f64,
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

impl From<RotatedRect> for Ellipse {
    fn from(rect: RotatedRect) -> Self {
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

impl Ellipse {
    fn perimeter(&self) -> f64 {
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
    /// Based on:
    /// Chatfield, Carl. “Simple Method for Distance to Ellipse.” Wet Robots. Wet Robots, August 28, 2017. https://blog.chatfield.io/simple-method-for-distance-to-ellipse/.
    fn distance_from_perimeter(&self, px_: f64, py_: f64) -> f64 {
        let px_ = px_ - self.x;
        let py_ = py_ - self.y;
        let (px_, py_) = (
            (-self.theta).cos() * px_ - (-self.theta).sin() * py_,
            (-self.theta).sin() * px_ + (-self.theta).cos() * py_,
        );
        let px = px_.abs();
        let py = py_.abs();

        let mut t = if self.is_inside(px, py) {
            py.atan2(px)
        } else {
            std::f64::consts::PI / 4.0
        };

        let a = self.a;
        let b = self.b;
        let mut x = 0.0;
        let mut y = 0.0;

        for _ in 0..10 {
            x = a * t.cos();
            y = b * t.sin();
            let ex = (a * a - b * b) * (t.cos().powi(3)) / a;
            let ey = (b * b - a * a) * (t.sin().powi(3)) / b;
            let rx = x - ex;
            let ry = y - ey;
            let qx = px - ex;
            let qy = py - ey;
            let r = ry.hypot(rx);
            let q = qy.hypot(qx);
            let delta_c = r * ((rx * qy - ry * qx) / (r * q)).asin();
            let delta_t = delta_c / (a * a + b * b - x * x - y * y).sqrt();
            t += delta_t;
            t = (std::f64::consts::PI * 0.5).min(t.max(0.0));
        }

        let dx = x.copysign(px_);
        let dy = y.copysign(py_);
        (dx * dx + dy * dy).sqrt()
    }
}

/// Robust ellipse fit on noisy data, based on
/// Kaewapichai, W. and Kaewtrakulpong, P., 2008. Robust ellipse detection by fitting randomly selected edge patches. World Academy of Science, Engineering, and Technology, 48, pp.30-33.
pub fn robust_fit_ellipse(cont: &Vector<Point_<i32>>) -> Vec<Ellipse> {
    let mut cont = cont.clone();
    let err: f64 = 0.6;
    let d = 3.0;
    let pvalue = 1. - err.powf(5.0);
    let K = ((1. - pvalue).log2() / (1. - (1. - err).powf(5.0)).log2() * 2.) as usize;
    let min_r = 6.0;
    let min_fittness = 0.3;
    let mut best_ellipses: Vec<Ellipse> = vec![];

    let mut prev_cont_len = 0;

    loop {
        if cont.len() < 30 || prev_cont_len == cont.len() {
            break;
        }
        prev_cont_len = cont.len();
        let mut samples: Vec<Vector<Point_<i32>>> = Vec::with_capacity(K);
        for _ in 0..K {
            let mut sample: Vector<Point_<i32>> = Vector::new();
            let mut added = 0;
            while added < 5 {
                let p1 = cont.get(fastrand::usize(..cont.len())).unwrap();
                let p2 = cont.get(fastrand::usize(..cont.len())).unwrap();
                let distance = (p1 - p2).norm();
                if distance > min_r * 2.0 && distance < 40.0 {
                    sample.push(p1.clone());
                    sample.push(p2.clone());
                    sample.extend(
                        cont.iter()
                            .filter(|&p| (p - p1).norm() <= min_r || (p - p2).norm() > min_r)
                            .filter(|&p| p != p1 && p != p2),
                    );
                    added += 2;
                }
            }
            samples.push(sample);
        }

        let ellipse_filter = |e: &Ellipse| {
            let pred11 = e.a >= 45.0 || e.b >= 45.0;
            let pred12 = e.a <= 65.0 || e.b <= 65.0;
            let aspect = if e.a > e.b { e.a / e.b } else { e.b / e.a };
            let pred2 = aspect >= 3.0;
            return pred11 && pred12 && pred2;
        };
        let ellipses = samples
            .iter()
            .map(|s| Ellipse::from(fit_ellipse_direct(s).unwrap()))
            .filter(ellipse_filter)
            .collect::<Vec<_>>();

        let perimeters: Vec<f64> = ellipses.iter().map(Ellipse::perimeter).collect();
        let fitnesses = ellipses
            .iter()
            .zip(perimeters.into_iter())
            .map(|(e, p)| {
                cont.iter()
                    .filter(|point| e.distance_from_perimeter(point.x as f64, point.y as f64) <= d)
                    .count() as f64
                    / p
            })
            .collect::<Vec<f64>>();

        let dists = cont
            .iter()
            .map(|p| ellipses[6].distance_from_perimeter(p.x as f64, p.y as f64))
            .collect::<Vec<f64>>();
        println!("{:?}", dists);
        println!("{:?}", fitnesses);

        if fitnesses.len() == 0 || fitnesses.iter().any(|&f| f <= min_fittness) {
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
                let distance = best_ellipse.distance_from_perimeter(point.x as f64, point.y as f64);
                distance > d
            })
            .collect();
    }

    best_ellipses
}
