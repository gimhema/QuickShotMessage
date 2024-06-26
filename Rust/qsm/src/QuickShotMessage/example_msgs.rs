
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
    fn unpack_message(self, buffer : String) -> Self {

        // 1:46:{[3:4:John][1:0:14][2:0:172.3][4:3:=1=10,32,47]}
        // message id => for RPC

        return Person::new_zero()   
    }

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
    pub fn new_zero() -> Self {
        return Person{
            id : 0, name : QString::new("".to_string()), age : QInteger::new(0),
            height : QFloat::new(0.0), grade : QArray::new(Vec::new(), QType::DEFAULT)
        }
    }

    pub fn new(
        _id : i64,
        _name : QString,
        _age : QInteger,
        _height : QFloat,
        _grade : QArray<i64> ) -> Self {
        return Person { id: _id, name: _name, age: _age, height: _height, grade: _grade }
    }
}

pub struct Actor {
    id : i64,
    loc_x : QFloat,
    loc_y : QFloat,
    loc_z : QFloat,
    rot_x : QFloat,
    rot_y : QFloat,
    rot_z : QFloat,
    rot_w : QFloat,
}

impl Actor {
    pub fn new_zero() -> Self {
        return Actor{
            id: 0, loc_x: QFloat::new(0.0), loc_y: QFloat::new(0.0), 
            loc_z: QFloat::new(0.0), rot_x: QFloat::new(0.0),
            rot_y : QFloat::new(0.0), rot_z : QFloat::new(0.0),
             rot_w : QFloat::new(0.0)
        };
    }
    pub fn new(
        _id : i64,
        _loc_x : QFloat,
        _loc_y : QFloat,
        _loc_z : QFloat,
        _rot_x : QFloat,
        _rot_y : QFloat,
        _rot_z : QFloat,
        _rot_w : QFloat, ) -> Self {
        return Actor { id: _id, loc_x: _loc_x, loc_y: _loc_y, 
            loc_z: _loc_z, rot_x: _rot_x, rot_y : _rot_y, rot_z : _rot_z, rot_w : _rot_w }
    }
}

impl MessageBuilder for Actor
{
    fn unpack_message(self, buffer : String) -> Self {
        return Actor::new_zero()
    }

    fn message_build(mut self) -> QMessage {
        let mut _data = Vec::new();
        let mut _size = 0;

        _data.push(self.loc_x.get_buffer());
        _size += self.loc_x.get_buffer().len();

        _data.push(self.loc_y.get_buffer());
        _size += self.loc_y.get_buffer().len();

        _data.push(self.loc_z.get_buffer());
        _size += self.loc_z.get_buffer().len();

        _data.push(self.rot_x.get_buffer());
        _size += self.rot_x.get_buffer().len();

        _data.push(self.rot_y.get_buffer());
        _size += self.rot_y.get_buffer().len();

        _data.push(self.rot_z.get_buffer());
        _size += self.rot_z.get_buffer().len();

        _data.push(self.rot_w.get_buffer());
        _size += self.rot_w.get_buffer().len();


        // size는 임시
        let mut ret = QMessage::new(self.id,_size, _data);

        return ret
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


    let start = Instant::now();

    let mut actor = Actor::new(7, QFloat::new(3.1), QFloat::new(2.7)
    , QFloat::new(11.6), QFloat::new(8.0), QFloat::new(9.1), QFloat::new(0.17)
    , QFloat::new(8.665));

    let mut actor_message = actor.message_build();

    let end = Instant::now();
    let elapsed_time = end - start;
    println!("actor create time out: {:?}", elapsed_time);

    println!("Actor Message Id : {}", actor_message.get_id().clone());
    println!("Actor Message Size : {}", actor_message.get_size().clone());
    println!("Actor Message Message : {:?}", actor_message.get_data());


    let start = Instant::now();
    
    println!("Created Message : {}", seirialize(actor_message));

    let end = Instant::now();
    let elapsed_time = end - start;
    println!("actor serialized time out: {:?}", elapsed_time);


}

pub fn TEST_Deseriialize()
{
    // Message -> Struct
    let start = Instant::now();

    let person = Person::new(1, QString::new("John".to_string()), QInteger::new(14), QFloat::new(172.3),
QArray::new(vec![10, 32, 47], QType::QInt));

    let mut person_message = person.message_build();

    let mut s_message = seirialize(person_message);

    println!("Previous Message : {}", s_message);
    let start = Instant::now();



    if let Some((id, size, data)) = deseirialize(s_message.as_str()) {

        let _data_vec = extract_data(data.as_str());
        let mut q_message = QMessage::new(id as i64, size as usize, _data_vec);

        println!("id = {}", q_message.get_id());
        println!("size = {}", q_message.get_size());
        println!("data = {:?}", q_message.get_data());

    } else {
        println!("Invalid input format");
    }

    let end = Instant::now();
    let elapsed_time = end - start;

    println!("deseirialize: {:?}", elapsed_time);

}