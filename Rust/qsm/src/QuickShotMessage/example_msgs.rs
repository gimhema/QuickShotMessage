
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
