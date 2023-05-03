#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use cxx2flow::generate;
use cxx2flow::display::GraphDisplayBackend;
use cxx2flow::display::d2::D2;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    pub content: Vec<u8>,
    pub file_name: String,
    pub function_name: Option<String>,
}

fuzz_target!(|data: FuzzInput| {
    let _ = generate(
        &data.content,
        &data.file_name,
        data.function_name,
        GraphDisplayBackend::D2(D2::new()),
    );
});