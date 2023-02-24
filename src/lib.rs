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
pub fn calculate(request: ArithmeticRequest) -> ArithmeticResponse {
    wasm_logger::init(wasm_logger::Config::default());

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
