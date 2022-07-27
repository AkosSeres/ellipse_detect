use imageproc::point::Point;
use nalgebra::{DVector, Matrix6, MatrixXx6};

/// Fits an ellipse to the given points using the direct least squares method.
/// Based on:
/// Halır, R. and Flusser, J., 1998, February. Numerically stable direct least squares fitting of ellipses. In Proc. 6th International Conference in Central Europe on Computer Graphics and Visualization. WSCG (Vol. 98, pp. 125-132). Citeseer.
pub fn fit_ellipse_dls(points: &[Point<f64>]) {
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
        return;
    }
    let to_eig = S * C;
    // Need to get the eigenvalues and eigenvectors of the matrix.
    todo!();
}
