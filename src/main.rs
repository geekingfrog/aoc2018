
use std::env::args;

mod util;
mod pb1;

fn main() {
    let args : Vec<String> = args().skip(1).collect();
    if args.len() == 0 {
        println!("usage: aoc <n> <args>");
    } else {
        match args[0].parse::<i32>() {
            Ok(1) => pb1::run(&args[1..]),
            _ => panic!("unknown number {:?}", args),
        }
    }
}
