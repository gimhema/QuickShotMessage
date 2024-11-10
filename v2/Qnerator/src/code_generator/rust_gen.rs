
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

    fn parse(&mut self) {
        
    }

    fn read_file(&mut self) -> Vec<(&'static str, &'static str)> {
        let result = vec![
            ("", "")
        ];
    
        result
    }
}