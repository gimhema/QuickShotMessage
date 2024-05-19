#include <iostream>
#include <vector>
#include <string>
#include <regex>
#include <memory>
#include <sstream>
#include <tuple>

// QType Enum
enum class QType {
    DEFAULT,
    QInt,
    QFloat,
    QString,
    QArray,
    QJson
};

int QTypeToValue(QType qType) {
    switch (qType) {
    case QType::DEFAULT: return 0;
    case QType::QInt: return 1;
    case QType::QFloat: return 2;
    case QType::QString: return 3;
    case QType::QArray: return 4;
    default: return -1;
    }
}

// QTuple Class
template<typename T>
class QTuple {
public:
    std::tuple<int, int64_t, double, std::string, std::vector<T>> field;

    QTuple() : field(std::make_tuple(0, 0, 0.0, "", std::vector<T>())) {}

    QTuple(int64_t qinteger, double qfloat, std::string qstring, std::vector<T> qvec)
        : field(std::make_tuple(0, qinteger, qfloat, qstring, qvec)) {}
};

// QValue Struct
struct QValue {
    QType qType;
    size_t meta_data;
    std::string buffer;

    QValue() : qType(QType::DEFAULT), meta_data(0), buffer("") {}

    QValue(QType type, size_t meta_data, std::string data)
        : qType(type), meta_data(meta_data), buffer(data) {}

    std::string convert() {
        return "";
    }
};

// QAction Interface
class QAction {
public:
    virtual void Initialize() = 0;
    virtual QValue get_value() = 0;
    virtual std::string get_buffer() = 0;
};

// QInteger Class
class QInteger : public QAction {
public:
    QValue val;
    int64_t data;

    QInteger(int64_t data) : val(QType::QInt, 0, ""), data(data) {
        Initialize();
    }

    void Initialize() override {
        val.meta_data = 0;
        val.buffer = "[" + std::to_string(QTypeToValue(val.qType)) + ":" + std::to_string(val.meta_data) + ":" + std::to_string(data) + "]";
    }

    QValue get_value() override {
        return val;
    }

    std::string get_buffer() override {
        return val.buffer;
    }
};

// QFloat Class
class QFloat : public QAction {
public:
    QValue val;
    double data;

    QFloat(double data) : val(QType::QFloat, 0, ""), data(data) {
        Initialize();
    }

    void Initialize() override {
        val.meta_data = 0;
        val.buffer = "[" + std::to_string(QTypeToValue(val.qType)) + ":" + std::to_string(val.meta_data) + ":" + std::to_string(data) + "]";
    }

    QValue get_value() override {
        return val;
    }

    std::string get_buffer() override {
        return val.buffer;
    }
};

// QString Class
class QString : public QAction {
public:
    QValue val;
    std::string data;

    QString(std::string data) : val(QType::QString, data.size(), ""), data(data) {
        Initialize();
    }

    void Initialize() override {
        val.meta_data = data.size();
        val.buffer = "[" + std::to_string(QTypeToValue(val.qType)) + ":" + std::to_string(val.meta_data) + ":" + data + "]";
    }

    QValue get_value() override {
        return val;
    }

    std::string get_buffer() override {
        return val.buffer;
    }
};

// QArray Class
template<typename T>
class QArray : public QAction {
public:
    QValue val;
    QType elem_type;
    std::vector<T> data;

    QArray(std::vector<T> data, QType elem_type) : val(QType::QArray, data.size(), ""), elem_type(elem_type), data(data) {
        Initialize();
    }

    void Initialize() override {
        val.meta_data = data.size();
        std::ostringstream buffer;
        buffer << "[" << QTypeToValue(val.qType) << ":" << val.meta_data << ":=" << QTypeToValue(elem_type) << "=";
        for (const auto& elem : data) {
            buffer << elem << ",";
        }
        std::string buf = buffer.str();
        buf.pop_back();
        buf += "]";
        val.buffer = buf;
    }

    QValue get_value() override {
        return val;
    }

    std::string get_buffer() override {
        return val.buffer;
    }
};

// QMessage Class
class QMessage {
public:
    int64_t id;
    size_t size;
    std::vector<std::string> data;

    QMessage(int64_t id, size_t size, std::vector<std::string> data) : id(id), size(size), data(data) {}

    int64_t get_id() const {
        return id;
    }

    size_t get_size() const {
        return size;
    }

    std::vector<std::string> get_data() const {
        return data;
    }
};

std::tuple<uint32_t, uint32_t, std::string> deserialize(const std::string& input) {
    std::regex re(R"((\d+):(\d+):(.*))");
    std::smatch matches;
    if (std::regex_search(input, matches, re) && matches.size() > 3) {
        uint32_t id = std::stoi(matches[1].str());
        uint32_t size = std::stoi(matches[2].str());
        std::string data = matches[3].str();
        return std::make_tuple(id, size, data);
    }
    else {
        return std::make_tuple(0, 0, "");
    }
}

std::vector<std::string> extract_data(const std::string& input) {
    std::regex re(R"(\[[^\[\]]*\])");
    std::sregex_iterator begin(input.begin(), input.end(), re);
    std::sregex_iterator end;
    std::vector<std::string> result;
    for (std::sregex_iterator i = begin; i != end; ++i) {
        result.push_back((*i).str());
    }
    return result;
}

std::string serialize(QMessage msg) {
    std::string serialized = std::to_string(msg.get_id()) + ":" + std::to_string(msg.get_size()) + ":{";
    for (const auto& elem : msg.get_data()) {
        serialized += elem;
    }
    serialized += "}";
    return serialized;
}

// int main() {
//     // Example usage
//     QInteger qInt(42);
//     std::cout << qInt.get_buffer() << std::endl;
// 
//     QFloat qFloat(3.14);
//     std::cout << qFloat.get_buffer() << std::endl;
// 
//     QString qString("Hello, World!");
//     std::cout << qString.get_buffer() << std::endl;
// 
//     std::vector<int> vec = { 1, 2, 3, 4, 5 };
//     QArray<int> qArray(vec, QType::QInt);
//     std::cout << qArray.get_buffer() << std::endl;
// 
//     std::vector<std::string> data = { qInt.get_buffer(), qFloat.get_buffer(), qString.get_buffer(), qArray.get_buffer() };
//     QMessage qMessage(1, data.size(), data);
//     std::cout << serialize(qMessage) << std::endl;
// 
//     return 0;
// }