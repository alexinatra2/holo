use std::num::ParseFloatError;

use regex::Regex;
use std::str::FromStr;

// Funktion zum Parsen der Polynomausdrücke (z.B. "3z + 2z^2 + 3")
pub fn parse_polynomial_expression(input: &str) -> Result<Vec<f64>, ParseFloatError> {
    let term_regex = Regex::new(r"([+-]?\d*\.?\d*)z(?:\^(\d+))?").unwrap();
    let constant_regex = Regex::new(r"([+-]?\d*\.?\d+)$").unwrap();

    let mut coefficients = vec![];

    // Find all polynomial terms like "3z^2", "2z", etc.
    for cap in term_regex.captures_iter(input) {
        let coefficient_str = cap.get(1).map_or("1", |m| m.as_str()); // Default coefficient is 1 if omitted
        let exponent_str = cap.get(2).map_or("1", |m| m.as_str()); // Default exponent is 1 if omitted
        let coefficient: f64 = if coefficient_str == "+" || coefficient_str.is_empty() {
            1.0
        } else if coefficient_str == "-" {
            -1.0
        } else {
            f64::from_str(coefficient_str)?
        };
        let exponent: usize = exponent_str.parse().unwrap_or(1);

        // Ensure the coefficients vector is large enough
        if coefficients.len() <= exponent {
            coefficients.resize(exponent + 1, 0.0);
        }

        // Add the coefficient to the corresponding power of z
        coefficients[exponent] = coefficient;
    }

    // Find any constant term (no z)
    if let Some(cap) = constant_regex.captures(input) {
        let constant: f64 = f64::from_str(cap.get(1).unwrap().as_str())?;
        if coefficients.is_empty() {
            coefficients.push(constant);
        } else {
            coefficients[0] = constant;
        }
    }

    Ok(coefficients)
}
