

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

pub trait CodeGenerator {
    fn read_file(&self)  {
        
    }

    fn parse(&self) {

    }
    
    fn generate(&self) {

    }
}