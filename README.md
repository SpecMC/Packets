# SpecMC Protocol

A library for parsing Minecraft protocol specification.

## Default types

Primitives:

-   `bool`: Boolean.
-   `u8`, `u16`, `u32`, `u64`: Unsigned integers of size 8, 16, 32 and 64 bits respectively.
-   `i8`, `i16`, `i32`, `i64`: Signed integers of size 8, 16, 32 and 64 bits respectively.
-   `VarInt`, `VarLong`: Regular signed integers of size 32 and 64 bits respectively, but encoded in the LEB128 format.
-   `f32`, `f64`: Floating point numbers of size 32 and 64 bits respectively.

Other types:

-   `String`: UTF-8 encoded string prefixed with its size in bytes as VarInt. The maximum length is 32767. Use `String[n]` to explicitly specify the length.
-   `List[type; n]`: A list, where `type` is the type of the elements and `n` is the number of elements. `n` can also be an identifier, in which case the number of elements is to be determined at runtime.
-   `Nbt`: NBT encoded data.

## Enums

An enum definition consists of an enum name, followed by its representation type, and a list of variants.
Variants will be implicitly assigned a value of the previous variant plus one.
The first variant will be implicitly assigned the value 0.
You can explicitly assign a value to an enum variant.
An enum `State` is necessary and is used for packet definitions.

Example:

```
enum ABC(i32) {
    A = 1
    B // implicitly assigned 2
    C // implicitly assigned 3
}
```

## Packets

A packet definition consists of a packet name, followed by its direction, state and id, and a list of fields.
Enums will be represented as the type specified in the enum definition.
`if` statements can be used to define conditional fields.
Fields of primitive types can be set to equal a literal or another field or constant.

Example:

```
packet SomePacket(serverbound, Play, 0x42) {
    u32 length
    List[u8; length] data
    if (length > 0) {
        String message
    }
}
```

## Types

A custom type can be defined using the `type` keyword.
Fields are equivalent to fields in a packet.

## Constants

Constants can be defined using the `const` keyword.
A constant `PVN` is automatically defined.

## Examples

```rust
use specmc_base::{parse::Parse, tokenize::tokenize};
use specmc_protocol::Protocol;

const INPUT: &str = "
enum TestEnum(i32) {}
type TestType {
    String message
}
packet TestPacket(serverbound, Play, 0x42) {
    u32 length
    List[u8; length] data
    if (length > 0) {
        TestType message
    }
}";

let mut tokens: Vec<String> = tokenize(INPUT);
tokens.reverse();
println!("{:#?}", Protocol::parse(&mut tokens));
```
