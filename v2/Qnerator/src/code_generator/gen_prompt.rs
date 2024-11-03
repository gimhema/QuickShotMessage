

use super::gen_trait::*;

use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

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

    pub fn parse_file(file_name : String) -> String {

        // 1. check file format

        // 2. decide parse mode 

        return "".to_string()
    }

    pub fn generate(content : String, generate_path : String) {

    }

    pub fn parse(&mut self, argv: Vec<String>) {
        
        match self.mode {
            MODE::FILE => {
                let mut _parse_result = Self::parse_file(argv[2].clone());

                Self::generate(_parse_result, argv[3].clone());
            },
            MODE::DIRECTORY => {

                let mut _file_list = Self::find_file_from_directory(argv[2].clone());

                for file in &_file_list {

                    let mut _parse_result = Self::parse_file(file.clone());

                    Self::generate(_parse_result, argv[3].clone());
    
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