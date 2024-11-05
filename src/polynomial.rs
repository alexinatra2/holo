use num_complex::Complex;

// Function to construct a holomorphic polynomial from the coefficients
pub fn construct_polynomial(coefficients: Vec<f64>) -> impl Fn(Complex<f64>) -> Complex<f64> {
    move |z: Complex<f64>| {
        let mut result = Complex::new(0.0, 0.0);
        let mut z_pow = Complex::new(1.0, 0.0); // This is z^0 initially
        for &coef in &coefficients {
            result += coef * z_pow;
            z_pow *= z; // Increment to the next power of z
        }
        result
    }
}

// Function to construct a rational function from numerator and denominator coefficients
pub fn construct_rational_function(
    numerator: Vec<f64>,
    denominator: Vec<f64>,
) -> impl Fn(Complex<f64>) -> Complex<f64> {
    move |z: Complex<f64>| {
        let mut num_result = Complex::new(0.0, 0.0);
        let mut denom_result = Complex::new(0.0, 0.0);

        // Calculate numerator
        let mut z_pow = Complex::new(1.0, 0.0); // Start with z^0
        for &coef in &numerator {
            num_result += coef * z_pow;
            z_pow *= z;
        }

        // Calculate denominator
        z_pow = Complex::new(1.0, 0.0); // Reset to z^0
        for &coef in &denominator {
            denom_result += coef * z_pow;
            z_pow *= z;
        }

        // Handle division by zero or very small denominator values
        if denom_result.norm() < 1e-6 {
            Complex::new(0.0, 0.0) // Return 0 if division by zero occurs
        } else {
            num_result / denom_result
        }
    }
}
