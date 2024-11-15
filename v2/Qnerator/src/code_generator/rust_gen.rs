
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

    fn read_file(&mut self) -> Vec<(String, String)> {
        let mut fields = Vec::new();
        
        // 파일 열기
        if let Ok(file) = File::open(&self.source) {
            let reader = BufReader::new(file);
    
            for line in reader.lines().flatten() {
                let trimmed_line = line.trim();
                
                // 필드 타입과 이름을 가져오기
                let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                if parts.len() == 2 {
                    let field_type = match parts[0] {
                        "Integer" => "Integer".to_string(),
                        "Long" => "Long".to_string(),
                        "Float" => "Float".to_string(),
                        "String" => "String".to_string(),
                        "ArrayInteger" => "ArrayInteger".to_string(),
                        "ArrayFloat" => "ArrayFloat".to_string(),
                        _ => continue,
                    };
                    
                    let field_name = parts[1].to_string();
    
                    fields.push((field_type, field_name));
                }
            }
        }
    
        fields
    }
}