use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use wasm_bindgen::prelude::*;

use crate::errors::ArithmeticError;

#[wasm_bindgen]
#[derive(Parser)]
#[grammar = "arithmetic.pest"]
pub struct ArithmeticParser;

pub fn parse(input: &str) -> Result<Pair<Rule>> {
    let parsed = ArithmeticParser::parse(Rule::expression, input)?.next();

    let parsed = match parsed {
        Some(value) => value,
        None => return Err(ArithmeticError::Empty.into()),
    };

    if parsed.as_str() != input {
        return Err(
            ArithmeticError::IncompleteParsing(input.to_string(), parsed.as_span().end()).into(),
        );
    }

    Ok(parsed)
}
