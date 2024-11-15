

pub enum GenType {
    NONE,
    CPP,
    RUST,
    GO,
    PYTHON
}

pub struct CodeGenProperty {
    directory: String,
    file_path: String,
    gen_mode: GenType
}

impl CodeGenProperty {
    pub fn new() -> Self {
        return CodeGenProperty{directory : "".to_string(), file_path : "".to_string(), gen_mode : GenType::NONE}
    }
}

pub trait CodeGenerator {
    
    fn read_file(&mut self) -> Vec<(String, String)>; // Vec<(&'static str, &'static str)>;

    fn parse(&mut self) {

    }
    
    fn generate(&self) {

    }

    fn set_source(&mut self, _source : String) {

    }
}