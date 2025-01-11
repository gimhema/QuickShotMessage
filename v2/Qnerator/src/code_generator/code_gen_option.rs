use std::sync::{RwLock, Arc};
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Arc<RwLock<CodeGenOptionManager>> = Arc::new(RwLock::new(CodeGenOptionManager::new()));
}

pub struct CodeGenOptionManager {

    gen_option : CodeGenProperty

}

impl CodeGenOptionManager {
    pub fn new() -> Self {
        return CodeGenOptionManager::new()   
    }

    pub fn get_instance() -> &'static Arc<RwLock<CodeGenOptionManager>> {
        &INSTANCE
    }
}


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