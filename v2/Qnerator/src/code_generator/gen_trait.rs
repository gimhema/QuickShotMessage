use std::fs::File;
use std::io::{BufRead, BufReader};


use std::io::{self, Write};
use std::path::Path;

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

    pub fn get_directory(&mut self) -> String {
        return self.directory.clone()
    }

    pub fn get_file_path(&mut self) -> String {
        return self.file_path.clone()
    }

    pub fn get_mode(&mut self) -> GenType {
        return self.gen_mode.clone()
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
    
    fn generate(&mut self, gen_property :  CodeGenProperty) {

    }

    fn set_source(&mut self, _source : String) {

    }

    fn write(&mut self, _directory: String, _file_path: String, _source: String) {
        println!("Code Generate . . . .");
    
        // Create the full path by combining directory and file path
        let full_path = Path::new(&_directory).join(&_file_path);
    
        // Try to open the file for writing
        match File::create(&full_path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(_source.as_bytes()) {
                    eprintln!("Failed to write to file: {}", err);
                } else {
                    println!("Code generation completed. File written to: {}", full_path.display());
                }
            }
            Err(err) => {
                eprintln!("Failed to create file: {}", err);
            }
        }
    }
}