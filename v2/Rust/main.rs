
mod qsm2;

use qsm2::qsm::*;

fn main() {
    // 새로운 PackedData 인스턴스 생성
    // let data = PackedData::new(1, 123456789);
    
    // // 바이너리 직렬화
    // let serialized = data.serialize();
    // println!("Serialized data: {:?}", serialized);

    // // 바이너리 역직렬화
    // match PackedData::deserialize(&serialized) {
    //     Ok(deserialized_data) => {
    //         println!("Deserialized data: {:?}", deserialized_data);
    //     }
    //     Err(e) => {
    //         println!("Error during deserialization: {}", e);
    //     }
    // }

    let invalid_message = BaseMessage::new(0).serialize();
    handle_message(&invalid_message);

    // id가 1인 PackedData 메시지 생성
    let packed_data = PackedData::new(1, 123456789);
    let packed_data_serialized = packed_data.serialize();
    handle_message(&packed_data_serialized);

}
