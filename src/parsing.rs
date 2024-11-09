use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use num_complex::Complex;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Number(f64),
    Variable,
    UnaryOp {
        op: char,
        expr: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
    Function {
        func: String,
        expr: Box<Expr>,
    },
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

#[allow(dead_code)]
fn parse(input: &str) -> Expr {
    let (_, expression) = parse_expression(input).unwrap();
    expression
}

fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    let (input, expr) = many0(pair(ws(alt((char('+'), char('-')))), parse_term))(input)?;

    Ok((
        input,
        expr.into_iter()
            .fold(init, |acc, (op, term)| Expr::BinaryOp {
                left: Box::new(acc),
                op,
                right: Box::new(term),
            }),
    ))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    let (input, expr) = many0(pair(ws(alt((char('*'), char('/')))), parse_factor))(input)?;

    Ok((
        input,
        expr.into_iter()
            .fold(init, |acc, (op, factor)| Expr::BinaryOp {
                left: Box::new(acc),
                op,
                right: Box::new(factor),
            }),
    ))
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_primary(input)?;
    let (input, expr) = many0(pair(ws(char('^')), parse_primary))(input)?;

    Ok((
        input,
        expr.into_iter()
            .fold(init, |acc, (_, primary)| Expr::BinaryOp {
                left: Box::new(acc),
                op: '^',
                right: Box::new(primary),
            }),
    ))
}

fn parse_primary(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_number,
        parse_variable,
        parse_function,
        delimited(ws(char('(')), parse_expression, ws(char(')'))),
        parse_unary,
    ))(input)
}

fn parse_unary(input: &str) -> IResult<&str, Expr> {
    let (input, op) = ws(char('-'))(input)?;
    let (input, expr) = parse_factor(input)?;
    Ok((
        input,
        Expr::UnaryOp {
            op,
            expr: Box::new(expr),
        },
    ))
}

fn parse_function(input: &str) -> IResult<&str, Expr> {
    let (input, func) = ws(alt((tag("sin"), tag("cos"), tag("tan"))))(input)?;
    let (input, expr) = delimited(ws(char('(')), parse_expression, ws(char(')')))(input)?;
    Ok((
        input,
        Expr::Function {
            func: func.to_string(),
            expr: Box::new(expr),
        },
    ))
}

fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(ws(tag("z")), |_| Expr::Variable)(input)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, num_str) = recognize(pair(
        opt(ws(char('-'))),
        alt((recognize(pair(digit1, pair(char('.'), digit1))), digit1)),
    ))(input)?;
    let num = f64::from_str(num_str).unwrap();
    Ok((input, Expr::Number(num)))
}

pub fn parse_and_generate_closure(
    input: &str,
) -> Result<Box<dyn Fn(Complex<f64>) -> Complex<f64>>, String> {
    match parse_expression(input) {
        Ok((_, expr)) => {
            let closure = move |z: Complex<f64>| expr.evaluate(z);
            Ok(Box::new(closure))
        }
        Err(err) => Err(format!("Failed to parse input: {:?}", err)),
    }
}

impl Expr {
    fn evaluate(&self, z: Complex<f64>) -> Complex<f64> {
        match self {
            Expr::Number(n) => Complex::new(*n, 0.0),
            Expr::Variable => z,
            Expr::UnaryOp { op, expr } => {
                let val = expr.evaluate(z);
                match *op {
                    '-' => -val,
                    _ => val,
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_val = left.evaluate(z);
                let right_val = right.evaluate(z);
                match *op {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => left_val / right_val,
                    _ => left_val, // Or handle other operations
                }
            }
            Expr::Function { func, expr } => {
                let val = expr.evaluate(z);
                match func.as_str() {
                    "sin" => val.sin(),
                    "cos" => val.cos(),
                    "tan" => val.tan(),
                    _ => val,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complex_num_tests() -> Vec<Complex<f64>> {
        vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(1.0, 1.0),
            Complex::new(-1.0, -1.0),
            Complex::new(2.0, -2.0),
            Complex::new(-2.0, 2.0),
            Complex::new(3.0, 3.0),
            Complex::new(-3.0, -3.0),
            Complex::new(1.5, -1.5),
        ]
    }

    #[test]
    fn test_simple_number_expressions() {
        assert_eq!(parse("5"), Expr::Number(5.0));
        assert_eq!(parse("-3"), Expr::Number(-3.0));
        assert_eq!(parse("z"), Expr::Variable);
    }

    #[test]
    fn test_simple_arithmetic_expressions() {
        assert_eq!(
            parse("5 + 3"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(5.0)),
                op: '+',
                right: Box::new(Expr::Number(3.0)),
            }
        );
        assert_eq!(
            parse("5 - 3"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(5.0)),
                op: '-',
                right: Box::new(Expr::Number(3.0)),
            }
        );
        assert_eq!(
            parse("4 * 2"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(4.0)),
                op: '*',
                right: Box::new(Expr::Number(2.0)),
            }
        );
        assert_eq!(
            parse("8 / 4"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(8.0)),
                op: '/',
                right: Box::new(Expr::Number(4.0)),
            }
        );
        assert_eq!(
            parse("2 ^ 3"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(2.0)),
                op: '^',
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_nested_expressions_without_functions() {
        assert_eq!(
            parse("5 + 3 * 2"),
            Expr::BinaryOp {
                left: Box::new(Expr::Number(5.0)),
                op: '+',
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3.0)),
                    op: '*',
                    right: Box::new(Expr::Number(2.0)),
                }),
            }
        );
        assert_eq!(
            parse("(5 + 3) * 2"),
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(5.0)),
                    op: '+',
                    right: Box::new(Expr::Number(3.0)),
                }),
                op: '*',
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_nested_expressions_with_functions() {
        assert_eq!(
            parse("sin(5) + 3"),
            Expr::BinaryOp {
                left: Box::new(Expr::Function {
                    func: "sin".to_string(),
                    expr: Box::new(Expr::Number(5.0)),
                }),
                op: '+',
                right: Box::new(Expr::Number(3.0)),
            }
        );
        assert_eq!(
            parse("cos(z ^ 2) + 1"),
            Expr::BinaryOp {
                left: Box::new(Expr::Function {
                    func: "cos".to_string(),
                    expr: Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Variable),
                        op: '^',
                        right: Box::new(Expr::Number(2.0)),
                    }),
                }),
                op: '+',
                right: Box::new(Expr::Number(1.0)),
            }
        );
    }

    #[test]
    fn test_expression_evaluation() {
        let complex_numbers = complex_num_tests();

        // Testing the expression "sin(z) + 1" for each complex number
        let expression = parse("sin(z) + 1");
        for z in complex_numbers.iter() {
            let result = evaluate_expr(&expression, *z);
            let expected = Complex::new(z.im.sin() + 1.0, z.re.cos()); // assuming sin and cos output for example
            assert_eq!(result, expected, "Failed for z = {:?}", z);
        }

        // Testing the expression "(z ^ 2) - 3 * z + 1"
        let expression = parse("(z ^ 2) - 3 * z + 1");
        for z in complex_numbers.iter() {
            let result = evaluate_expr(&expression, *z);
            let expected = z * z - Complex::new(3.0, 0.0) * *z + Complex::new(1.0, 0.0);
            assert_eq!(result, expected, "Failed for z = {:?}", z);
        }
    }
}
