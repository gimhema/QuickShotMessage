
use super::CodeGenerator;
use super::gen_trait::*;


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

    fn parse(&self) {
        
    }

    fn read_file(&self) -> Vec<String> {
        return Vec::new()
    }
}