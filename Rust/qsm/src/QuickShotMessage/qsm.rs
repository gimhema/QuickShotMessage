use std::collections::VecDeque;

// <ID>:<SIZE>:{[<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>]...}

pub enum QType {
    DEFAULT,
    QInt,
    QFloat,
    QString,
    QArray,
    QJson
}

pub fn QTypeToValue(_qType : QType) -> i32{
    match _qType {
        QType::DEFAULT => return 0,
        QType::QInt => return 1,
        QType::QFloat => return 2,
        QType::QString => return 3,
        QType::QArray => return 4,
        _ => return -1
    };

    return -1
}


pub struct QValue 
{
    qType : QType,
    meta_data : usize,
    buffer : String
}

impl QValue 
{
    pub fn new_zero(_qType : QType) -> QValue {
        return QValue { qType: _qType, meta_data: 0, buffer: String::new() }
    }

    pub fn new(_type : QType, _meta_data : usize, _data : String) -> QValue {
        return QValue { qType: _type, meta_data: _meta_data, buffer : _data }
    }

    pub fn convert() -> String {
        return "".to_string()
    }
}



pub trait QAction {
    fn Initialize(&mut self);
    fn get_value(&mut self) -> QValue;
    fn get_buffer(&mut self) -> String;
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QInteger 
{
    val : QValue,
    data : i64
}

impl QInteger {
    pub fn new(data : i64) -> QInteger {
        let mut ret = QInteger {val : QValue::new_zero(QType::QInt), data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QInteger {
    fn Initialize(&mut self) {
        self.val.meta_data = 0; // QInteger not use meta data
        self.val.buffer = "[" + QTypeToValue(self.val.qType).to_string() + ":" + self.val.meta_data + ":" + self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QFloat 
{
    val : QValue,
    data : f64
}

impl QFloat {
    pub fn new(data : f64) -> QFloat {
        let mut ret = QFloat {val : QValue::new_zero(QType::QFloat), data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QFloat {
    fn Initialize(&mut self) {
        self.val.meta_data = 0; // QFloat not use meta data
        self.val.buffer = "[" + QTypeToValue(self.val.qType).to_string() + ":" + self.val.meta_data + ":" + self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QString 
{
    val : QValue,
    data : String
}

impl QAction for QString {
    fn Initialize(&mut self) {
        
    }

    fn get_value(&mut self) -> QValue {
        return self.val
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QArray
{
    val : QValue,
    elem_type : QType,
    data : String
}

impl QAction for QArray {
    fn Initialize(&mut self) {
        
    }

    fn get_value(&mut self) -> QValue {
        return self.val
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}
pub struct QMessage 
{
    id : i64,
    size : usize,
    data : Vec<String>
}

impl QMessage {
    pub fn new (_id : i64, _size : usize, _data : Vec<String>) -> QMessage {
        return QMessage { id: _id, size : _size , data: _data }
    }
}

pub trait MessageBuilder
{
    fn message_build(self) -> QMessage;
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
    let _size = datas.len();


    let ret = QMessage::new(id, _size, datas);

    return ret
}

pub fn seirialize(msg : QMessage) -> String {
    // id + data . . .
    let serialized = String::new();

    return serialized
}

