#include <iostream>
#include <thread>
#include <vector>
#include <string>
#include <cstring>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <unistd.h>
#include <arpa/inet.h>

void handle_client(int client_socket) {
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

    close(client_socket);
}

void run_server(const char* port) {
    int result;
    struct addrinfo hints, *addr_result;

    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;

    if ((result = getaddrinfo(NULL, port, &hints, &addr_result)) != 0) {
        std::cerr << "getaddrinfo failed: " << gai_strerror(result) << "\n";
        return;
    }

    int listen_socket = socket(addr_result->ai_family, addr_result->ai_socktype, addr_result->ai_protocol);
    if (listen_socket == -1) {
        std::cerr << "Error at socket(): " << strerror(errno) << "\n";
        freeaddrinfo(addr_result);
        return;
    }

    if (bind(listen_socket, addr_result->ai_addr, addr_result->ai_addrlen) == -1) {
        std::cerr << "bind failed: " << strerror(errno) << "\n";
        freeaddrinfo(addr_result);
        close(listen_socket);
        return;
    }

    freeaddrinfo(addr_result);

    if (listen(listen_socket, SOMAXCONN) == -1) {
        std::cerr << "Listen failed: " << strerror(errno) << "\n";
        close(listen_socket);
        return;
    }

    std::cout << "Server running on port " << port << "\n";

    while (true) {
        int client_socket = accept(listen_socket, NULL, NULL);
        if (client_socket == -1) {
            std::cerr << "accept failed: " << strerror(errno) << "\n";
            close(listen_socket);
            return;
        }

        std::thread(handle_client, client_socket).detach();
    }

    close(listen_socket);
}

void run_client(const char* server_address, const char* port) {
    int result;
    struct addrinfo hints, *addr_result;

    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;

    if ((result = getaddrinfo(server_address, port, &hints, &addr_result)) != 0) {
        std::cerr << "getaddrinfo failed: " << gai_strerror(result) << "\n";
        return;
    }

    int connect_socket = socket(addr_result->ai_family, addr_result->ai_socktype, addr_result->ai_protocol);
    if (connect_socket == -1) {
        std::cerr << "Error at socket(): " << strerror(errno) << "\n";
        freeaddrinfo(addr_result);
        return;
    }

    if (connect(connect_socket, addr_result->ai_addr, addr_result->ai_addrlen) == -1) {
        std::cerr << "Unable to connect to server: " << strerror(errno) << "\n";
        close(connect_socket);
        freeaddrinfo(addr_result);
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
            std::cerr << "recv failed: " << strerror(errno) << "\n";
            break;
        }
    }

    close(connect_socket);
}

// int main() {
//     run_server("8080");
//     return 0;
// }

// int main() {
//     run_client("127.0.0.1", "8080");
//     return 0;
// }
