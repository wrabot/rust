mod day1;
mod day16;

pub fn year2022() {
    println!("year2022");
    check_input!(day1::part1, "day1_test1.txt", 24000);
    check_input!(day1::part1, "day1_input.txt", 66487);
    check_input!(day1::part2, "day1_test1.txt", 45000);
    check_input!(day1::part2, "day1_input.txt", 197301);
    //assert_eq!(day1::part2(include_str!("day1_test1.txt")), 45000);
    //assert_eq!(day1::part2(include_str!("day1_input.txt")), 197301);
    check_input!(day16::part1, "day16_input.txt", 2077);
    check_input_with_duration!(day16::part2, "day16_input.txt", 2741);
}
