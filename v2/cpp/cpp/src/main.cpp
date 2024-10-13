#include"QuickShotMessage/qsm.hpp"

int main() {
    // id가 0인 메시지 생성
    BaseMessage invalid_message(0);
    std::vector<uint8_t> invalid_message_serialized = invalid_message.serialize();
    handle_message(invalid_message_serialized);

    // id가 1인 PackedData 메시지 생성
    PackedData packed_data(1, 123456789);
    std::vector<uint8_t> packed_data_serialized = packed_data.serialize();
    handle_message(packed_data_serialized);

    return 0;
}