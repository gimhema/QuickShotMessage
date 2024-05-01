use std::collections::VecDeque;

// <ID>:<SIZE>:{[<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>]...}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
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
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
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

impl QString {
    pub fn new(_data : String) -> QString {
        let mut ret = QString {val : QValue::new_zero(QType::QString), data : _data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QString {
    fn Initialize(&mut self) {
        self.val.meta_data = self.data.len().clone(); // QString use meta data as length
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data + "]";         
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QArray<T>
{
    val : QValue,
    elem_type : QType,
    data : Vec<T>
}

impl<T> QArray<T> {
    pub fn new(_data: Vec<T>, _elem_type : QType) -> QArray<T> {
        let mut ret = QArray {
            val: QValue::new_zero(QType::QArray),
            elem_type: _elem_type, // 이 부분은 원하는 타입으로 설정해야 합니다.
            data: _data,
        };
        ret.Initialize();
        return ret;
    }
}

impl<T> QAction for QArray<T> {
    fn Initialize(&mut self) {
        
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
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

