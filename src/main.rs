use std::time::Instant;
#[macro_use]
extern crate lazy_static;

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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day21_part2;
mod day22;
#[allow(dead_code)]
mod day23;
mod day23_fast;
mod day24;
mod day24_generated;
mod day24_interpreted;
mod day25;
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
    day13::solve();
    day14::solve();
    day15::solve();
    day16::solve();
    day17::solve();
    day18::solve();
    day19::solve();
    day20::solve();
    day21::solve();
    day21_part2::solve();
    day22::solve();
    day23_fast::solve();
    day24::solve();
    day25::solve();
    println!("Elapsed time: {}ms", start.elapsed().as_millis());
}
