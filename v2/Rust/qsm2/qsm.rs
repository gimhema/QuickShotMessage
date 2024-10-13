use std::io::{self, Cursor, Read, Write};

pub fn handle_message(buffer: &[u8]) {
    // BaseMessage의 ID 확인
    let base_message = BaseMessage::deserialize(buffer).unwrap();

    let base_message_id = base_message.id; // id를 복사

    match base_message_id {
        0 => {
            // id가 0이면 Invalid 출력
            println!("Invalid message id: 0");
        }
        1 => {
            // id가 1이면 PackedData의 값을 출력
            println!("Check Packed Message");
            let packed_data = PackedData::deserialize(buffer).unwrap();
            println!("PackedData received: {:?}", packed_data);
        }
        _ => {
            println!("Unknown message id: {}", base_message_id);
        }
    }
}


#[repr(packed)]
pub struct BaseMessage {
    id: u32,   // 메시지 타입을 나타냄
}

impl BaseMessage {
    // 새로운 BaseMessage 생성
    pub fn new(id: u32) -> Self {
        BaseMessage { id }
    }

    // 메시지의 바이너리 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(std::mem::size_of::<BaseMessage>());
        buffer.extend(&self.id.to_le_bytes()); // id 값을 리틀 엔디안으로 직렬화
        buffer
    }

    // 바이너리 데이터를 역직렬화하여 BaseMessage 생성
    // pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
    //     if buffer.len() < 4 {
    //         return Err(io::Error::new(io::ErrorKind::InvalidData, "Buffer too short"));
    //     }

    //     let mut id_bytes = [0u8; 4];
    //     id_bytes.copy_from_slice(&buffer[0..4]);

    //     let id = u32::from_le_bytes(id_bytes);
    //     Ok(BaseMessage { id })
    // }
    pub fn deserialize(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 4 {
            return Err("Buffer too short");
        }
        let id = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        Ok(BaseMessage { id })
    }
}


// 데이터 패킹을 위한 구조체 정의
#[repr(packed)]
#[derive(Debug, Clone)]
pub struct PackedData {
    id: u32,       // 4 bytes
    size: u32,     // 4 bytes
    value: u64,    // 8 bytes
}

impl PackedData {
    pub fn new(id: u32, value: u64) -> Self {
        let size = std::mem::size_of::<PackedData>() as u32; // 구조체의 크기
        PackedData { id, size, value }
    }

    // 바이너리 직렬화
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.size as usize);
        let mut cursor = Cursor::new(&mut buffer);
        cursor.write_all(&self.id.to_le_bytes()).unwrap();
        cursor.write_all(&self.size.to_le_bytes()).unwrap();
        cursor.write_all(&self.value.to_le_bytes()).unwrap();
        buffer
    }

    // 바이너리 역직렬화
    // pub fn deserialize(buffer: &[u8]) -> io::Result<Self> {
    //     let mut cursor = Cursor::new(buffer);
    //     let mut id_bytes = [0u8; 4];
    //     let mut size_bytes = [0u8; 4];
    //     let mut value_bytes = [0u8; 8];

    //     cursor.read_exact(&mut id_bytes)?;
    //     cursor.read_exact(&mut size_bytes)?;
    //     cursor.read_exact(&mut value_bytes)?;

    //     let id = u32::from_le_bytes(id_bytes);
    //     let size = u32::from_le_bytes(size_bytes);
    //     let value = u64::from_le_bytes(value_bytes);

    //     if size != buffer.len() as u32 {
    //         return Err(io::Error::new(io::ErrorKind::InvalidData, "Size mismatch"));
    //     }

    //     Ok(PackedData { id, size, value })
    // }

    pub fn deserialize(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() < std::mem::size_of::<PackedData>() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Buffer too short"));
        }
        
        let id = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let size = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let value = u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]);

        Ok(PackedData { id, size, value })
    }

}
