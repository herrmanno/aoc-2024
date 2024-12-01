use std::process::exit;

use aoc2024::days::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned()) {
        usage();
        exit(0);
    }

    let mut days = Days::new();
    let (day, part) = get_args();
    if let Some(day) = day {
        let input = std::io::stdin()
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");

        days.run_part(day, part, &input, &mut days.get_analyzer());
    } else {
        let inputs = (1..=days.len())
            .map(|idx| {
                let input_file_path = format!("./input/{:0>2}.txt", idx);
                std::fs::read_to_string(input_file_path).ok()
            })
            .collect::<Vec<Option<String>>>();
        days.run_some(&inputs[..]);
    }
}

fn get_args() -> (Option<usize>, Option<usize>) {
    let args = std::env::args().collect::<Vec<String>>();
    let day = args
        .get(1)
        .map(|arg| arg.parse().expect("'day' must be a number"));
    let part = args
        .get(2)
        .map(|arg| arg.parse().expect("'part' must be a number"));
    (day, part)
}

fn usage() {
    let binary_name = std::env::current_exe()
        .ok()
        .and_then(|path| {
            let file_name = path.file_name()?;
            let name_str = file_name.to_str()?;
            Some(name_str.to_string())
        })
        .unwrap_or_else(|| String::from("<binary>"));
    println!("USAGE: {} [day] [part]", binary_name);
}
