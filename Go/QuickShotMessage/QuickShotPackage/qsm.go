package quickshotmessage

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// QType Enum
type EType int

const (
	DEFAULT EType = iota
	EInt
	EFloat
	EString
	EArray
	EJson
)

func QTypeToValue(qType EType) int {
	switch qType {
	case DEFAULT:
		return 0
	case EInt:
		return 1
	case EFloat:
		return 2
	case EString:
		return 3
	case EArray:
		return 4
	default:
		return -1
	}
}

// QTuple Struct
type QTuple struct {
	Field struct {
		EType
		QInteger int64
		QFloat   float64
		QString  string
		QArray   []interface{}
	}
}

// QValue Struct
type QValue struct {
	EType
	MetaData int
	Buffer   string
}

func (qv *QValue) convert() string {
	return ""
}

// QAction Interface
type QAction interface {
	Initialize()
	getValue() QValue
	getBuffer() string
}

// QInteger Struct
type QInteger struct {
	Val  QValue
	Data int64
}

func NewQInteger(data int64) *QInteger {
	qInt := &QInteger{}
	qInt.Val.EType = EInt
	qInt.Val.MetaData = 0
	qInt.Val.Buffer = fmt.Sprintf("[%d:%d:%d]", QTypeToValue(qInt.Val.EType), qInt.Val.MetaData, data)
	qInt.Data = data
	return qInt
}

func (qi *QInteger) Initialize() {
	qi.Val.MetaData = 0
	qi.Val.Buffer = fmt.Sprintf("[%d:%d:%d]", QTypeToValue(qi.Val.EType), qi.Val.MetaData, qi.Data)
}

func (qi *QInteger) getValue() QValue {
	return qi.Val
}

func (qi *QInteger) getBuffer() string {
	return qi.Val.Buffer
}

// QFloat Struct
type QFloat struct {
	Val  QValue
	Data float64
}

func NewQFloat(data float64) *QFloat {
	qFloat := &QFloat{}
	qFloat.Val.EType = EFloat
	qFloat.Val.MetaData = 0
	qFloat.Val.Buffer = fmt.Sprintf("[%d:%d:%f]", QTypeToValue(qFloat.Val.EType), qFloat.Val.MetaData, data)
	qFloat.Data = data
	return qFloat
}

func (qf *QFloat) Initialize() {
	qf.Val.MetaData = 0
	qf.Val.Buffer = fmt.Sprintf("[%d:%d:%f]", QTypeToValue(qf.Val.EType), qf.Val.MetaData, qf.Data)
}

func (qf *QFloat) getValue() QValue {
	return qf.Val
}

func (qf *QFloat) getBuffer() string {
	return qf.Val.Buffer
}

// QString Struct
type QString struct {
	Val  QValue
	Data string
}

func NewQString(data string) *QString {
	qString := &QString{}
	qString.Val.EType = EString
	qString.Val.MetaData = len(data)
	qString.Val.Buffer = fmt.Sprintf("[%d:%d:%s]", QTypeToValue(qString.Val.EType), qString.Val.MetaData, data)
	qString.Data = data
	return qString
}

func (qs *QString) Initialize() {
	qs.Val.MetaData = len(qs.Data)
	qs.Val.Buffer = fmt.Sprintf("[%d:%d:%s]", QTypeToValue(qs.Val.EType), qs.Val.MetaData, qs.Data)
}

func (qs *QString) getValue() QValue {
	return qs.Val
}

func (qs *QString) getBuffer() string {
	return qs.Val.Buffer
}

// QArray Struct
type QArray struct {
	Val      QValue
	ElemType EType
	Data     []interface{}
}

func NewEArray(data []interface{}, elemType EType) *QValue {
	var buffer strings.Builder
	buffer.WriteString(fmt.Sprintf("[%d:", EArray))
	buffer.WriteString(fmt.Sprintf("%d:", len(data)))
	buffer.WriteString(fmt.Sprintf("=%d=", elemType))
	for _, elem := range data {
		buffer.WriteString(fmt.Sprintf("%v,", elem))
	}
	result := buffer.String()
	if len(result) > 0 {
		result = result[:len(result)-1]
	}
	result += "]"
	return &QValue{EType: EArray, MetaData: len(data), Buffer: result}
}

func (qa *QArray) Initialize() {
	qa.Val.MetaData = len(qa.Data)
	var buffer strings.Builder
	buffer.WriteString(fmt.Sprintf("[%d:%d:=%d=", QTypeToValue(qa.Val.EType), qa.Val.MetaData, QTypeToValue(qa.ElemType)))
	for _, elem := range qa.Data {
		buffer.WriteString(fmt.Sprintf("%v,", elem))
	}
	result := buffer.String()
	if len(result) > 0 {
		result = result[:len(result)-1]
	}
	result += "]"
	qa.Val.Buffer = result
}

func (qa *QArray) getValue() QValue {
	return qa.Val
}

func (qa *QArray) getBuffer() string {
	return qa.Val.Buffer
}

// QMessage Struct
type QMessage struct {
	ID   int64
	Size int
	Data []string
}

func NewQMessage(id int64, size int, data []string) *QMessage {
	return &QMessage{
		ID:   id,
		Size: size,
		Data: data,
	}
}

func (qm *QMessage) getID() int64 {
	return qm.ID
}

func (qm *QMessage) getSize() int {
	return qm.Size
}

func (qm *QMessage) getData() []string {
	return qm.Data
}

func deserialize(input string) (uint32, uint32, string) {
	re := regexp.MustCompile(`(\d+):(\d+):(.*)`)
	matches := re.FindStringSubmatch(input)
	if len(matches) > 3 {
		id, _ := strconv.Atoi(matches[1])
		size, _ := strconv.Atoi(matches[2])
		data := matches[3]
		return uint32(id), uint32(size), data
	}
	return 0, 0, ""
}

func extractData(input string) []string {
	re := regexp.MustCompile(`\[[^\[\]]*\]`)
	matches := re.FindAllString(input, -1)
	var result []string
	for _, match := range matches {
		result = append(result, match)
	}
	return result
}

func serialize(msg *QMessage) string {
	var serialized strings.Builder
	serialized.WriteString(fmt.Sprintf("%d:%d:{", msg.ID, msg.Size))
	for _, elem := range msg.Data {
		serialized.WriteString(elem)
	}
	serialized.WriteString("}")
	return serialized.String()
}
