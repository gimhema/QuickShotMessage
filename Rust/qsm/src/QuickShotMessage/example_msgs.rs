
use super::qsm::*;
use std::time::{Instant};

pub struct Person
{
    id : i64,
    name : QString,
    age : QInteger,
    height : QFloat,
    grade : QArray<i64>
}

impl MessageBuilder for Person
{
    fn message_build(mut self) -> QMessage {
        let mut _data = Vec::new();
        let mut _size = 0;

        _data.push(self.name.get_buffer());
        _size += self.name.get_buffer().len();

        _data.push(self.age.get_buffer());
        _size += self.age.get_buffer().len();

        _data.push(self.height.get_buffer());
        _size += self.height.get_buffer().len();

        _data.push(self.grade.get_buffer());
        _size += self.grade.get_buffer().len();
        
        // size는 임시
        let mut ret = QMessage::new(self.id,_size, _data);

        return ret
    }
}

impl Person {
    pub fn new(
        _id : i64,
        _name : QString,
        _age : QInteger,
        _height : QFloat,
        _grade : QArray<i64> ) -> Self {
        return Person { id: _id, name: _name, age: _age, height: _height, grade: _grade }
    }
}

pub fn TEST_Seriialize()
{

    // Struct -> Message
    let start = Instant::now();

    let person = Person::new(1, QString::new("John".to_string()), QInteger::new(14), QFloat::new(172.3),
QArray::new(vec![10, 32, 47], QType::QInt));

    let mut person_message = person.message_build();

    println!("Id : {}", person_message.get_id().clone());
    println!("Size : {}", person_message.get_size().clone());
    println!("Message : {:?}", person_message.get_data());

    let end = Instant::now();

    let elapsed_time = end - start;
    println!("time out: {:?}", elapsed_time);


    let m_crate_start = Instant::now();

    println!("Created Message : {}", seirialize(person_message));

    let m_crate_end = Instant::now();

    let m_crate_elapsed_time = m_crate_end - m_crate_start;
    println!("message create time out: {:?}", m_crate_elapsed_time);
}

pub fn TEST_Deseriialize()
{
    // Message -> Struct
    let msg = "";
}