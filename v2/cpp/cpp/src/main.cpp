#include"QuickShotMessage/qsm.hpp"
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>

int main() {
    // // id가 0인 메시지 생성
    // BaseMessage invalid_message(0);
    // std::vector<uint8_t> invalid_message_serialized = invalid_message.serialize();
    // handle_message(invalid_message_serialized);

    // // id가 1인 PackedData 메시지 생성
    // PackedData packed_data(1, 123456789);
    // std::vector<uint8_t> packed_data_serialized = packed_data.serialize();
    // handle_message(packed_data_serialized);

    // return 0;

    int sock = 0;
    struct sockaddr_in serv_addr;
    
    // 소켓 생성
    if ((sock = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        std::cerr << "Socket creation error" << std::endl;
        return -1;
    }

    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(8080);

    // 서버 주소 설정 (127.0.0.1)
    if (inet_pton(AF_INET, "127.0.0.1", &serv_addr.sin_addr) <= 0) {
        std::cerr << "Invalid address/ Address not supported" << std::endl;
        return -1;
    }

    // 서버에 연결
    if (connect(sock, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        std::cerr << "Connection Failed" << std::endl;
        return -1;
    }

// PackedData 메시지 생성 (id = 1, value = 123456789)
    PackedData packed_data(1, 123456789);
    std::vector<uint8_t> serialized_data = packed_data.serialize();

    // 메시지 전송
    send(sock, serialized_data.data(), serialized_data.size(), 0);
    std::cout << "PackedData message sent" << std::endl;


    close(sock);
    return 0;

}
