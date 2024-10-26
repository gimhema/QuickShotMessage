# Overview

QuickShotMessage generator

## Detail

### Message File Format (for generate)

<MESSAGE_FILE_MANE>.qsmb
```
msg <MESSAGE_NAME>
{
  <TYPE_NAME> <VALUE_NAME>
}

```

### Example

ExampleMessage.qsmb
```
msg ExampleMessage
{
  Integer intVal
  Float  floatVal
  String stringVal
}
```

### Generate Output Result


#### cpp

QS_ExampleMessage.hpp
```
```

#### Rust
QS_ExampleMessage.rs
```
```


## Command Usage

- qnerator <FILE_NAME> <LANGUAGE> <GENERATE_DIRECTORY> : Generate quickshot message file as selected language and move file in the <GENERATE_DIRECTORY>


### Example
```
console input : qnerator ExampleMEssage.qsmb cpp Example
console output : will generate QS_ExampleMessage.hpp in Example directory
```

- qnerator <DIRECTORY> <LANGUAGE> <GENERATE_DIRECTORY> : Search all qsmb file in the <DIRECTORY> and generate quickshot message file as selected language and move files in the <GENERATE_DIRECTORY>











