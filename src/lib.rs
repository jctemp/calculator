mod errors;
mod evaluation;
mod parsing;

use wasm_bindgen::prelude::*;

use crate::{evaluation::*, parsing::*};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    SUCCESS,
    FAILED,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ArithmeticResponse {
    result: f64,
    message: String,
    status: Status,
}

#[wasm_bindgen]
impl ArithmeticResponse {
    pub fn result(&self) -> f64 {
        self.result
    }

    pub fn message(&self) -> String {
        self.message.to_owned()
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ArithmeticRequest {
    expression: String,
}

#[wasm_bindgen]
impl ArithmeticRequest {
    #[wasm_bindgen(constructor)]
    pub fn new(expression: String) -> Self {
        ArithmeticRequest { expression }
    }
}

#[wasm_bindgen]
pub fn enable_logging() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
pub fn calculate(request: ArithmeticRequest) -> ArithmeticResponse {
    log::debug!("Request: {:#?}", request);

    let pair = match parse(&request.expression) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("{}", e);
            return ArithmeticResponse {
                result: f64::NAN,
                message: format!("{}", e),
                status: Status::FAILED,
            };
        }
    };

    log::debug!("Parsed: {:#?}, {:#?}", pair.as_rule(), pair.as_span());

    let (result, message, status) = match evaluate(&pair) {
        Ok(result) => (result, String::new(), Status::SUCCESS),
        Err(e) => {
            log::error!("{}", e);
            (f64::NAN, format!("{}", e), Status::FAILED)
        }
    };

    ArithmeticResponse {
        result: result,
        message: message,
        status: status,
    }
}

#[wasm_bindgen]
pub struct ExpressionBuilder {
    literals: Vec<char>,
    comma: bool,
}

#[wasm_bindgen]
impl ExpressionBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ExpressionBuilder {
            literals: Vec::new(),
            comma: false,
        }
    }

    pub fn extend(&mut self, value: char) {
        match value {
            '0'..='9' => self.digit(value),
            '+' | '-' | '*' | '/' => self.operator(value),
            '.' => self.comma(value),
            '=' => self.check(),
            _ => panic!("Cannot be a part of an expression."),
        }
    }

    fn digit(&mut self, value: char) {
        self.literals.push(value);
    }

    fn operator(&mut self, value: char) {
        match self.literals.last() {
            None | Some('.') => self.literals.push('0'),
            Some('+') | Some('-') | Some('*') | Some('/') => {
                self.literals.pop();
            }
            _ => { /* NOOP */ }
        }
        self.literals.push(value);
        self.comma = false;
    }

    fn comma(&mut self, value: char) {
        if self.comma {
            return;
        }

        match self.literals.last() {
            None | Some('+') | Some('-') | Some('*') | Some('/') => self.literals.push('0'),
            Some('.') => return,
            _ => { /* NOOP */ }
        };
        self.literals.push(value);
        self.comma = true;
    }

    fn check(&mut self) {
        match self.literals.last() {
            Some(c) if *c == '+' || *c == '-' || *c == '*' || *c == '/' => {
                self.literals.pop();
            }
            Some(c) if *c == '.' => self.literals.push('0'),
            _ => { /* NOOP */ }
        };
    }

    pub fn delete(&mut self) {
        match self.literals.pop() {
            Some(c) if c == '+' || c == '-' || c == '*' || c == '/' => self.literals.push(c),
            Some('.') => self.comma = false,
            _ => { /* NOOP */ }
        }
    }

    pub fn clear(&mut self) {
        self.literals.clear();
        self.comma = false;
    }

    pub fn collect(&mut self) -> String {
        let res = self.literals.iter().collect();
        self.literals.clear();
        self.comma = false;
        res
    }

    pub fn to_string(&self) -> String {
        self.literals.iter().collect()
    }

    pub fn debug(&self) -> String {
        format!("{:?}", self.literals)
    }
}
