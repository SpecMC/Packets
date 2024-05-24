from enum import Enum

class IntegerType(Enum):
    U8 = 0
    U16 = 1
    U32 = 2
    U64 = 3
    I8 = 4
    I16 = 5
    I32 = 6
    I64 = 7
    VarInt = 8
    VarLong = 9

class _Enum:
    name: str
    type: str
    fields: list[str, int]
    