use std::collections::VecDeque;



pub enum QType {
    DEFAULT,
    QInt,
    QFloat,
    QString,
    QArray,
    QJson
}

pub struct QValue 
{
    qType : QType,
    data : String
}

impl QValue 
{
    pub fn new(_type : QType, _data : String) -> QValue {
        return QValue { qType: _type, data : _data }
    }

    pub fn convert() -> String {
        return "".to_string()
    }
}

pub trait QAction {
    fn Initialize(&mut self);
    fn convert(&mut self) -> String;
}

pub struct QInteger 
{
    val : QValue,
    data : i64
}

impl QAction for QInteger {
    fn Initialize(&mut self) {
        
    }

    fn convert(&mut self) -> String {
        return "".to_string()
    }
}

pub struct QFloat 
{
    val : QValue,
    data : f64
}

impl QAction for QFloat {
    fn Initialize(&mut self) {
        
    }

    fn convert(&mut self) -> String {
        return "".to_string()
    }
}

pub struct QString 
{
    val : QValue,
    data : String
}

impl QAction for QString {
    fn Initialize(&mut self) {
        
    }

    fn convert(&mut self) -> String {
        return "".to_string()
    }
}

pub struct QMessage 
{
    id : i64,
    data : Vec<String>
}

impl QMessage {
    pub fn new (_id : i64, _data : Vec<String>) -> QMessage {
        return QMessage { id: _id , data: _data }
    }
}

pub fn build_message_from_buffer(buffer : String) -> QMessage {
    let id = 0;
    let data = Vec::new();
    let ret = QMessage::new(id, data);

    return ret
}


