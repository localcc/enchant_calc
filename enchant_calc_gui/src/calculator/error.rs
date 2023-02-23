#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
enum CalculatorErrorCode {
    #[cfg(target_arch = "wasm32")]
    BinCode(bincode::Error),
    #[cfg(target_arch = "wasm32")]
    JsError(Option<String>),
}

impl std::fmt::Display for CalculatorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(target_arch = "wasm32")]
            CalculatorErrorCode::BinCode(ref err) => write!(f, "{}", err),
            #[cfg(target_arch = "wasm32")]
            CalculatorErrorCode::JsError(desc) => match desc {
                Some(ref desc) => write!(f, "{}", desc),
                None => write!(f, "Unknown js error"),
            },
            #[cfg(not(target_arch = "wasm32"))]
            _ => {
                let _ = f;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct CalculatorError {
    code: CalculatorErrorCode,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code.fmt(f)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<JsError> for CalculatorError {
    fn from(_: JsError) -> Self {
        CalculatorError {
            code: CalculatorErrorCode::JsError(None),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<JsValue> for CalculatorError {
    fn from(value: JsValue) -> Self {
        CalculatorError {
            code: CalculatorErrorCode::JsError(value.as_string()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<bincode::Error> for CalculatorError {
    fn from(value: bincode::Error) -> Self {
        CalculatorError {
            code: CalculatorErrorCode::BinCode(value),
        }
    }
}

impl std::error::Error for CalculatorError {}
