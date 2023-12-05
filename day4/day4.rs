use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::FromIterator;

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
    let mut result = 0;

    for line in input {
        let card = parse_card(line)?;
        let match_count = card.numbers.intersection(&card.winnings).count();
        if match_count > 0 {
            result += 2i32
                .checked_pow(match_count as u32 - 1)
                .ok_or(format!("pow overflowed on 2^{}", match_count))?
        }
    }

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let cards = input
        .iter()
        .map(parse_card)
        .collect::<GenResult<Vec<_>>>()?;

    let mut copies = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let num_cards_copy = card.numbers.intersection(&card.winnings).count();
        let copy_amount = *copies.get(i).ok_or(format!(
            "index out of bounds, {} out of {}",
            i,
            copies.len()
        ))?;

        let begin_index = i + 1;
        let end_index = i + (num_cards_copy as usize);

        for j in begin_index..=end_index {
            let copies_len = copies.len();
            let write_ref = copies
                .get_mut(j)
                .ok_or(format!("index out of bounds, j={}, len={}", j, copies_len))?;
            *write_ref += copy_amount;
        }
    }

    let result: i32 = copies.iter().sum();

    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug)]
struct ScratchCard {
    numbers: HashSet<i32>,
    winnings: HashSet<i32>,
}

fn parse_card(line: &String) -> GenResult<ScratchCard> {
    let (_, card) = line
        .split_once(':')
        .ok_or(format!("Could not find ':' in line: {}", line))?;

    let (numbers_raw, winnings_raw) = card
        .trim()
        .split_once('|')
        .ok_or(format!("Could not find '|' in line: {}", line))?;

    let numbers_vec = numbers_raw
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let winnings_vec = winnings_raw
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ScratchCard {
        numbers: HashSet::from_iter(numbers_vec),
        winnings: HashSet::from_iter(winnings_vec),
    })
}
