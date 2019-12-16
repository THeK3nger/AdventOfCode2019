use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn digits(num: i32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn non_decreasing(i: i32) -> bool {
    let digits = digits(i);
    let mut prev = digits[0];
    for d in digits {
        if d < prev {
            return false;
        }
        prev = d;
    }
    true
}

fn has_pair(i: i32) -> bool {
    let digits = digits(i);
    let mut prev = digits[0];
    for d in digits.into_iter().skip(1) {
        if d == prev {
            return true;
        }
        prev = d;
    }
    false
}

fn has_valid_pair(i: i32) -> bool {
    let digits = digits(i);
    let digits_test = digits.clone();
    for d in digits {
        let num = digits_test.clone().iter().filter(|&n| *n == d).count();
        if num == 2 {
            return true;
        }
    }
    false
}

fn is_valid(i: i32) -> bool {
    non_decreasing(i) && has_pair(i)
}

fn is_more_valid(i: i32) -> bool {
    non_decreasing(i) && has_valid_pair(i)
}

fn part1(input: &str) -> Result<()> {
    let split = input.split('-').collect::<Vec<&str>>();
    let min = split[0].parse::<i32>().unwrap();
    let max = split[1].parse::<i32>().unwrap();
    let mut count = 0;
    for i in min..max {
        if is_valid(i) {
            count += 1;
        }
    }
    println!("Part 1 = {}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let split = input.split('-').collect::<Vec<&str>>();
    let min = split[0].parse::<i32>().unwrap();
    let max = split[1].parse::<i32>().unwrap();
    let mut count = 0;
    for i in min..max {
        if is_more_valid(i) {
            count += 1;
        }
    }
    println!("Part 2 = {}", count);
    Ok(())
}
