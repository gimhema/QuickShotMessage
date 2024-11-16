
use super::CodeGenerator;
use super::gen_trait::*;

use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct RustGenerator {
    source : String
}

impl RustGenerator {
    pub fn new() -> Self {
        return RustGenerator{source : "".to_string()}
    }

    pub fn format_rust_code(&mut self, struct_name: &str, fields: &[((String, String))]) -> String {
        let mut rust_code = String::new();

        return rust_code
    }

    pub fn generate_rust_code(&mut self, contents: &str) -> Option<String> {
        let rust_code = String::new();

        return Some(rust_code)
    }
}


impl CodeGenerator for RustGenerator {
    fn generate(&self) {
        
    }

    fn parse(&mut self) {
        
    }

}