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

fn main() {
    let mut tokens: Vec<String> = tokenize(INPUT);
    tokens.reverse();
    println!("{:#?}", Protocol::parse(&mut tokens));
}
