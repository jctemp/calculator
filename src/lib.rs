use std::str::FromStr;

use pest::Parser;
use pest_derive::Parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Parser)]
#[grammar = "arithmetic.pest"]
pub struct ArithmeticParser;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorSymbol {
    Add,
    Sub,
    Mul,
    Div,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operator(OperatorSymbol, fn(f64, f64) -> f64);

impl FromStr for Operator {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "+" => Ok(Operator(OperatorSymbol::Add, |a, b| a + b)),
            "-" => Ok(Operator(OperatorSymbol::Sub, |a, b| a - b)),
            "*" => Ok(Operator(OperatorSymbol::Mul, |a, b| a * b)),
            "/" => Ok(Operator(OperatorSymbol::Div, |a, b| a / b)),
            _ => Err(format!("Operator '{}' is not supported.", value)),
        }
    }
}

#[wasm_bindgen]
pub fn test(input: &str) -> f64 {
    let expression = ArithmeticParser::parse(Rule::expression, input)
        .expect("Expression should be parsable.")
        .into_iter()
        .next()
        .expect("At least one expression should be contained in the parsed string.");

    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    for token in expression.into_inner() {
        let operator = match operators.last() {
            Some(&Operator(OperatorSymbol::Mul, _)) | Some(&Operator(OperatorSymbol::Div, _)) => {
                operators.pop()
            }
            _ => None,
        };

        match token.as_rule() {
            Rule::float => {
                let number: f64 = token.as_str().parse().expect(
                    "Parsed float should not fail as it has correct form. Maybe too large or so.",
                );

                if let Some(operator) = operator {
                    let prev = numbers.pop().expect("Must have a predecessor.");
                    numbers.push(operator.1(prev, number));
                } else {
                    numbers.push(number);
                }
            }
            Rule::operator => {
                operators.push(Operator::from_str(token.as_str()).unwrap());
            }
            _ => panic!("This should not happen."),
        };
    }

    
    for (idx, ops) in operators.iter_mut().enumerate() {
        if ops.0 == OperatorSymbol::Sub {
            numbers[idx + 1] *= -1.0;
            ops.0 = OperatorSymbol::Add;
            ops.1 = |a, b| a + b;
        }
    }

    let mut ops = String::new();
    let mut numbers = numbers;
    let mut operators = operators;
    while let Some(op) = operators.pop() {
        let b = numbers.pop().unwrap();
        let a = numbers.pop().unwrap();
        ops += &format!("\n{:?} {:?} {:?}", a, op.0, b);
        numbers.push(op.1(a, b));
    }

    numbers[0]
}
