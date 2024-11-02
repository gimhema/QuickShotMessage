

use super::gen_trait::*;

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

    pub fn parse(&mut self, _mode : MODE, argv: String) {
        
        match _mode {
            MODE::FILE => {

            },
            MODE::DIRECTORY => {

            },
            _ => {println!("Unexpected action . . . .");}
        }

    }

    pub fn read_argv(&mut self, argv: String) {

        self.mode = self.set_mode_by_prefix(argv);

        match self.mode {
            MODE::DEFAULT => { self.print_help(); }
            _ => { self.print_help(); }
        }

    }

}