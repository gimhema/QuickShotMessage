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
    buffer : String
}

impl QValue 
{
    pub fn new(_type : QType, _data : String) -> QValue {
        return QValue { qType: _type, buffer : _data }
    }

    pub fn convert() -> String {
        return "".to_string()
    }
}

pub trait QAction {
    fn Initialize(&mut self);
    fn convert(&mut self) -> String;
    fn get_buffer(&mut self) -> String;
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

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
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

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
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

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
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

pub fn deseirialize(buffer : String) -> QMessage {
    let id = 0;
        
        /*
        for _data
            get TYPE from _data
            get META_DATA from _data
            get VALUE from _data
            Assemble data 
            push data to datas
         */
    let datas = Vec::new();


    let ret = QMessage::new(id, datas);

    return ret
}

pub fn seirialize(msg : QMessage) -> String {
    // id + data . . .
    let serialized = String::new();

    return serialized
}

