
use super::CodeGenerator;
use super::gen_trait::*;


pub struct CPPGenerator {
    source : String
}

impl CPPGenerator {
    pub fn new() -> Self {
        return CPPGenerator{source : "".to_string()}
    }
}


impl CodeGenerator for CPPGenerator {
    fn generate(&self) {
        
    }

    fn parse(&self) {
        
    }

    fn read_file(&self) {
        
    }
}