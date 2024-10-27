

use super::gen_trait::*;

pub struct GenPrompt {
    property: CodeGenProperty
}

impl GenPrompt {
    pub fn new() -> Self {
        return GenPrompt{property : CodeGenProperty::new()}
    }
}