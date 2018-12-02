
use std::fs::File;
use std::io::{BufRead,BufReader};
use fxhash::FxHashSet;

fn parse_ints(arg: &str) -> Vec<i32> {
    let file = File::open(arg).unwrap();
    let reader = BufReader::new(file);

    // parse lines as integers
    reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s|
             s.trim().parse::<i32>().unwrap()
         ).collect()
}

fn find_first_repeating(ops: &[i32]) -> i32 {
    let mut set = FxHashSet::default();

    let mut freq = 0;
    for op in ops.iter().cycle() {
        freq += op;
        if set.contains(&freq) {
            return freq;
        } else {
            set.insert(freq);
        }
    };
    unreachable!()
}

pub(crate) fn run(args: &[String]) {
    let ops = parse_ints(&args[0]);

    // find final frequency
    {
        let final_freq =
            ops.iter().fold(0, |x,y| x+y);
        println!("final freq: {}", final_freq);
    }

    // find first repeating frequency
    {
        let freq = find_first_repeating(&ops);
        println!("first repeating freq: {}", freq);
    }
}
