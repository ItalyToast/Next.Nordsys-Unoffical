mod langpack;
mod parse;
mod kotlin;
mod rust;

fn main() {
    rust::gen_rust_from_next_js();
    kotlin::gen_kotlin_from_next_js();
    langpack::run_codegen();
}