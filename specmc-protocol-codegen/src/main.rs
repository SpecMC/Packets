mod parse;
mod tokenize;

use parse::{CustomType, DefaultType, Enum, Parse};
use tokenize::tokenize;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    //
    // if args.len() != 2 {
    //     panic!("Usage: {} <file>", args[0]);
    // }
    //
    // let contents: String = std::fs::read_to_string(&args[1]).unwrap();
    //
    // // Remove comments
    // let contents: String = contents
    //     .split("\n")
    //     .map(|line: &str| {
    //         if let Some(comment_index) = line.find("//") {
    //             &line[..comment_index]
    //         } else {
    //             line
    //         }
    //     })
    //     .filter(|line: &&str| !line.is_empty())
    //     .collect::<Vec<&str>>()
    //     .join("\n");
    //
    // let tokens: Vec<String> = tokenize(&contents);

    let mut tokens: Vec<String> = "type a { i32 a }"
        .split_whitespace()
        .map(|str| str.to_string()) // TODO
        .collect();
    tokens.reverse();
    println!("{:?}", CustomType::parse(&mut tokens));
}
