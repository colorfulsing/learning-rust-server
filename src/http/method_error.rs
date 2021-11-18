use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct MethodError {
    value: Option<String>
}

impl MethodError {
    pub fn new(value: Option<String>) -> Self {
        Self{
            value
        }
    }

    pub fn message(&self) -> String {
        match &self.value {
            Some(v) => format!("unknown method \"{}\"", &v[..]),
            _ => "unknown method".to_string()
        }
    }
}

impl Display for MethodError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for MethodError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.message())
    }
}