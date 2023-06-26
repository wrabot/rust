pub fn part1(input: &str) -> i32 { calories(input).into_iter().max().unwrap() }

pub fn part2(input: &str) -> i32 { calories(input).into_iter().sum() }

fn calories(input: &str) -> Vec<i32> {
    return input.split("\n\n").map(|elf|
        elf.split("\n").map(|snack|
            snack.parse::<i32>().unwrap()
        ).sum()
    ).collect::<Vec<_>>();
}