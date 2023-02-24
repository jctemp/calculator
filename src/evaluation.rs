use anyhow::Result;
use pest::iterators::Pair;
use std::str::FromStr;

use crate::{errors::ArithmeticError, parsing::*};

#[derive(Debug, PartialEq, Eq)]
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

    operators
        .iter_mut()
        .enumerate()
        .for_each(|(index, operator)| {
            if *operator == Operator::Sub {
                let number = numbers
                    .get_mut(index + 1)
                    .expect("Cannot be malformed as it was parsed.");

                *operator = Operator::Add;
                *number *= -1.0;
            }
        });

    let mut marker = Vec::new();
    operators
        .iter_mut()
        .enumerate()
        .for_each(|(index, operator)| {
            let pre_number = *numbers
                .get(index)
                .expect("Cannot be malformed as it was parsed.");
            let number = numbers
                .get_mut(index + 1)
                .expect("Cannot be malformed as it was parsed.");

            match operator {
                Operator::Mul => {
                    let result = pre_number * *number;
                    *number = result;
                    marker.push(index);
                }
                Operator::Div => {
                    let result = pre_number / *number;
                    *number = result;
                    marker.push(index);
                }
                _ => { /* NO-OP */ }
            }
        });

    Ok(numbers
        .iter_mut()
        .enumerate()
        .filter(|(idx, _)| !marker.contains(idx))
        .map(|(_, number)| *number)
        .sum())
}
