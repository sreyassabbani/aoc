use std::{env, error::Error};

mod day01;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(env::args().count(), 5); // cargo run [year] -- [day] [part]

    let args: Vec<_> = env::args().collect();

    // shadow
    let args = (args[3].parse()?, args[4].parse()?);

    match args {
        (1, 1) => day01::part1()?,
        (1, 2) => day01::part2()?,
        _ => {}
    }

    Ok(())
}
