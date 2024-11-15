
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
}


impl CodeGenerator for RustGenerator {
    fn generate(&self) {
        
    }

    fn parse(&mut self) {
        
    }

}