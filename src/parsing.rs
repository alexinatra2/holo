use std::str::FromStr;

use num_complex::Complex;

/// Parses a polynomial expression from a string of space-separated coefficients
/// and returns them as a vector of coefficients.
pub fn parse_polynomial_expression(input: &str) -> Result<Vec<f64>, String> {
    input
        .split_whitespace()
        .map(f64::from_str)
        .collect::<Result<Vec<f64>, _>>()
        .map_err(|_| "Failed to parse coefficients".to_string())
}

/// Constructs a polynomial function from a list of coefficients.
fn construct_polynomial(coefficients: Vec<f64>) -> impl Fn(Complex<f64>) -> Complex<f64> {
    move |z: Complex<f64>| {
        coefficients
            .iter()
            .rev()
            .enumerate()
            .fold(Complex::new(0.0, 0.0), |acc, (i, &coef)| {
                acc + coef * z.powi(i as i32)
            })
    }
}

/// Constructs a rational function from numerator and denominator coefficients.
fn construct_rational_function(
    numerator: Vec<f64>,
    denominator: Vec<f64>,
) -> impl Fn(Complex<f64>) -> Complex<f64> {
    let numerator_fn = construct_polynomial(numerator);
    let denominator_fn = construct_polynomial(denominator);

    move |z: Complex<f64>| numerator_fn(z) / denominator_fn(z)
}

/// Parses an input string to determine and return a closure representing
/// either a polynomial or a rational function.
pub fn parse_holomorphic_function(
    input: &str,
) -> Result<Box<dyn Fn(Complex<f64>) -> Complex<f64>>, String> {
    if input.contains('/') {
        // Split the input by '/' to separate numerator and denominator.
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid input format for rational function".to_string());
        }

        // Parse numerator and denominator coefficients.
        let numerator = parse_polynomial_expression(parts[0])?;
        let denominator = parse_polynomial_expression(parts[1])?;

        // Create and return the rational function as a closure.
        Ok(Box::new(construct_rational_function(
            numerator,
            denominator,
        )))
    } else {
        // Parse as a polynomial function.
        let coefficients = parse_polynomial_expression(input)?;

        // Create and return the polynomial function as a closure.
        Ok(Box::new(construct_polynomial(coefficients)))
    }
}
