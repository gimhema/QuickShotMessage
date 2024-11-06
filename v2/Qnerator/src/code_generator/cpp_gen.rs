
use super::CodeGenerator;
use super::gen_trait::*;
use std::fs;
use std::io::{self, Write};

pub struct CPPGenerator {
    source : String
}

impl CPPGenerator {
    pub fn new() -> Self {
        return CPPGenerator{source : "".to_string()}
    }

    pub fn format_cpp_code(&mut self, struct_name: &str, fields: &[(<&str, &str)]) -> String {
        let mut cpp_code = String::new();
    
        // 구조체 정의 시작
        cpp_code.push_str(&format!("#pragma pack(push, 1)\nstruct {} {{\n", struct_name));
    
        // 필드 정의
        for (typ, name) in fields {
            let cpp_type = match *typ {
                "Integer" => "uint32_t",
                "Long" => "uint64_t",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!("    {} {};\n", cpp_type, name));
        }
    
        // 생성자 정의
        let constructor_params: Vec<String> = fields.iter().map(|(typ, name)| {
            let cpp_type = match *typ {
                "Integer" => "uint32_t",
                "Long" => "uint64_t",
                _ => panic!("Unsupported type: {}", typ),
            };
            format!("{} {}", cpp_type, name)
        }).collect();
    
        cpp_code.push_str(&format!("\n    {}({}) : ", struct_name, constructor_params.join(", ")));
    
        let init_list: Vec<String> = fields.iter().map(|(_, name)| {
            format!("{}({})", name, name)
        }).collect();
    
        cpp_code.push_str(&init_list.join(", "));
        cpp_code.push_str(" {}\n\n");
    
        // serialize 함수 정의
        cpp_code.push_str("    std::vector<uint8_t> serialize() {\n");
        cpp_code.push_str("        std::vector<uint8_t> buffer(sizeof(*this));\n");
    
        let mut offset = 0;
        for (typ, name) in fields {
            let size = match *typ {
                "Integer" => "sizeof(uint32_t)",
                "Long" => "sizeof(uint64_t)",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!(
                "        std::memcpy(buffer.data() + {}, &{}, {});\n",
                offset, name, size
            ));
            offset += size.parse::<usize>().unwrap();
        }
        cpp_code.push_str("        return buffer;\n    }\n\n");
    
        // deserialize 함수 정의
        cpp_code.push_str(&format!(
            "    static {} deserialize(const std::vector<uint8_t>& buffer) {{\n",
            struct_name
        ));
        cpp_code.push_str("        if (buffer.size() < sizeof(*this)) {\n");
        cpp_code.push_str("            throw std::runtime_error(\"Buffer too short\");\n");
        cpp_code.push_str("        }\n");
    
        for (typ, name) in fields {
            let cpp_type = match *typ {
                "Integer" => "uint32_t",
                "Long" => "uint64_t",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!("        {} {};\n", cpp_type, name));
        }
    
        offset = 0;
        for (typ, name) in fields {
            let size = match *typ {
                "Integer" => "sizeof(uint32_t)",
                "Long" => "sizeof(uint64_t)",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!(
                "        std::memcpy(&{}, buffer.data() + {}, {});\n",
                name, offset, size
            ));
            offset += size.parse::<usize>().unwrap();
        }
    
        let constructor_args: Vec<String> = fields.iter().map(|(_, name)| name.to_string()).collect();
        cpp_code.push_str(&format!(
            "        return {}({});\n    }}\n",
            struct_name, constructor_args.join(", ")
        ));
    
        // friend 출력 연산자 정의
        cpp_code.push_str(&format!(
            "    friend std::ostream& operator<<(std::ostream& os, const {}& data) {{\n",
            struct_name
        ));
        cpp_code.push_str(&format!("        os << \"{} {{ ", struct_name));
        for (i, (_, name)) in fields.iter().enumerate() {
            cpp_code.push_str(&format!("{}: \" << data.{}", name, name));
            if i < fields.len() - 1 {
                cpp_code.push_str(" << \", ");
            }
        }
        cpp_code.push_str(" << \" }\";\n        return os;\n    }\n};\n");
        cpp_code.push_str("#pragma pack(pop)\n");
    
        cpp_code
    }

    fn generate_cpp_code(&mut self, contents: &str) -> Option<String> {
        let mut lines = contents.lines();
        
        // 첫 번째 줄에서 구조체 이름을 가져옵니다.
        let struct_name_line = lines.next()?;
        let struct_name = struct_name_line.split_whitespace().nth(1)?;
    
        // 필드 정보를 수집합니다.
        let mut fields = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 2 {
                fields.push((parts[0], parts[1]));
            }
        }
    
        // C++ 코드를 생성합니다.
        let cpp_code = self.format_cpp_code(struct_name, &fields);
        Some(cpp_code)
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

/*
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // msg.txt 파일을 읽습니다.
    let contents = fs::read_to_string("msg.txt")?;
    
    // 파일 내용을 바탕으로 구조체를 파싱합니다.
    if let Some(generated_code) = generate_cpp_code(&contents) {
        // 결과를 파일로 저장합니다.
        let mut output = fs::File::create("generated.cpp")?;
        output.write_all(generated_code.as_bytes())?;
    } else {
        println!("Failed to generate C++ code.");
    }

    Ok(())
}






*/
