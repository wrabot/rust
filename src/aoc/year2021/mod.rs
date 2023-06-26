mod day15;

pub fn year2021() {
    println!("year2021");
    check_day!(day15::part1, "day15_test1.txt", 40);
    check_day!(day15::part1, "day15_input.txt", 562);
    check_day!(day15::part2, "day15_test1.txt", 315);
    check_day!(day15::part2, "day15_input.txt", 2874);
}
