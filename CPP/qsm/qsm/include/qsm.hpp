#include<string>
#include<vector>
#include<unordered_map>

namespace QType
{
	enum Type
	{
		Default,
		QInt,
		QFloat,
		QString,
		QArray,
		QJson
	};
};

class QValue
{
public:
	QValue()
	{

	}
	~QValue()
	{

	}
public:
	QType::Type type = QType::Type::Default;

public:
	std::string convert()
	{
		return "";
	}
};

class QInteger : public QValue
{
public:
	QInteger()
	{
		type = QType::QInt;
	}
	~QInteger()
	{

	}
	
public:
	std::string convert()
	{
		return "";
	}


public:

};

class QFloat : public QValue
{
public:
	QFloat()
	{
		type = QType::QFloat;
	}
	~QFloat()
	{

	}

public:
	std::string convert()
	{
		return "";
	}

public:

};



class QString : public QValue
{
public:
	QString()
	{
		type = QType::QString;
	}
	~QString()
	{

	}

public:
	std::string convert()
	{
		return "";
	}

public:

};

class QArray : public QValue
{
public:
	QArray(QType::Type _elemType)
	{
		type = QType::QArray;
		elemType = _elemType;
	}
	~QArray()
	{

	}

public:
	QType::Type elemType = QType::Type::Default;

public:
	std::string convert()
	{
		return "";
	}
};

class QJson : public QValue
{
public:
	QJson()
	{
		type = QType::QJson;
	}
	~QJson()
	{

	}

public:
	std::string convert()
	{
		return "";
	}

public:

};

class QMessage
{
public:
	QMessage()
	{

	}
	~QMessage()
	{

	}
public:
	int MessageID;
	std::vector<QValue> values;

public:
	const char* build()
	{
		std::string dest = "";

		for (int i = 0; i < values.size(); ++i)
		{
			dest = dest + values[i].convert();
		}

		dest = std::to_string(MessageID) + dest;

		return dest.c_str();
	}
};


namespace QuickShot
{
	QMessage build_message_from_buffer(const char* buffer)
	{
		QMessage result;

		return result;
	}
}

