use anyhow::Context;
use anyhow::Result;
use std::env;
use std::fs;
use std::io::Read;

pub fn read_input_day(day: usize) -> Result<String> {
    println!("Reading input for day {}", day);

    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        let file_path = format!("inputs/day{:02}.txt", day);
        return fs::read_to_string(&file_path)
            .with_context(|| format!("could not open file {}", &file_path));
    }

    assert_eq!(args.len(), 2);

    if args[1] == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        return Ok(buf);
    }

    let file_path = &args[1];
    fs::read_to_string(file_path).with_context(|| format!("could not read file {}", file_path))
}
