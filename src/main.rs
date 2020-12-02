use std::env;
mod util;
mod day_1;
mod day_2;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Pass day as an argument");
    }

    match args[1].as_str() {
        "1"  => day_1::solve_part_1(),
        "1N" => day_1::solve_part_2(),
        "2"  => day_2::solve_part_1(),
        _    => println!("Unknown day"),
    }
}
