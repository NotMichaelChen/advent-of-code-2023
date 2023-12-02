use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line_res in reader.lines() {
        let line = line_res?;
        let number1 = parse_number_p1(&line)?;
        sum1 += number1;

        let number2 = parse_number_p2(&line)?;
        sum2 += number2;
    }

    println!("part 1: {}", sum1);
    println!("part 2: {}", sum2);

    Ok(())
}

fn parse_number_p1(line: &str) -> Result<i32, Box<dyn error::Error>> {
    let digit_one = line
        .chars()
        .find(|&c| char::is_numeric(c))
        .ok_or(format!("Could not find first digit: {}", line))?;

    let digit_two = line
        .chars()
        .rfind(|&c| char::is_numeric(c))
        .ok_or(format!("Could not find second digit: {}", line))?;

    let res = format!("{}{}", digit_one, digit_two).parse::<i32>()?;

    Ok(res)
}

fn parse_number_p2(line: &str) -> Result<i32, Box<dyn error::Error>> {
    fn consume_number(s: &str) -> Option<i32> {
        let head = s.chars().next()?;

        if char::is_numeric(head) {
            head.to_digit(10).map(|n| n as i32)
        } else {
            if s.starts_with("one") {
                Some(1)
            } else if s.starts_with("two") {
                Some(2)
            } else if s.starts_with("three") {
                Some(3)
            } else if s.starts_with("four") {
                Some(4)
            } else if s.starts_with("five") {
                Some(5)
            } else if s.starts_with("six") {
                Some(6)
            } else if s.starts_with("seven") {
                Some(7)
            } else if s.starts_with("eight") {
                Some(8)
            } else if s.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        }
    }

    let parsed_nums: Vec<Option<i32>> = line
        .char_indices()
        .map(|(index, _)| consume_number(line.get(index..)?))
        .collect();

    let nums: Vec<i32> = parsed_nums.into_iter().flatten().collect();

    let first = nums
        .first()
        .ok_or(format!("Could not find first digit (p2): {}", line))?;

    let second = nums
        .last()
        .ok_or(format!("Could not find second digit (p2): {}", line))?;

    let res = format!("{}{}", first, second).parse::<i32>()?;

    Ok(res)
}
