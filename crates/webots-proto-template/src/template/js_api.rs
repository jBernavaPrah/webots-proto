use boa_engine::JsValue;
use boa_gc::{Finalize, Trace};

// Placeholder for field mapping logic
#[derive(Debug, Trace, Finalize)]
pub struct ProtoField {
    pub value: JsValue,
}

impl ProtoField {
    pub fn new(value: JsValue) -> Self {
        Self { value }
    }
}
