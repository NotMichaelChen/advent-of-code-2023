use std::error::Error;
use std::fs::read_to_string;

fn read_lines() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_lines()?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (i, line) in input.iter().enumerate() {
        let game_num = i + 1;

        let valid_game = is_valid_game(&line)?;

        if valid_game {
            sum += game_num;
        }
    }

    println!("part 1: {}", sum);

    Ok(())
}

fn part2(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in input.iter() {
        sum += game_power(&line)?;
    }

    println!("part 2: {}", sum);

    Ok(())
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse_game(game_str: &str) -> Result<Vec<Round>, Box<dyn Error>> {
    let (_, rounds_str) = game_str
        .split_once(':')
        .ok_or(format!("Could not get rounds from game: {}", game_str))?;

    rounds_str
        .trim()
        .split(';')
        .map(|s| parse_round(s.trim()))
        .collect()
}

fn parse_round(round_str: &str) -> Result<Round, Box<dyn Error>> {
    let mut round = Round {
        red: 0,
        blue: 0,
        green: 0,
    };

    for cubes_str in round_str.split(',') {
        let trimmed = cubes_str.trim();
        let (count_str, color_str) = trimmed
            .split_once(' ')
            .ok_or(format!("Could not split cubes_str: {}", cubes_str))?;

        let count = count_str.parse::<i32>()?;

        match color_str {
            "red" => round.red = count,
            "green" => round.green = count,
            "blue" => round.blue = count,
            _ => return Err(format!("Invalid cube color in round: {}", round_str).into()),
        }
    }

    Ok(round)
}

fn is_valid_game(line: &str) -> Result<bool, Box<dyn Error>> {
    let parsed_rounds = parse_game(line)?;

    for round in parsed_rounds {
        if round.red > 12 || round.green > 13 || round.blue > 14 {
            return Ok(false);
        }
    }

    Ok(true)
}

fn game_power(line: &str) -> Result<i32, Box<dyn Error>> {
    let parsed_rounds = parse_game(line)?;

    let mut max_round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for round in parsed_rounds {
        if round.red > max_round.red {
            max_round.red = round.red;
        }
        if round.green > max_round.green {
            max_round.green = round.green;
        }
        if round.blue > max_round.blue {
            max_round.blue = round.blue;
        }
    }

    Ok(max_round.red * max_round.green * max_round.blue)
}
