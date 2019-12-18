use std::io::{self, BufRead};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn parse_opcode(i: i32) -> (i32, i32) {
    (i % 100, i / 100)
}

fn parse_parameter_mode_flags(i: i32) -> (i32, i32, i32) {
    (i % 10, (i / 10) % 10, (i / 100) % 10)
}

fn parse_one_parameter(program: &[i32], current_idx: usize, p1_flag: i32) -> i32 {
    let idx_a = program[current_idx + 1];
    if p1_flag == 0 {
        program[idx_a as usize]
    } else {
        idx_a
    }
}

fn parse_two_parameters(
    program: &[i32],
    current_idx: usize,
    p1_flag: i32,
    p2_flag: i32,
) -> (i32, i32) {
    let idx_a = program[current_idx + 1];
    let idx_b = program[current_idx + 2];
    (
        if p1_flag == 0 {
            program[idx_a as usize]
        } else {
            idx_a
        },
        if p2_flag == 0 {
            program[idx_b as usize]
        } else {
            idx_b
        },
    )
}

fn parse_three_parameters(
    program: &[i32],
    current_idx: usize,
    p1_flag: i32,
    p2_flag: i32,
    p3_flag: i32,
) -> (i32, i32, i32) {
    let idx_a = program[current_idx + 1];
    let idx_b = program[current_idx + 2];
    let idx_c = program[current_idx + 3];
    (
        if p1_flag == 0 {
            program[idx_a as usize]
        } else {
            idx_a
        },
        if p2_flag == 0 {
            program[idx_b as usize]
        } else {
            idx_b
        },
        if p3_flag == 0 {
            println!("{} {}", idx_c, program[idx_c as usize]);
            program[idx_c as usize]
        } else {
            idx_c
        },
    )
}

fn run_program(mut program: Vec<i32>) -> (Vec<i32>, i32) {
    let mut i: usize = 0;
    let mut last_out = -1;
    while program[i] != 99 {
        let (opcode, parameter_mode_flags) = parse_opcode(program[i]);
        let (p1, p2, p3) = parse_parameter_mode_flags(parameter_mode_flags);
        // println!(
        //     "RawCode= {} ~ OpCode = {} ~ Mode={}-{}-{} [{}]",
        //     program[i], opcode, p1, p2, p3, parameter_mode_flags
        // );
        match opcode {
            1 => {
                let (a, b, r) = parse_three_parameters(&program, i, p1, p2, 1);
                // println!("Write {} in {}", a + b, r);
                program[r as usize] = a + b;
                i += 4;
            }
            2 => {
                let (a, b, r) = parse_three_parameters(&program, i, p1, p2, 1);
                program[r as usize] = a * b;
                //println!("Write {} in {}", a * b, idx_r);
                i += 4;
            }
            3 => {
                let idx_a: usize = program[i + 1] as usize;
                let stdin = io::stdin();
                let input_str = stdin.lock().lines().next().unwrap().unwrap();
                let input = input_str.parse::<i32>().unwrap();
                // println!("Save {} in {}", input, idx_a);
                program[idx_a] = input;
                i += 2;
            }
            4 => {
                let a = parse_one_parameter(&program, i, p1);
                println!("{}", a);
                last_out = a;
                i += 2;
            }
            5 => {
                let (a, b) = parse_two_parameters(&program, i, p1, p2);
                if a != 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let (a, b) = parse_two_parameters(&program, i, p1, p2);
                if a == 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let (a, b, r) = parse_three_parameters(&program, i, p1, p2, 1);
                program[r as usize] = if a < b { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let (a, b, r) = parse_three_parameters(&program, i, p1, p2, 1);
                program[r as usize] = if a == b { 1 } else { 0 };
                i += 4;
            }
            _ => {
                println!("OPS {}", opcode);
            }
        };
    }
    (program, last_out)
}

fn part1(input: &str) -> Result<()> {
    let program_string = input.trim().split(',').collect::<Vec<&str>>();
    print!("Parsing IntCode...");
    let program = program_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("[DONE]");
    println!();
    let (_, out) = run_program(program);
    println!();
    println!("Part 1 = {}", out);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let program_string = input.trim().split(',').collect::<Vec<&str>>();
    print!("Parsing IntCode...");
    let program = program_string
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("[DONE]");
    println!();
    let (_, out) = run_program(program);
    println!();
    println!("Part 2 = {}", out);
    Ok(())
}
