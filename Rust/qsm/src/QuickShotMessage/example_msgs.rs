
use super::qsm::*;


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

        _data.push(self.name.get_buffer());
        _data.push(self.age.get_buffer());
        _data.push(self.height.get_buffer());
        _data.push(self.grade.get_buffer());

        // size는 임시
        let mut ret = QMessage::new(self.id,0, _data);

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

pub fn PersonTest()
{
    let person = Person::new(1, QString::new("John".to_string()), QInteger::new(14), QFloat::new(172.3),
QArray::new(vec![10, 32, 47], QType::QInt));

    let person_message = person.message_build();

    // println!("Message : {}", person_message.)
}
