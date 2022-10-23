use rand::Rng;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let max = if Some(args.get(1)) { &args[1] } else { 10 };

    let mut max = 10;
    if !args.get(1).is_none() {
        max = args[1].parse::<i32>().unwrap();
    }

    // let max = match args.get(1) {
    //     Some(x) => $args[1],
    //     None => 10
    // };

    let mut rng = rand::thread_rng();

    let dictionnary = fs::read_to_string("./src/dictionnary.txt").expect("Dictionnary file loaded");
    let dictionnary_split: String = dictionnary.split("\n").collect();

    for _i in 0..max {
        println!("{}", rng.gen_range(0..dictionnary_split.len()));
    }
}
