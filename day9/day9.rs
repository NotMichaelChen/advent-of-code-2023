use std::error::Error;
use std::fs::read_to_string;

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
    let result = compute_result(input, false)?;

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let result = compute_result(input, true)?;

    println!("part 2: {}", result);

    Ok(())
}

fn compute_result(input: &Vec<String>, is_prev: bool) -> GenResult<i32> {
    let all_nums = input
        .iter()
        .map(|s| parse_line(s))
        .collect::<Result<Vec<_>, _>>()?;

    let values = all_nums
        .iter()
        .map(|nums| extrapolate(nums, is_prev))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(values.iter().sum())
}

fn parse_line(line: &str) -> GenResult<Vec<i32>> {
    Ok(line
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn extrapolate(nums: &Vec<i32>, is_prev: bool) -> GenResult<i32> {
    if nums.iter().all(|&n| n == 0) {
        return Ok(0);
    }

    let diffs = compute_diffs(nums)?;

    let diffs_extrapolate = extrapolate(&diffs, is_prev)?;
    let extrapolated = if is_prev {
        nums.first().ok_or("nums is empty")? - diffs_extrapolate
    } else {
        nums.last().ok_or("nums is empty")? + diffs_extrapolate
    };

    Ok(extrapolated)
}

fn compute_diffs(nums: &Vec<i32>) -> GenResult<Vec<i32>> {
    Ok(nums
        .windows(2)
        .map(|window| match window {
            [left, right] => Ok(right - left),
            _ => Err(format!("Invalid window: {:?} from {:?}", window, nums)),
        })
        .collect::<Result<Vec<_>, _>>()?)
}
