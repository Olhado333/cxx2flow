#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use cxx2flow::generate;
use cxx2flow::display::GraphDisplayBackend;
use cxx2flow::display::dot::Dot;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    pub content: Vec<u8>,
    pub file_name: String,
    pub function_name: Option<String>,
    pub is_curly_brace: bool,
}

fuzz_target!(|data: FuzzInput| {
    let _ = generate(
        &data.content,
        &data.file_name,
        data.function_name,
        GraphDisplayBackend::Dot(Dot::new(data.is_curly_brace)),
    );
});