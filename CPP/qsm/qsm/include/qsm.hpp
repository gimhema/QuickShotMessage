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

	}
	~QInteger()
	{

	}
	
public:


public:

};

class QFloat : public QValue
{
public:
	QFloat()
	{

	}
	~QFloat()
	{

	}

public:


public:

};



class QString : public QValue
{
public:
	QString()
	{

	}
	~QString()
	{

	}

public:


public:

};

class QArray : public QValue
{
public:
	QArray()
	{

	}
	~QArray()
	{

	}

public:


public:

};

class QJson : public QValue
{
public:
	QJson()
	{

	}
	~QJson()
	{

	}

public:


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
	std::vector<QValue> values;

public:
	const char* build()
	{
		std::string dest = "";

		for (int i = 0; i < values.size(); ++i)
		{
			dest = dest + values[i].convert();
		}
		return dest.c_str();
	}
};


namespace QuickShot
{
	
}

