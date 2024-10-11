use std::collections::VecDeque;
use std::any::type_name;
use std::any::TypeId;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum QType {
    DEFAULT,
    QInt,
    QFloat,
    QString,
    QArray,
    QJson,
}

pub fn QTypeToValue(_qType: QType) -> i32 {
    match _qType {
        QType::DEFAULT => 0,
        QType::QInt => 1,
        QType::QFloat => 2,
        QType::QString => 3,
        QType::QArray => 4,
        _ => -1,
    }
}

pub struct QTuple<T> {
    field: (i32, i64, f64, String, Vec<T>),
}

pub struct QUnpacked<T> {
    data: Vec<QTuple<T>>,
}

impl<T: std::fmt::Display> QTuple<T> {
    pub fn new_zero() -> Self {
        QTuple {
            field: (0, 0, 0.0, String::new(), Vec::new()),
        }
    }

    pub fn new(qinteger: i64, qfloat: f64, qstring: String, qvec: Vec<T>) -> Self {
        QTuple {
            field: (0, qinteger, qfloat, qstring, qvec),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QValue {
    qType: QType,
    meta_data: usize,
    buffer: String,
}

impl QValue {
    pub fn new_zero(_qType: QType) -> QValue {
        QValue {
            qType: _qType,
            meta_data: 0,
            buffer: String::new(),
        }
    }

    pub fn new(_type: QType, _meta_data: usize, _data: String) -> QValue {
        QValue {
            qType: _type,
            meta_data: _meta_data,
            buffer: _data,
        }
    }
}

pub trait QAction {
    fn initialize(&mut self);
    fn get_value(&self) -> &QValue;
    fn get_buffer(&self) -> &String;
}

pub struct QInteger {
    val: QValue,
    data: i64,
}

impl QInteger {
    pub fn new(data: i64) -> QInteger {
        let mut ret = QInteger {
            val: QValue::new_zero(QType::QInt),
            data,
        };
        ret.initialize();
        ret
    }
}

impl QAction for QInteger {
    fn initialize(&mut self) {
        self.val.meta_data = 0;
        let mut buffer = String::with_capacity(30);
        buffer.push_str("[");
        buffer.push_str(&QTypeToValue(self.val.qType.clone()).to_string());
        buffer.push(':');
        buffer.push_str(&self.val.meta_data.to_string());
        buffer.push(':');
        buffer.push_str(&self.data.to_string());
        buffer.push(']');
        self.val.buffer = buffer;
    }

    fn get_value(&self) -> &QValue {
        &self.val
    }

    fn get_buffer(&self) -> &String {
        &self.val.buffer
    }
}

pub struct QFloat {
    val: QValue,
    data: f64,
}

impl QFloat {
    pub fn new(data: f64) -> QFloat {
        let mut ret = QFloat {
            val: QValue::new_zero(QType::QFloat),
            data,
        };
        ret.initialize();
        ret
    }
}

impl QAction for QFloat {
    fn initialize(&mut self) {
        self.val.meta_data = 0;
        let mut buffer = String::with_capacity(40);
        buffer.push('[');
        buffer.push_str(&QTypeToValue(self.val.qType.clone()).to_string());
        buffer.push(':');
        buffer.push_str(&self.val.meta_data.to_string());
        buffer.push(':');
        buffer.push_str(&self.data.to_string());
        buffer.push(']');
        self.val.buffer = buffer;
    }

    fn get_value(&self) -> &QValue {
        &self.val
    }

    fn get_buffer(&self) -> &String {
        &self.val.buffer
    }
}

pub struct QString {
    val: QValue,
    data: String,
}

impl QString {
    pub fn new(_data: String) -> QString {
        let mut ret = QString {
            val: QValue::new_zero(QType::QString),
            data: _data,
        };
        ret.initialize();
        ret
    }
}

impl QAction for QString {
    fn initialize(&mut self) {
        self.val.meta_data = self.data.len();
        let mut buffer = String::with_capacity(self.val.meta_data + 20);
        buffer.push('[');
        buffer.push_str(&QTypeToValue(self.val.qType.clone()).to_string());
        buffer.push(':');
        buffer.push_str(&self.val.meta_data.to_string());
        buffer.push(':');
        buffer.push_str(&self.data);
        buffer.push(']');
        self.val.buffer = buffer;
    }

    fn get_value(&self) -> &QValue {
        &self.val
    }

    fn get_buffer(&self) -> &String {
        &self.val.buffer
    }
}

pub struct QArray<T> {
    val: QValue,
    elem_type: QType,
    data: Vec<T>,
}

impl<T: std::fmt::Display> QArray<T> {
    pub fn new(_data: Vec<T>, _elem_type: QType) -> QArray<T> {
        let mut ret = QArray {
            val: QValue::new_zero(QType::QArray),
            elem_type: _elem_type,
            data: _data,
        };
        ret.initialize();
        ret
    }
}

impl<T: std::fmt::Display> QAction for QArray<T> {
    fn initialize(&mut self) {
        self.val.meta_data = self.data.len();
        let elements = self
            .data
            .iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<_>>()
            .join(",");
        
        let mut buffer = String::with_capacity(50 + elements.len());
        buffer.push('[');
        buffer.push_str(&QTypeToValue(self.val.qType.clone()).to_string());
        buffer.push(':');
        buffer.push_str(&self.val.meta_data.to_string());
        buffer.push_str(":=");
        buffer.push_str(&QTypeToValue(self.elem_type.clone()).to_string());
        buffer.push('=');
        buffer.push_str(&elements);
        buffer.push(']');
        self.val.buffer = buffer;
    }

    fn get_value(&self) -> &QValue {
        &self.val
    }

    fn get_buffer(&self) -> &String {
        &self.val.buffer
    }
}

pub struct QMessage {
    id: i64,
    size: usize,
    data: Vec<String>,
}

impl QMessage {
    pub fn new(_id: i64, _size: usize, _data: Vec<String>) -> QMessage {
        QMessage {
            id: _id,
            size: _size,
            data: _data,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }
}

pub fn serialize(msg: QMessage) -> String {
    let total_capacity = msg
        .get_id()
        .to_string()
        .len()
        + msg.get_size().to_string().len()
        + msg.get_data().iter().map(|s| s.len()).sum::<usize>()
        + 10;
    
    let mut serialized = String::with_capacity(total_capacity);

    serialized.push_str(&msg.get_id().to_string());
    serialized.push(':');
    serialized.push_str(&msg.get_size().to_string());
    serialized.push(':');
    serialized.push('{');

    for elem in &msg.data {
        serialized.push_str(elem);
    }

    serialized.push('}');
    serialized
}
