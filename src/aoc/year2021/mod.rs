mod day15;

pub fn year2021() {
    day15::part1(include_str!("day15_test1.txt"), 40);
    day15::part1(include_str!("day15_input.txt"), 562);
    day15::part2(include_str!("day15_test1.txt"), 315);
    day15::part2(include_str!("day15_input.txt"), 2874);
}
