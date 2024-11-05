use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use num_complex::{Complex, ComplexFloat};
use std::str::FromStr;

type CFunc = Box<dyn Fn(Complex<f64>) -> Complex<f64>>;

/// Parses the variable `z` with optional exponentiation, e.g., "z^2".
fn parse_variable(input: &str) -> IResult<&str, CFunc> {
    let (input, _) = tag("z")(input)?;
    let (input, exponent) = opt(preceded(char('^'), nom::character::complete::digit1))(input)?;

    // Parse the exponent as an integer, defaulting to 1 if not provided.
    let exponent: u32 = exponent.map_or(1, |e| e.parse().unwrap_or(1));

    Ok((
        input,
        Box::new(move |z: Complex<f64>| z.powf(exponent as f64)),
    ))
}

/// Parses constants, e.g., "3.0", and returns them as closures.
fn parse_constant(input: &str) -> IResult<&str, CFunc> {
    let (input, constant) = nom::combinator::recognize(tuple((
        opt(char('-')),
        nom::character::complete::digit1,
        opt(tuple((char('.'), nom::character::complete::digit1))),
    )))(input)?;

    let constant = Complex::from_str(constant).unwrap_or(Complex::new(0.0, 0.0));
    Ok((input, Box::new(move |_| constant)))
}

/// Parses functions like `cos(z)` or `exp(z)`.
fn parse_function(input: &str) -> IResult<&str, CFunc> {
    let (input, func_name) = alt((
        tag("cos"),
        tag("sin"),
        tag("exp"),
        tag("tan"),
        tag("sec"),
        tag("csc"),
        tag("cot"),
        tag("log"),
        tag("sqrt"),
        tag("abs"),
        tag("arg"),
        tag("conj"),
        tag("re"),
        tag("im"),
    ))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, inner_func) = delimited(char('('), parse_expression, char(')'))(input)?;

    let function: fn(Complex<f64>) -> Complex<f64> = match func_name {
        "cos" => Complex::cos,
        "sin" => Complex::sin,
        "exp" => Complex::exp,
        "tan" => Complex::tan,
        "sec" => |z: Complex<f64>| Complex::new(1.0, 0.0) / Complex::cos(z),
        "csc" => |z: Complex<f64>| Complex::new(1.0, 0.0) / Complex::sin(z),
        "cot" => |z: Complex<f64>| Complex::new(1.0, 0.0) / Complex::tan(z),
        "log" => Complex::ln,
        "sqrt" => Complex::sqrt,
        "abs" => |z: Complex<f64>| Complex::new(z.re.abs(), z.im.abs()), // Abs as a complex function
        "arg" => |z: Complex<f64>| Complex::new(z.arg(), 0.0), // Argument of the complex number
        "conj" => |z: Complex<f64>| z.conj(),
        "re" => |z: Complex<f64>| Complex::new(z.re, 0.0),
        "im" => |z: Complex<f64>| Complex::new(0.0, z.im),
        _ => unreachable!(),
    };

    Ok((
        input,
        Box::new(move |z: Complex<f64>| function(inner_func(z))),
    ))
}

/// Parses terms connected by addition, allowing `z + exp(z)`.
fn parse_term(input: &str) -> IResult<&str, CFunc> {
    let (input, initial) = parse_factor(input)?;
    let (input, others) = separated_list0(preceded(multispace0, char('+')), parse_factor)(input)?;

    Ok((
        input,
        Box::new(move |z: Complex<f64>| {
            let mut sum = initial(z);
            for term in &others {
                sum += term(z);
            }
            sum
        }),
    ))
}

/// Parses multiplication between factors, allowing `2 * z` or `exp(z) * cos(z)`.
fn parse_factor(input: &str) -> IResult<&str, CFunc> {
    let (input, initial) = alt((parse_constant, parse_variable, parse_function))(input)?;
    let (input, others) = separated_list0(
        preceded(multispace0, char('*')),
        alt((parse_constant, parse_variable, parse_function)),
    )(input)?;

    Ok((
        input,
        Box::new(move |z: Complex<f64>| {
            let mut product = initial(z);
            for factor in &others {
                product *= factor(z);
            }
            product
        }),
    ))
}

/// Parses an entire expression like `z + exp(z)`.
pub fn parse_expression(input: &str) -> IResult<&str, CFunc> {
    parse_term(input)
}
