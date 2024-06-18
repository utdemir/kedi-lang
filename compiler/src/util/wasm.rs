use wasmprinter;

pub struct WasmBytes {
    pub bytes: Vec<u8>,
}

impl WasmBytes {
    pub fn to_wat(&self) -> Option<WatText> {
        let wat = wasmprinter::print_bytes(&self.bytes);
        match wat {
            Ok(text) => Some(WatText { text }),
            Err(_) => None,
        }
    }
}

pub struct WatText {
    pub text: String,
}
