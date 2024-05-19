#include <iostream>
#include <chrono>
#include "QuickShotMessage/qsm.hpp"

int main() {
    // Example usage
    QInteger qInt(42);
    std::cout << qInt.get_buffer() << std::endl;

    QFloat qFloat(3.14);
    std::cout << qFloat.get_buffer() << std::endl;

    QString qString("Hello, World!");
    std::cout << qString.get_buffer() << std::endl;

    std::vector<int> vec = { 1, 2, 3, 4, 5 };
    QArray<int> qArray(vec, QType::QInt);
    std::cout << qArray.get_buffer() << std::endl;

    auto start = std::chrono::high_resolution_clock::now();

    std::vector<std::string> data = { qInt.get_buffer(), qFloat.get_buffer(), qString.get_buffer(), qArray.get_buffer() };
    QMessage qMessage(1, data.size(), data);
    std::cout << serialize(qMessage) << std::endl;

    auto end = std::chrono::high_resolution_clock::now();
    
    std::chrono::duration<double> duration = end - start;

    std::cout << "Execution time: " << duration.count() << " seconds" << std::endl;


    auto ds_start = std::chrono::high_resolution_clock::now();

    std::string input = "123:456:Hello, world!";
    auto [id, size, ds_data] = deserialize(input);

    auto ds_end = std::chrono::high_resolution_clock::now();

    std::chrono::duration<double> ds_duration = ds_start - ds_end;
    std::cout << "Execution time: " << ds_duration.count() << " seconds" << std::endl;

    std::cout << "ID: " << id << "\n";
    std::cout << "Size: " << size << "\n";
    std::cout << "Data: " << ds_data << "\n";


    return 0;
}