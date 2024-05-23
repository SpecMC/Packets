use macros::protocol_def;

fn main() {}

protocol_def! {
    enum ASD(VarInt) {
        A
        B = 19
        C
    }

    enum BCD(VarInt) {
        D = 1
        E = 5
        F
    }
}
