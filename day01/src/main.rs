use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut fuel_sum = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel_sum += compute_fuel(mass);
    }
    writeln!(io::stdout(), "Part 1 = {}", fuel_sum)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut fuel_sum = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        fuel_sum += compute_recursive_fuel(mass);
    }
    writeln!(io::stdout(), "Part 2 = {}", fuel_sum)?;
    Ok(())
}

fn compute_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn compute_recursive_fuel(mass: i32) -> i32 {
    let mut current_fuel = 0;
    let mut additional_fuel = compute_fuel(mass);
    while additional_fuel > 0 {
        current_fuel += additional_fuel;
        additional_fuel = compute_fuel(additional_fuel);
    }
    current_fuel
}
