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
    PYTHON,
    CSHARP
}

#[derive(Clone)]
pub struct CodeGenProperty {
    target_file_directory: String,
    generate_directory: String,
    file_name: String,
    gen_mode: GenType
}

impl CodeGenProperty {
    pub fn new() -> Self {
        return CodeGenProperty{
            target_file_directory : "".to_string(),
            generate_directory : "".to_string(),
             file_name : "".to_string(),
              gen_mode : GenType::NONE}
    }

    pub fn get_generate_directory(&mut self) -> String {
        return self.generate_directory.clone()
    }

    pub fn get_file_name(&mut self) -> String {
        return self.file_name.clone()
    }

    pub fn get_mode(&mut self) -> GenType {
        return self.gen_mode.clone()
    }

    pub fn get_target_file_directory(&mut self) -> String {
        return self.target_file_directory.clone()
    }

    pub fn set_target_file_directory(&mut self, _directory: String) {
        self.target_file_directory = _directory;
    }

    pub fn set_generate_directory(&mut self, _direcotry : String) {
        self.generate_directory = _direcotry;
    }

    pub fn set_file_name(&mut self, _file_name : String) {
        self.file_name = _file_name;
    }

    pub fn set_mode(&mut self, _type : GenType) {
        self.gen_mode = _type;
    }
}


pub fn read_parse_struct(direcotry_name : String, file_name : String) -> Vec<(String, String)> {

    let mut _read_file_dir = format!("{}{}", direcotry_name, file_name);
    
    let mut fields = Vec::new();
        
        // 파일 열기
        if let Ok(file) = File::open(&_read_file_dir) {
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

    fn init_code_generator(&mut self, _source : String) {

    }

    fn change_file_format_by_gen_mode(&mut self, _file_name: String, _gen_mode: GenType) -> Option<String> {
        // Split the file name by '.' and handle the case where no '.' exists
        let mut parts = _file_name.rsplitn(2, '.'); // Split from the right, max 2 parts
        let file_name = parts.next()?.to_string();
        let _file_format = match _gen_mode {
            GenType::CPP => ".cpp",
            GenType::RUST => ".rs",
            GenType::PYTHON => ".py",
            GenType::CSHARP => ".cs",
            GenType::GO => ".go",
            _ => {
                println!("Unsupported type . . .");
                return None;
            }
        };
    
        Some(file_name + _file_format)
    }

    fn write(&mut self, _directory: String, _file_name: String, _source: String, _gen_mode : GenType) {
        println!("Code Generate . . . .");
        
        let mut _generate_file_name = self.change_file_format_by_gen_mode(_file_name, _gen_mode);

        // Create the full path by combining directory and file path
        let full_path = Path::new(&_directory).join(&_generate_file_name.unwrap());
    
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

    fn get_first_part(input: &str) -> &str {
        input.split('.').next().unwrap_or("")
    }
}

// #[derive(Clone)]
// pub struct GeneratorCommon{
//     read_file_path : String,
//     generate_file_path : String,
//     genrate_file_name : String,
//     generate_source : String
// }

// impl GeneratorCommon {
    
//     pub fn new() -> Self {
//         GeneratorCommon {
//             read_file_path : "".to_string(),
//             generate_file_path : "".to_string(),
//             genrate_file_name : "".to_string(),
//             generate_source : "".to_string()
//         }
//     }

//     pub fn set_read_file_path(&mut self, _path : String) {
//         self.read_file_path = _path;
//     }   

//     pub fn set_generate_file_path(&mut self, _path : String) {
//         self.generate_file_path = _path;
//     }

//     pub fn set_generate_source(&mut self, _src : String) {
//         self.generate_source = _src;
//     }

//     pub fn set_genrate_file_name(&mut self, _name : String) {
//         self.genrate_file_name = _name;
//     }

//     pub fn get_genrate_file_name(&mut self) -> String {
//         return self.read_file_path.clone()
//     }

//     pub fn get_read_file_path(&mut self) -> String {
//         return self.read_file_path.clone()
//     }   

//     pub fn get_generate_file_path(&mut self) -> String {
//         return self.generate_file_path.clone()
//     }

//     pub fn get_generate_source(&mut self) -> String {
//         return self.generate_source.clone()
//     }
// }
