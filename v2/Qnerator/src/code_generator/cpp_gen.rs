use super::CodeGenerator;
use std::fs;
use std::io::{self, Write};

pub struct CPPGenerator {
    source: String,
}

impl CPPGenerator {
    pub fn new() -> Self {
        CPPGenerator {
            source: "".to_string(),
        }
    }

    pub fn format_cpp_code(&mut self, struct_name: &str, fields: &[(&str, &str)]) -> String {
        let mut cpp_code = String::new();

        // Structure definition
        cpp_code.push_str(&format!("#pragma pack(push, 1)\nstruct {} {{\n", struct_name));

        // Field definitions
        for (typ, name) in fields {
            let cpp_type = match *typ {
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

        let constructor_params: Vec<String> = fields
        .iter()
        .map(|(typ, name)| {
            let cpp_type = match *typ {
                "Integer" => "uint32_t".to_string(),  // `&str`을 `String`으로 변환
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
    
                    // `format!`에서 반환되는 `String`을 그대로 사용
                    format!("std::vector<{}>", element_type)
                },
                _ => panic!("Unsupported type: {}", typ),
            };
    
            format!("{} {}", cpp_type, name)
        })
        .collect();
    
    
    


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
            match *typ {
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
            match *typ {
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

    pub fn generate_cpp_code(&mut self, contents: &str) -> Option<String> {
        let mut lines = contents.lines();

        // Get struct name from the first line
        let struct_name_line = lines.next()?;
        let struct_name = struct_name_line.split_whitespace().nth(1)?;

        // Collect field information
        let mut fields = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 2 {
                fields.push((parts[0], parts[1]));
            }
        }

        // Generate C++ code
        let cpp_code = self.format_cpp_code(struct_name, &fields);
        Some(cpp_code)
    }
}


impl CodeGenerator for CPPGenerator {
    fn generate(&self) {
        // Implementation of generate method
    }

    fn parse(&self) {
        // Implementation of parse method

        /*
        let fields = vec![
        ("Integer", "id"),
        ("Long", "timestamp"),
        ("Float", "val1"),
        ("String", "name"),
        ("ArrayInteger", "nums"),
        ("ArrayFloat", "vals"),
        ];

        let cpp_generator = CPPGenerator::new();

        // C++ 코드 생성
        let cpp_code = cpp_generator.format_cpp_code("MyStruct", &fields);

        println!("{}", cpp_code);
        
         */
        let fields = self.read_file();

        // let cpp_code = self.format_cpp_code(self.source.as_str(), fields);
    }

    fn read_file(&self) -> Vec<String> {
        // Implementation of read_file method
        // self.source 가지고 뭘 한다 . . .
        return Vec::new()
    }

    fn set_source(&mut self, _source : String) {
        self.source = _source.clone();
    }
}
