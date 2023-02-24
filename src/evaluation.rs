use anyhow::Result;
use pest::iterators::Pair;
use std::str::FromStr;

use crate::{errors::ArithmeticError, parsing::*};

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::str::FromStr for Operator {
    type Err = ArithmeticError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(ArithmeticError::InvalidToken(s.to_string())),
        }
    }
}

pub fn evaluate(expression: &Pair<Rule>) -> Result<f64> {
    let mut numbers = expression
        .clone()
        .into_inner()
        .filter(|token| token.as_rule() == Rule::float)
        .map(|number| number.as_str().parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()?;

    let mut operators = expression
        .clone()
        .into_inner()
        .filter(|token| token.as_rule() == Rule::operator)
        .map(|operator| Operator::from_str(operator.as_str()))
        .collect::<Result<Vec<Operator>, _>>()?;

    for (index, operator) in operators.iter_mut().enumerate() {
        match operator {
            Operator::Sub => {
                let number = match numbers.get_mut(index + 1) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };

                *operator = Operator::Add;
                *number *= -1.0;
            }
            _ => { /* NO-OP */ }
        }
    }

    let mut marker = Vec::new();
    for (index, operator) in operators.iter_mut().enumerate() {
        match operator {
            Operator::Mul => {
                let a = match numbers.get(index) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };

                let b = match numbers.get(index + 1) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };
                let result = a * b;

                let number = match numbers.get_mut(index + 1) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };
                *number = result;
                marker.push(index);
            }
            Operator::Div => {
                let a = match numbers.get(index) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };

                let b = match numbers.get(index + 1) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };
                let result = a / b;

                let number = match numbers.get_mut(index + 1) {
                    Some(num) => num,
                    None => return Err(ArithmeticError::MalformedExpression.into()),
                };
                *number = result;
                marker.push(index);
            }
            _ => { /* NO-OP */ }
        }
    }

    Ok(numbers
        .iter_mut()
        .enumerate()
        .filter(|(idx, _)| !marker.contains(idx))
        .map(|(_, number)| *number)
        .sum())
}
