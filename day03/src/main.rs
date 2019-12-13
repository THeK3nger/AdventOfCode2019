extern crate num;

use num::Complex;
use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn without_first(string: &str) -> &str {
    string
        .char_indices()
        .nth(1)
        .and_then(|(i, _)| string.get(i..))
        .unwrap_or("")
}

fn get_points(input: Vec<&str>) -> HashSet<Complex<i32>> {
    let mut result = HashSet::new();
    let mut current = Complex::new(0, 0);
    let mut direction: Complex<i32>;
    for command in input {
        if command.starts_with('R') {
            direction = Complex::new(1, 0);
        } else if command.starts_with('L') {
            direction = Complex::new(-1, 0);
        } else if command.starts_with('U') {
            direction = Complex::new(0, 1);
        } else if command.starts_with('D') {
            direction = Complex::new(0, -1);
        } else {
            panic!("This is wrong!");
        }
        let amount = without_first(command).parse::<i32>().unwrap();
        for _ in 0..amount {
            current += direction;
            result.insert(current);
        }
    }
    result
}

fn get_points_vec(input: Vec<&str>) -> Vec<Complex<i32>> {
    let mut result = Vec::new();
    let mut current = Complex::new(0, 0);
    let mut direction: Complex<i32>;
    for command in input {
        if command.starts_with('R') {
            direction = Complex::new(1, 0);
        } else if command.starts_with('L') {
            direction = Complex::new(-1, 0);
        } else if command.starts_with('U') {
            direction = Complex::new(0, 1);
        } else if command.starts_with('D') {
            direction = Complex::new(0, -1);
        } else {
            panic!("This is wrong!");
        }
        let amount = without_first(command).parse::<i32>().unwrap();
        for _ in 0..amount {
            current += direction;
            result.push(current);
        }
    }
    result
}

fn part1(input: &str) -> Result<()> {
    let split_input: Vec<&str> = input.lines().collect();
    let line_a: Vec<&str> = split_input[0].split(',').collect();
    let line_b: Vec<&str> = split_input[1].split(',').collect();
    let set_a = get_points(line_a);
    let set_b = get_points(line_b);
    let crossing = set_a.intersection(&set_b);
    let mut min_distance = 100_000_000;
    for x in crossing {
        let dist = x.re.abs() + x.im.abs();
        if dist < min_distance {
            min_distance = dist;
        }
    }
    println!("Part 1 = {}", min_distance);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let split_input: Vec<&str> = input.lines().collect();
    let line_a: Vec<&str> = split_input[0].split(',').collect();
    let line_b: Vec<&str> = split_input[1].split(',').collect();
    let vec_a = get_points_vec(line_a);
    let vec_b = get_points_vec(line_b);
    let set_a: HashSet<_> = vec_a.iter().cloned().collect();
    let set_b: HashSet<_> = vec_b.iter().cloned().collect();
    let crossing = set_a.intersection(&set_b);
    let mut min_step = 100_000_000;
    for x in crossing {
        let idx_a = vec_a.iter().position(|&r| r == *x).unwrap();
        let idx_b = vec_b.iter().position(|&r| r == *x).unwrap();
        let step = idx_a + 1 + idx_b + 1;
        if step < min_step {
            min_step = step;
        }
    }

    println!("Part 2 = {}", min_step);
    Ok(())
}
