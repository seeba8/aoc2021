use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
#[allow(dead_code)]
mod day06;
mod day06_fast;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
fn main() {
    let start = Instant::now();
    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    //day6::solve();
    day06_fast::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}
