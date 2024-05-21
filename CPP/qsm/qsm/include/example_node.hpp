#include <iostream>
#include <thread>
#include <vector>
#include <string>
#include <cstring>
#include <sys/types.h>
#include <winsock2.h>
#include <ws2tcpip.h>

#pragma comment(lib, "Ws2_32.lib")

void handle_client(SOCKET client_socket) {
    char buffer[1024];
    int result;
    while ((result = recv(client_socket, buffer, 1024, 0)) > 0) {
        buffer[result] = '\0';
        std::string msg(buffer);

        if (msg == "exit\n") {
            std::cout << "Client requested disconnection.\n";
            break;
        }

        std::cout << "Received: " << msg;

        // Echo the message back to the client
        send(client_socket, buffer, result, 0);
    }

    closesocket(client_socket);
}

void run_server(const char* port) {
    WSADATA wsaData;
    int result;

    if ((result = WSAStartup(MAKEWORD(2, 2), &wsaData)) != 0) {
        std::cerr << "WSAStartup failed: " << result << "\n";
        return;
    }

    struct addrinfo* addr_result = nullptr;
    struct addrinfo hints;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;
    hints.ai_flags = AI_PASSIVE;

    if ((result = getaddrinfo(NULL, port, &hints, &addr_result)) != 0) {
        std::cerr << "getaddrinfo failed: " << result << "\n";
        WSACleanup();
        return;
    }

    SOCKET listen_socket = socket(addr_result->ai_family, addr_result->ai_socktype, addr_result->ai_protocol);
    if (listen_socket == INVALID_SOCKET) {
        std::cerr << "Error at socket(): " << WSAGetLastError() << "\n";
        freeaddrinfo(addr_result);
        WSACleanup();
        return;
    }

    if (bind(listen_socket, addr_result->ai_addr, (int)addr_result->ai_addrlen) == SOCKET_ERROR) {
        std::cerr << "bind failed: " << WSAGetLastError() << "\n";
        freeaddrinfo(addr_result);
        closesocket(listen_socket);
        WSACleanup();
        return;
    }

    freeaddrinfo(addr_result);

    if (listen(listen_socket, SOMAXCONN) == SOCKET_ERROR) {
        std::cerr << "Listen failed: " << WSAGetLastError() << "\n";
        closesocket(listen_socket);
        WSACleanup();
        return;
    }

    std::cout << "Server running on port " << port << "\n";

    while (true) {
        SOCKET client_socket = accept(listen_socket, NULL, NULL);
        if (client_socket == INVALID_SOCKET) {
            std::cerr << "accept failed: " << WSAGetLastError() << "\n";
            closesocket(listen_socket);
            WSACleanup();
            return;
        }

        std::thread(handle_client, client_socket).detach();
    }

    closesocket(listen_socket);
    WSACleanup();
}

void run_client(const char* server_address, const char* port) {
    WSADATA wsaData;
    int result;

    if ((result = WSAStartup(MAKEWORD(2, 2), &wsaData)) != 0) {
        std::cerr << "WSAStartup failed: " << result << "\n";
        return;
    }

    struct addrinfo* addr_result = nullptr;
    struct addrinfo hints;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    if ((result = getaddrinfo(server_address, port, &hints, &addr_result)) != 0) {
        std::cerr << "getaddrinfo failed: " << result << "\n";
        WSACleanup();
        return;
    }

    SOCKET connect_socket = socket(addr_result->ai_family, addr_result->ai_socktype, addr_result->ai_protocol);
    if (connect_socket == INVALID_SOCKET) {
        std::cerr << "Error at socket(): " << WSAGetLastError() << "\n";
        freeaddrinfo(addr_result);
        WSACleanup();
        return;
    }

    if (connect(connect_socket, addr_result->ai_addr, (int)addr_result->ai_addrlen) == SOCKET_ERROR) {
        std::cerr << "Unable to connect to server: " << WSAGetLastError() << "\n";
        closesocket(connect_socket);
        freeaddrinfo(addr_result);
        WSACleanup();
        return;
    }

    freeaddrinfo(addr_result);

    std::string input;
    char buffer[1024];

    while (true) {
        std::getline(std::cin, input);

        if (input == "exit") {
            std::cout << "Exiting...\n";
            break;
        }

        send(connect_socket, input.c_str(), input.length(), 0);

        int result = recv(connect_socket, buffer, 1024, 0);
        if (result > 0) {
            std::cout << "Received: " << std::string(buffer, result) << "\n";
        }
        else if (result == 0) {
            std::cout << "Connection closed\n";
            break;
        }
        else {
            std::cerr << "recv failed: " << WSAGetLastError() << "\n";
            break;
        }
    }

    closesocket(connect_socket);
    WSACleanup();
}

// int main() {
//     run_server("8080");
//     return 0;
// }

// int main() {
//     run_client("127.0.0.1", "8080");
//     return 0;
// }
