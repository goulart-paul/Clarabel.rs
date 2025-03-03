#![allow(non_snake_case)]

use crate::algebra::{
    DenseMatrix, FloatT, Matrix, MatrixTriangle, MultiplySYMV, ShapedMatrix, Symmetric,
};

impl<'a, T> MultiplySYMV for Symmetric<'a, Matrix<T>>
where
    T: FloatT,
{
    type T = T;
    // implements y = αA*x + βy
    fn symv(&self, x: &[Self::T], y: &mut [Self::T], α: Self::T, β: Self::T) {
        let (m, n) = self.size();
        assert!(m == n);

        // standard BLAS ?symv arguments for computing matrix-vector product
        let uplo = MatrixTriangle::Triu.as_blas_char();
        let n = n.try_into().unwrap();
        let a = self.src.data();
        let lda = n;
        let incx = 1;
        let incy = 1;
        T::xsymv(uplo, n, α, a, lda, x, incx, β, y, incy);
    }
}

#[test]
fn test_gsymv() {
    use crate::algebra::Matrix;
    let (m, n) = (3, 3);
    let a = vec![1.0, 0.0, 0.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0];
    let A = Matrix::new((m, n), a);

    let x = vec![1., -2., 3.];
    let mut y = vec![-4., -1., 3.];
    A.sym().symv(&x, &mut y, 2.0, 3.0);
    assert!(y == [6.0, 19.0, 33.0]);
}
