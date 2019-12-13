use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let program_string = input.split(',').collect::<Vec<&str>>();
    let mut program = program_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    program = run_program(program);
    println!("Part 1 = {}", program[0]);
    Ok(())
}

fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let mut i: usize = 0;
    while program[i] != 99 {
        let opcode = program[i];
        let idx_a: usize = program[i + 1] as usize;
        let idx_b: usize = program[i + 2] as usize;
        let idx_r: usize = program[i + 3] as usize;
        if opcode == 1 {
            program[idx_r] = program[idx_a] + program[idx_b];
        } else if opcode == 2 {
            program[idx_r] = program[idx_a] * program[idx_b];
        } else {
            println!("OPS {}", opcode);
        }
        i += 4;
    }
    program
}

fn part2(input: &str) -> Result<()> {
    let program_string = input.split(',').collect::<Vec<&str>>();
    let original_program = program_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut noun = 0;
    let mut verb = 0;
    let program = original_program.to_vec();
    while program[0] != 19_690_720 && verb <= 99 {
        let mut program = original_program.to_vec();
        program[1] = noun;
        program[2] = verb;
        program = run_program(program);
        if program[0] == 19_690_720 {
            break;
        }
        noun += 1;
        if noun > 99 {
            noun = 0;
            verb += 1;
        }
    }
    println!("Part 2 = {}", 100 * noun + verb);
    Ok(())
}
