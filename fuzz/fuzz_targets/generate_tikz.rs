#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use cxx2flow::generate;
use cxx2flow::display::GraphDisplayBackend;
use cxx2flow::display::tikz::Tikz;

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
        GraphDisplayBackend::Tikz(Tikz::new()),
    );
});