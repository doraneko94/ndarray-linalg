#![allow(non_snake_case)]
use ndarray::{stack, Array, Array1, Array2, ArrayBase, Axis, Data, Ix1, Ix2};
use ndarray_linalg::{random, Solve};
use ndarray_rand::RandomExt;
use ndarray_rand::distributions::StandardNormal;

/// The simple linear regression model is
///     y = bX + e  where e ~ N(0, sigma^2 * I)
/// In probabilistic terms this corresponds to
///     y - bX ~ N(0, sigma^2 * I)
///     y | X, b ~ N(bX, sigma^2 * I)
/// The loss for the model is simply the squared error between the model
/// predictions and the true values:
///     Loss = ||y - bX||^2
/// The maximum likelihood estimation for the model parameters `beta` can be computed
/// in closed form via the normal equation:
///     b = (X^T X)^{-1} X^T y
/// where (X^T X)^{-1} X^T is known as the pseudoinverse or Moore-Penrose inverse.
///
/// Adapted from: https://github.com/xinscrs/numpy-ml
struct LinearRegression {
    pub beta: Option<Array1<f64>>,
    fit_intercept: bool,
}

impl LinearRegression {
    fn new(fit_intercept: bool) -> LinearRegression {
        LinearRegression {
            beta: None,
            fit_intercept,
        }
    }

    fn fit<A, B>(&mut self, X: ArrayBase<A, Ix2>, y: ArrayBase<B, Ix1>)
    where
        A: Data<Elem = f64>,
        B: Data<Elem = f64>,
    {
        let (n_samples, _) = X.dim();

        // Check that our inputs have compatible shapes
        assert_eq!(y.dim(), n_samples);

        // If we are fitting the intercept, we need an additional column
        self.beta = if self.fit_intercept {
            let dummy_column: Array<f64, _> = Array::ones((n_samples, 1));
            let X = stack(Axis(1), &[dummy_column.view(), X.view()]).unwrap();
            Some(LinearRegression::solve_normal_equation(X, y))
        } else {
            Some(LinearRegression::solve_normal_equation(X, y))
        };
    }

    fn solve_normal_equation<A, B>(X: ArrayBase<A, Ix2>, y: ArrayBase<B, Ix1>) -> Array1<f64>
    where
        A: Data<Elem = f64>,
        B: Data<Elem = f64>,
    {
        let rhs = X.t().dot(&y);
        let linear_operator = X.t().dot(&X);
        linear_operator.solve_into(rhs).unwrap()
    }

    fn predict<A>(&self, X: &ArrayBase<A, Ix2>) -> Array1<f64>
    where
        A: Data<Elem = f64>,
    {
        let (n_samples, _) = X.dim();

        // If we are fitting the intercept, we need an additional column
        if self.fit_intercept {
            let dummy_column: Array<f64, _> = Array::ones((n_samples, 1));
            let X = stack(Axis(1), &[dummy_column.view(), X.view()]).unwrap();
            self._predict(&X)
        } else {
            self._predict(X)
        }
    }

    fn _predict<A>(&self, X: &ArrayBase<A, Ix2>) -> Array1<f64>
    where
        A: Data<Elem = f64>,
    {
        match &self.beta {
            None => panic!("The linear regression estimator has to be fitted first!"),
            Some(beta) => X.dot(beta),
        }
    }
}

fn get_data(n_samples: usize, n_features: usize) -> (Array2<f64>, Array1<f64>) {
    let shape = (n_samples, n_features);
    let noise = Array::random(n_samples, StandardNormal);
    let beta: Array1<f64> = random(n_features) * 10.;
    println!("Beta used to generate target variable: {:.3}", beta);

    let X = random(shape);
    let y = X.dot(&beta) + noise;
    (X, y)
}

pub fn main() {
    let n_train_samples = 5000;
    let n_test_samples = 1000;
    let n_features = 3;

    let (X, y) = get_data(n_train_samples + n_test_samples, n_features);
    let (X_train, X_test) = X.view().split_at(Axis(0), n_train_samples);
    let (y_train, _y_test) = y.view().split_at(Axis(0), n_train_samples);

    let mut linear_regressor = LinearRegression::new(false);
    linear_regressor.fit(X_train, y_train);

    let _test_predictions = linear_regressor.predict(&X_test);
    println!(
        "Beta estimated from the training data: {:.3}",
        linear_regressor.beta.unwrap()
    );
}
