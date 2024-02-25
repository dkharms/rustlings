// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.

const NUMBER: i32 = 3;

fn main() {
    println!("zeroes in binary 0b{:b} {}", NUMBER, NUMBER.count_ones(),);
}
