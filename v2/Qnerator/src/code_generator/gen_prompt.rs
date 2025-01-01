

use super::gen_trait::*;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

use super::GenType;

use super::CPPGenerator;
use super::RustGenerator;

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

    pub fn parse_file(&mut self, file_name : String) -> String {

        // 1. check file format
        // 2. decide parse mode 
        let mut _property_clone = self.get_property_clone();
        let mut _generated_mode = self.get_property_clone().get_mode();
        let mut _source = file_name.clone();

        match _generated_mode {
            GenType::CPP => {
                println!("checked cpp");
                let mut generator = CPPGenerator::new();


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

    pub fn set_generate_mode_by_console_argv(&mut self, command : String) {

        match command.as_str() {
            "cpp" => { self.property.set_mode(GenType::CPP); },
            "rust" => { self.property.set_mode(GenType::RUST); },
            "go" => { self.property.set_mode(GenType::GO); },
            "python" => { self.property.set_mode(GenType::PYTHON); },
            "csharp" => { self.property.set_mode(GenType::CSHARP); },
            _ => {println!("Unsupported type . . .");}
        }

    }

    pub fn parse(&mut self, argv: Vec<String>) {
        
    // qnerator -f <FILE_NAME> <GENERATE_DIRECTORY>
    // qnerator -d <FILE_DIRECTORY> <GENERATE_DIRECTORY>

    // use case :  qnerator -f ExampleMEssage.qsmb cpp Example
    // use case :  qnerator -d ExampleMEssages cpp Example

    self.set_generate_mode_by_console_argv(argv[3].clone());

        match self.mode {
            MODE::FILE => {
                // argv[1] : prompt mode
                // argv[2] : file name
                // argv[3] : generate language
                // argv[4] : generate directory
                let mut file_name = argv[2].clone();


                let mut _parse_result = self.parse_file(file_name.clone());
            },
            MODE::DIRECTORY => {
                // argv[1] : prompt mode
                // argv[2] : target file directory name
                // argv[3] : generate language
                // argv[4] : generate directory
                let mut _file_list = Self::find_file_from_directory(argv[2].clone());

                for file in &_file_list {


                    let mut _parse_result = self.parse_file(file.clone());
    
                }

            
            },
            _ => {println!("Unexpected action . . . .");}
        }

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
