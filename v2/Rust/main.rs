mod qsm2;

use qsm2::qsm::*;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()>  {
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

    // let invalid_message = BaseMessage::new(0).serialize();
    // handle_message(&invalid_message);

    // // id가 1인 PackedData 메시지 생성
    // let packed_data = PackedData::new(1, 123456789);
    // let packed_data_serialized = packed_data.serialize();
    // handle_message(&packed_data_serialized);

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = vec![0; std::mem::size_of::<PackedData>()]; // PackedData 크기 만큼의 버퍼 생성
        
        // 데이터가 충분히 수신될 때까지 읽기
        let mut total_read = 0;
        while total_read < buffer.len() {
            let bytes_read = stream.read(&mut buffer[total_read..])?;
            if bytes_read == 0 {
                // 연결이 종료된 경우
                break;
            }
            total_read += bytes_read;
        }

        handle_message(&buffer);
    }

    Ok(())

}
