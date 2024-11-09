use super::CodeGenerator;
use super::gen_trait::*;
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
                "Integer" => "uint32_t",
                "Long" => "uint64_t",
                "Float" => "float",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!("    {} {};\n", cpp_type, name));
        }

        // Constructor
        let constructor_params: Vec<String> = fields
            .iter()
            .map(|(typ, name)| {
                let cpp_type = match *typ {
                    "Integer" => "uint32_t",
                    "Long" => "uint64_t",
                    "Float" => "float",
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
        cpp_code.push_str("    std::vector<uint8_t> serialize() {\n");
        cpp_code.push_str("        std::vector<uint8_t> buffer(sizeof(*this));\n");

        let mut offset = 0;
        for (typ, name) in fields {
            let size = match *typ {
                "Integer" => 4,
                "Long" => 8,
                "Float" => 4,
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!(
                "        std::memcpy(buffer.data() + {}, &{}, {});\n",
                offset, name, size
            ));
            offset += size;
        }
        cpp_code.push_str("        return buffer;\n    }\n\n");

        // Deserialize function
        cpp_code.push_str(&format!(
            "    static {} deserialize(const std::vector<uint8_t>& buffer) {{\n",
            struct_name
        ));
        cpp_code.push_str(&format!("        if (buffer.size() < sizeof({})) {{\n", struct_name));

        cpp_code.push_str("            throw std::runtime_error(\"Buffer too short\");\n");
        cpp_code.push_str("        }\n");

        for (typ, name) in fields {
            let cpp_type = match *typ {
                "Integer" => "uint32_t",
                "Long" => "uint64_t",
                "Float" => "float",
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!("        {} {};\n", cpp_type, name));
        }

        offset = 0;
        for (typ, name) in fields {
            let size = match *typ {
                "Integer" => 4,
                "Long" => 8,
                "Float" => 4,
                _ => panic!("Unsupported type: {}", typ),
            };
            cpp_code.push_str(&format!(
                "        std::memcpy(&{}, buffer.data() + {}, {});\n",
                name, offset, size
            ));
            offset += size;
        }

        let constructor_args: Vec<String> = fields.iter().map(|(_, name)| name.to_string()).collect();
        cpp_code.push_str(&format!(
            "        return {}({});\n    }}\n",
            struct_name,
            constructor_args.join(", ")
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
    }

    fn read_file(&self) {
        // Implementation of read_file method
    }
}
