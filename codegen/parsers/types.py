from pyparsing import *

bool = Keyword("bool")
u8 = Keyword("u8")
u16 = Keyword("u16")
u32 = Keyword("u32")
u64 = Keyword("u64")
i8 = Keyword("i8")
i16 = Keyword("i16")
i32 = Keyword("i32")
i64 = Keyword("i64")
f32 = Keyword("f32")
f64 = Keyword("f64")
VarInt = Keyword("VarInt")
VarLong = Keyword("VarLong")

primitive = bool | u8 | u16 | u32 | u64 | i8 | i16 | i32 | i64 | f32 | f64 | VarInt | VarLong

String = Keyword("String") + Optional("[" + Word(nums) + "]")
Nbt = Keyword("Nbt")

type = primitive | String | Nbt
List = Keyword("List") + "[" + type + "," + Word(nums) + "]"
type = primitive | String | List | Nbt
List = Keyword("List") + "[" + type + "," + Word(nums) + "]"
type = primitive | String | List | Nbt

# With this you technically you can't create triple-nested lists, but you won't need that in the protocol anyway.

