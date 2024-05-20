#include "../../../include/qsm.hpp"


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

    std::vector<std::string> data = { qInt.get_buffer(), qFloat.get_buffer(), qString.get_buffer(), qArray.get_buffer() };
    QMessage qMessage(1, data.size(), data);
    std::cout << serialize(qMessage) << std::endl;

    return 0;
}
