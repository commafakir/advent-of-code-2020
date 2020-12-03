mod util;
mod day_1;
mod day_2;
mod day_3;

trait Challenge {
    fn solve(&self) -> (&str, Option<String>, Option<String>);
}

fn main() {

    println!("SoLViNg AOC 2020 - noobie rust");

    let results = vec![
        day_1::Day1{}.solve(),
        day_2::Day2{}.solve(),
    ];

    for (day, r1, r2) in results {
        println!("{} \t {} // {}", 
            day, 
            r1.unwrap_or("**FAILS**".to_string()), 
            r2.unwrap_or("**FAILS**".to_string()));
    }

    
}
