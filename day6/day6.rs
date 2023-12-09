use std::error::Error;
use std::fs::read_to_string;
use std::iter::zip;

type GenResult<T> = Result<T, Box<dyn Error>>;

fn read_lines() -> GenResult<Vec<String>> {
    Ok(read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect())
}

fn main() -> GenResult<()> {
    let input = read_lines()?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &Vec<String>) -> GenResult<()> {
    let races = parse_races(input)?;
    let result: i64 = races.iter().map(compute_margin_count).product();

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let race = parse_race(input)?;
    let result = compute_margin_count(&race);

    println!("part 2: {}", result);

    Ok(())
}

struct Race {
    time: i64,
    distance: i64,
}

fn compute_margin_count(race: &Race) -> i64 {
    let time_float = race.time as f64;
    let dist_float = race.distance as f64;

    let lower_bound = compute_margin(time_float, dist_float, false).floor() as i64;
    let upper_bound = compute_margin(time_float, dist_float, true).ceil() as i64;

    upper_bound - lower_bound - 1
}

fn compute_margin(time: f64, distance: f64, is_addition: bool) -> f64 {
    let mult = if is_addition { 1.0 } else { -1.0 };
    let top = time + mult * (time.powf(2.0) - (4.0 * distance)).sqrt();
    top / 2.0
}

fn parse_race(input: &Vec<String>) -> GenResult<Race> {
    let parsed_lines = input.iter().map(parse_num).collect::<Result<Vec<_>, _>>()?;

    let time = parsed_lines.get(0).ok_or(format!(
        "Expected parsed_lines size to equal 2: {:?}",
        parsed_lines
    ))?;
    let distance = parsed_lines.get(1).ok_or(format!(
        "Expected parsed_lines size to equal 2: {:?}",
        parsed_lines
    ))?;

    Ok(Race {
        time: *time,
        distance: *distance,
    })
}

fn parse_num(line: &String) -> GenResult<i64> {
    let (_, raw_untrimmed_nums) = line
        .split_once(':')
        .ok_or(format!("Expected ':' in line: {}", line))?;

    Ok(raw_untrimmed_nums
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()?)
}

fn parse_races(input: &Vec<String>) -> GenResult<Vec<Race>> {
    let parsed_lines = input
        .iter()
        .map(parse_nums)
        .collect::<Result<Vec<_>, _>>()?;

    let time = parsed_lines.get(0).ok_or(format!(
        "Expected parsed_lines size to equal 2: {:?}",
        parsed_lines
    ))?;
    let distance = parsed_lines.get(1).ok_or(format!(
        "Expected parsed_lines size to equal 2: {:?}",
        parsed_lines
    ))?;

    Ok(zip(time, distance)
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect())
}

fn parse_nums(line: &String) -> GenResult<Vec<i64>> {
    let (_, raw_untrimmed_nums) = line
        .split_once(':')
        .ok_or(format!("Expected ':' in line: {}", line))?;

    Ok(raw_untrimmed_nums
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}
