use macros::parse_enum;

fn main() {
    parse_enum! {
        enum ASD(VarInt) {
            A
            B
            C
        }
    }
}
