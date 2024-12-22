use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub enum GenType {
    NONE,
    CPP,
    RUST,
    GO,
    PYTHON
}

#[derive(Clone)]
pub struct CodeGenProperty {
    directory: String,
    file_path: String,
    gen_mode: GenType
}

impl CodeGenProperty {
    pub fn new() -> Self {
        return CodeGenProperty{directory : "".to_string(), file_path : "".to_string(), gen_mode : GenType::NONE}
    }
}

pub fn read_parse_struct(file_name : String) -> Vec<(String, String)> {
    let mut fields = Vec::new();
        
        // 파일 열기
        if let Ok(file) = File::open(&file_name) {
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

pub trait CodeGenerator {
    
    fn parse(&mut self) {

    }
    
    fn generate(&mut self) {

    }

    fn set_source(&mut self, _source : String) {

    }
}