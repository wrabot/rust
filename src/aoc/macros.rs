#[macro_export]
macro_rules! check_input {
    ($f:path, $input:literal, $output:literal) => {
        if $f(include_str!($input)) != $output {
            eprintln!("error: {} with {} != {}", stringify!($f), $input, $output)
        }
    };
}

#[macro_export]
macro_rules! check_input_with_duration {
    ($f:path, $input:literal, $output:literal) => {
        let start = std::time::Instant::now();
        check_input!($f, $input, $output);
        println!(
            "duration: {} with {} => {:?}",
            stringify!($f),
            $input,
            start.elapsed()
        );
    };
}
