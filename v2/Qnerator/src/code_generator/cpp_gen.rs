use crate::code_generator::read_parse_struct;

use super::CodeGenerator;
use std::fs;
use std::io::{self, Write};

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CPPGenerator {
    source: String,
}

impl CPPGenerator {
    pub fn new() -> Self {
        CPPGenerator {
            source: "".to_string(),
        }
    }

    pub fn format_cpp_code(&mut self, struct_name: &str, fields: &[((String, String))]) -> String {
        let mut cpp_code = String::new();
    
        // Structure definition
        cpp_code.push_str(&format!("#pragma pack(push, 1)\nstruct {} {{\n", struct_name));
    
        // Field definitions
        for (typ, name) in fields {
            let cpp_type = match &typ[..] {  // typ은 String이므로, 그대로 사용.
                "Integer" => "uint32_t".to_string(),
                "Long" => "uint64_t".to_string(),
                "Float" => "float".to_string(),
                "String" => "std::string".to_string(),
                typ if typ.starts_with("Array") => {
                    let element_type = match &typ[6..] {
                        "Integer" => "int32_t".to_string(),
                        "Long" => "int64_t".to_string(),
                        "Float" => "float".to_string(),
                        "String" => "std::string".to_string(),
                        _ => panic!("Unsupported array type: {}", &typ[6..]),
                    };
                    format!("std::vector<{}>", element_type)
                }
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!("    {} {};\n", cpp_type, name));
        }
    
        // Constructor params handling
        let constructor_params: Vec<String> = fields.iter().map(|(typ, name)| {
            let cpp_type = match &typ[..] {  // typ은 String이므로, 그대로 사용.
                "Integer" => "uint32_t".to_string(),
                "Long" => "uint64_t".to_string(),
                "Float" => "float".to_string(),
                "String" => "std::string".to_string(),
                typ if typ.starts_with("Array") => {
                    let element_type = match &typ[6..] {
                        "Integer" => "int32_t".to_string(),
                        "Long" => "int64_t".to_string(),
                        "Float" => "float".to_string(),
                        "String" => "std::string".to_string(),
                        _ => panic!("Unsupported array type: {}", &typ[6..]),
                    };
                    format!("std::vector<{}>", element_type)
                }
                _ => panic!("Unsupported type: {}", typ),
            };
            format!("{} {}", cpp_type, name)
        }).collect();
    
        cpp_code.push_str(&format!(
            "\n    {}({}) : ",
            struct_name,
            constructor_params.join(", ")
        ));
    
        let init_list: Vec<String> = fields.iter().map(|(_, name)| format!("{}({})", name, name)).collect();
        cpp_code.push_str(&init_list.join(", "));
        cpp_code.push_str(" {}\n\n");
    
        // Serialization and Deserialization code (no change needed)
    
        cpp_code
    }
    

    pub fn generate_cpp_code(&mut self, contents: &str) -> Option<String> {
        let mut lines = contents.lines();
    
        // Get struct name from the first line
        let struct_name_line = lines.next()?;
        let struct_name = struct_name_line.split_whitespace().nth(1)?;
    
        // Collect field information, converting to String
        let mut fields = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 2 {
                // Convert parts[0] and parts[1] to String before pushing
                fields.push((parts[0].to_string(), parts[1].to_string()));
            }
        }
    
        // Generate C++ code using the updated fields (Vec<(String, String)>)
        let cpp_code = self.format_cpp_code(struct_name, &fields);
        Some(cpp_code)
    }
    
}


impl CodeGenerator for CPPGenerator {
    fn generate(&self) {
        // Implementation of generate method
    }

    fn parse(&mut self) {
        // Implementation of parse method

        /*

        let cpp_generator = CPPGenerator::new();

        // C++ 코드 생성
        let cpp_code = cpp_generator.format_cpp_code("MyStruct", &fields);

        println!("{}", cpp_code);
        
         */
        let file_name = self.source.clone(); // source 값을 로컬 변수로 복사하여 빌림 해제
        let fields = read_parse_struct(self.source.clone());
        let cpp_code = self.format_cpp_code(&file_name, &fields);
    
        // 생성된 C++ 코드를 확인
        println!("{}", cpp_code);
    
    }


    fn set_source(&mut self, _source : String) {
        self.source = _source.clone();
    }
}
