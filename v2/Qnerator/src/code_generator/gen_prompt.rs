

use super::gen_trait::*;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

use super::GenType;

use super::CPPGenerator;
use super::RustGenerator;

// qnerator -f <FILE_NAME> <GENERATE_DIRECTORY>
// qnerator -d <FILE_DIRECTORY> <GENERATE_DIRECTORY>

pub enum MODE {
    DEFAULT,
    FILE,
    DIRECTORY
}

pub struct GenPrompt {
    property: CodeGenProperty,
    mode : MODE
}

impl GenPrompt {
    pub fn new() -> Self {
        return GenPrompt{property : CodeGenProperty::new(), mode : MODE::DEFAULT}
    }

    pub fn get_property_clone(&mut self) -> CodeGenProperty {
        return self.property.clone()
    }

    fn get_first_word<'a>(&mut self, s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }

    pub fn set_mode_by_prefix(&mut self, argv: String) -> MODE {
        let first_word = self.get_first_word(&argv);

        match first_word.to_lowercase().as_str() {
            "-f" => MODE::FILE,
            "-d" => MODE::DIRECTORY,
            _ => MODE::DEFAULT,
        }
    }

    pub fn print_help(&mut self) {
        println!("===============================");
        println!("-d");
        println!("-f");
        println!("===============================");
    }

    pub fn find_file_from_directory(directroy : String) -> Vec<String> {
        let mut result = Vec::new();


        return result.clone()
    }

    pub fn check_lang_format(file_name : String) -> GenType {

        let path = Path::new(&file_name);

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("cpp") => {
                return GenType::CPP
            },
            Some("rs") => {
                return GenType::RUST
            },
            Some("py") => {
                return GenType::PYTHON
            },
            Some("go") => {
                return GenType::GO
            },
            _ => {
                println!("Unexpected Format");
            }
        }

        return GenType::NONE
    }

    pub fn parse_file(&mut self, file_name : String) -> String {

        // 1. check file format
        // 2. decide parse mode 
        let mut _property_clone = self.get_property_clone();

        let mut _source = file_name.clone();

        match Self::check_lang_format(file_name) {
            GenType::CPP => {
                println!("checked cpp");
                let mut generator = CPPGenerator::new();
                self.property.set_mode(GenType::CPP);

                generator.set_source(_source);
                generator.parse();
                generator.generate(_property_clone);
            }
            GenType::GO  => {
                println!("checked go");
            }
            GenType::PYTHON  => {
                println!("checked python");
            }
            GenType::RUST  => {
                println!("checked rust");
                let mut generator = RustGenerator::new();
                self.property.set_mode(GenType::RUST);

                generator.set_source(_source);
                generator.parse();
                generator.generate(_property_clone);
            }
            GenType::NONE  => {
                println!("unexpected format . . .");
            }
            _ => {println!("unexpected format . . .");}
        }


        return "".to_string()
    }


    pub fn parse(&mut self, argv: Vec<String>) {
        
        match self.mode {
            MODE::FILE => {

                let mut file_name = argv[2].clone();

                self.set_code_property(
                    String::from("/generated"),
                    file_name.clone(),
                      GenType::NONE);

                let mut _parse_result = self.parse_file(file_name.clone());
            },
            MODE::DIRECTORY => {

                let mut _file_list = Self::find_file_from_directory(argv[2].clone());

                for file in &_file_list {

                    self.set_code_property(
                        String::from("/generated"),
                        file.clone(),
                          GenType::NONE);

                    let mut _parse_result = self.parse_file(file.clone());
    
                }

            
            },
            _ => {println!("Unexpected action . . . .");}
        }

    }

    pub fn set_code_property(&mut self,
         generate_file_path : String,
          generate_file_name : String,
            gen_mode : GenType) {

        self.property.set_directory(generate_file_path);
        self.property.set_file_path(generate_file_name);
        self.property.set_mode(gen_mode);
    }

    pub fn run(&mut self, argv: Vec<String>) {

        self.mode = self.set_mode_by_prefix(argv[1].clone());

        match self.mode {
            MODE::DEFAULT => { self.print_help(); }
            MODE::FILE => {self.parse(argv.clone());}
            MODE::DIRECTORY => {self.parse(argv.clone());}
            _ => { self.print_help(); }
        }

    }

}
