use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
#[allow(dead_code)]
mod day6;
mod day6_fast;
mod day7;
fn main() {
    let start = Instant::now();
    day1::solve();
    day2::solve();
    day3::solve();
    day4::solve();
    day5::solve();
    //day6::solve();
    day6_fast::solve();
    day7::solve();
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}