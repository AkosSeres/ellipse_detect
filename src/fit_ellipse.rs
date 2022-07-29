use imageproc::point::Point;
use nalgebra::{DVector, Matrix6, MatrixXx6, Vector6};

use crate::robust_fit::Ellipse;

/// Fits an ellipse to the given points using the direct least squares method.
/// Based on:
/// Halır, R. and Flusser, J., 1998, February. Numerically stable direct least squares fitting of ellipses. In Proc. 6th International Conference in Central Europe on Computer Graphics and Visualization. WSCG (Vol. 98, pp. 125-132). Citeseer.
pub fn fit_ellipse_dls(points: &[Point<f64>]) -> Option<Ellipse> {
    let D: MatrixXx6<f64> = MatrixXx6::from_columns(&[
        DVector::from_iterator(points.len(), points.iter().map(|p| p.x * p.x)),
        DVector::from_iterator(points.len(), points.iter().map(|p| p.x * p.y)),
        DVector::from_iterator(points.len(), points.iter().map(|p| p.y * p.y)),
        DVector::from_iterator(points.len(), points.iter().map(|p| p.x)),
        DVector::from_iterator(points.len(), points.iter().map(|p| p.y)),
        DVector::from_element(points.len(), 1.0),
    ]);
    let mut S = D.transpose() * D;
    let mut C = Matrix6::from_element(0.0);
    C.m13 = 2.0;
    C.m22 = -1.0;
    C.m31 = 2.0;
    let is_inverse_success = S.try_inverse_mut();
    if !is_inverse_success {
        return None;
    }
    let to_eig_orig = S * C;
    // Find eigenvlues using the QR algorithm
    let mut to_eig = to_eig_orig.hessenberg().h();
    for _ in 0..20 {
        let qr = to_eig.qr();
        to_eig = qr.r() * qr.q();
    }
    let eigvals = [
        to_eig.m11, to_eig.m22, to_eig.m33, to_eig.m44, to_eig.m55, to_eig.m66,
    ];
    let pos_eigval = eigvals.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    // Then find the eigenvector using the inverse iteration method
    let mut eigvec = Vector6::from_element(1.0).normalize();
    let iter_mat = (to_eig_orig - pos_eigval * Matrix6::identity()).try_inverse()?;
    for _ in 0..20 {
        eigvec = iter_mat * eigvec;
        eigvec.normalize_mut();
    }
    let factor = eigvec.transpose() * C * eigvec;
    let eigvec = eigvec / factor.x.sqrt();

    let [a, b, c, d, e, f] = eigvec.data.0[0];
    let axis_a = (2.0
        * (a * e * e + c * d * d - b * d * e - f)
        * ((a + c) + ((a - c).powi(2) + b * b).sqrt()))
    .sqrt();
    let axis_b = (2.0
        * (a * e * e + c * d * d - b * d * e - f)
        * ((a + c) - ((a - c).powi(2) + b * b).sqrt()))
    .sqrt();
    let center_x = b * e - 2.0 * c * d;
    let center_y = b * d - 2.0 * a * e;
    let theta = if b != 0.0 {
        ((c - a - ((a - c).powi(2) + b * b).sqrt()) / b).atan()
    } else {
        if a < c {
            0.0
        } else {
            std::f64::consts::PI / 2.0
        }
    };

    Some(Ellipse::new(center_x, center_y, axis_a, axis_b, theta))
}
