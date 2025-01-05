use crate::code_generator::read_parse_struct;

use super::CodeGenerator;
use std::fs;
use std::io::{self, Write};

use std::fs::File;
use std::io::{BufRead, BufReader};
use super::gen_trait::*;

pub struct CPPGenerator{
//    source: String,
    gen_property : GeneratorCommon
}

impl CPPGenerator {
    pub fn new() -> Self {
        CPPGenerator {
            gen_property : GeneratorCommon::new()
        }
    }

    // pub fn set_source(&mut self,  _source: String) {
    //     self.source = _source;
    // }
// 
    // pub fn get_source(&mut self) -> String {
    //     return self.source.clone()
    // }

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
    
        // Serialize function
        cpp_code.push_str("    std::vector<uint8_t> serialize() const {\n");
        cpp_code.push_str("        std::vector<uint8_t> buffer;\n");

        for (typ, name) in fields {
            match typ.as_str() {
                "Integer" | "Long" | "Float" => {
                    cpp_code.push_str(&format!(
                        "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{}), reinterpret_cast<const uint8_t*>(&{} + 1));\n",
                        name, name
                    ));
                }
                "String" => {
                    cpp_code.push_str(&format!(
                        "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{}.size()), reinterpret_cast<const uint8_t*>(&{}.size() + 1));\n",
                        name, name
                    ));
                    cpp_code.push_str(&format!(
                        "        buffer.insert(buffer.end(), {}.begin(), {}.end());\n",
                        name, name
                    ));
                }
                typ if typ.starts_with("Array") => {
                    cpp_code.push_str(&format!(
                        "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(&{}.size()), reinterpret_cast<const uint8_t*>(&{}.size() + 1));\n",
                        name, name
                    ));
                    cpp_code.push_str(&format!(
                        "        buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>({}.data()), reinterpret_cast<const uint8_t*>({}.data() + {}.size()));\n",
                        name, name, name
                    ));
                }
                _ => panic!("Unsupported type: {}", typ),
            }
        }
        cpp_code.push_str("        return buffer;\n    }\n\n");

        // Deserialize function
        cpp_code.push_str(&format!(
            "    static {} deserialize(const std::vector<uint8_t>& buffer) {{\n",
            struct_name
        ));
        cpp_code.push_str("        size_t offset = 0;\n");

        let mut deserialized_fields = Vec::new();
        for (typ, name) in fields {
            match typ.as_str() {
                "Integer" => cpp_code.push_str(&format!(
                    "        uint32_t {} = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                    name
                )),
                "Long" => cpp_code.push_str(&format!(
                    "        uint64_t {} = *reinterpret_cast<const uint64_t*>(buffer.data() + offset);\n        offset += sizeof(uint64_t);\n",
                    name
                )),
                "Float" => cpp_code.push_str(&format!(
                    "        float {} = *reinterpret_cast<const float*>(buffer.data() + offset);\n        offset += sizeof(float);\n",
                    name
                )),
                "String" => {
                    cpp_code.push_str(&format!(
                        "        uint32_t {0}_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                        name
                    ));
                    cpp_code.push_str(&format!(
                        "        std::string {0}(buffer.begin() + offset, buffer.begin() + offset + {0}_length);\n        offset += {0}_length;\n",
                        name
                    ));
                }
                typ if typ.starts_with("Array") => {
                    cpp_code.push_str(&format!(
                        "        uint32_t {0}_length = *reinterpret_cast<const uint32_t*>(buffer.data() + offset);\n        offset += sizeof(uint32_t);\n",
                        name
                    ));
                    let element_type = match &typ[6..] {
                        "Integer" => "int32_t",
                        "Long" => "int64_t",
                        "Float" => "float",
                        "String" => "std::string",
                        _ => panic!("Unsupported array type: {}", &typ[6..]),
                    };
                    cpp_code.push_str(&format!(
                        "        std::vector<{}> {}({}_length);\n",
                        element_type, name, name
                    ));
                    cpp_code.push_str(&format!(
                        "        std::memcpy({}.data(), buffer.data() + offset, {}_length * sizeof({}));\n        offset += {}_length * sizeof({});\n",
                        name, name, element_type, name, element_type
                    ));
                }
                _ => panic!("Unsupported type: {}", typ),
            }
            deserialized_fields.push(name.to_string());
        }

        cpp_code.push_str(&format!(
            "        return {}({});\n    }}\n",
            struct_name,
            deserialized_fields.join(", ")
        ));

        // Friend operator<< overload
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
    
    
}


impl CodeGenerator for CPPGenerator {
    fn generate(&mut self, mut gen_property : CodeGenProperty) {

        let mut _source = self.get_source();
        let mut _file_path = gen_property.get_file_name();
        let mut _directory = gen_property.get_generate_directory();
        let mut _gen_mode = gen_property.get_mode();

        self.write(_directory, 
            _file_path, 
            _source,
            _gen_mode);
    }

    fn parse(&mut self) {
        // Implementation of parse method

        /*

        let cpp_generator = CPPGenerator::new();

        // C++ 코드 생성
        let cpp_code = cpp_generator.format_cpp_code("MyStruct", &fields);

        println!("{}", cpp_code);
        
         */
        // let dir_name = self.
        let directory_name = self.gen_property.get_generate_file_path();
        let file_name = self.source.clone(); // source 값을 로컬 변수로 복사하여 빌림 해제

        let fields = read_parse_struct();
        let cpp_code = self.format_cpp_code(&file_name, &fields);
    
        // 생성된 C++ 코드를 확인
        println!("{}", cpp_code);
    
    }


    fn set_source(&mut self, _source : String) {
        self.gen_property.set_generate_source(_source);
    }
}
