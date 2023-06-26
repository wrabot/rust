#[macro_use]
mod macros;
mod year2021;
mod year2022;

pub fn aoc() {
    year2021::year2021();
    year2022::year2022();
}
