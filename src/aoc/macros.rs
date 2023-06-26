#[macro_export]
macro_rules! check_day {
    ($f:path, $input:literal, $output:literal) => {
        let result = $f(include_str!($input));
        if result != $output {
            eprintln!(
                "error: {} with {} -> {} != {}",
                stringify!($f),
                $input,
                result,
                $output
            )
        }
    };
}

#[macro_export]
macro_rules! check_day_with_duration {
    ($f:path, $input:literal, $output:literal) => {
        let start = std::time::Instant::now();
        check_day!($f, $input, $output);
        println!(
            "duration: {} with {} -> {:?}",
            stringify!($f),
            $input,
            start.elapsed()
        );
    };
}
