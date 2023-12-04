use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::FromIterator;

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
    let (numbers, symbols) = parse_schematic(input)?;

    let result: i32 = numbers
        .iter()
        .map(|num| {
            if is_part_number(num, &symbols) {
                num.num
            } else {
                0
            }
        })
        .sum();

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let (numbers, symbols) = parse_schematic(input)?;

    let result: i32 = symbols
        .iter()
        .map(|symb| gear_ratio(symb, &numbers))
        .flatten()
        .sum();

    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug)]
struct Number {
    num: i32,
    row: i32,
    start_col: i32,
    end_col: i32,
}

#[derive(Debug)]
struct Symbol {
    symb: String,
    row: i32,
    col: i32,
}

fn parse_schematic(
    schematic_raw: &Vec<String>,
) -> Result<(Vec<Number>, Vec<Symbol>), Box<dyn Error>> {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for (row, line) in schematic_raw.iter().enumerate() {
        let mut row_numbers = parse_numbers(row as i32, line)?;
        let mut row_symbols = parse_symbols(row as i32, line)?;

        numbers.append(&mut row_numbers);
        symbols.append(&mut row_symbols);
    }

    Ok((numbers, symbols))
}

fn parse_numbers(row: i32, line: &str) -> Result<Vec<Number>, Box<dyn Error>> {
    let mut numbers = vec![];

    let num_chars = line.match_indices(char::is_numeric).collect::<Vec<_>>();

    let mut ind_splits = vec![];
    for window in num_chars.windows(2) {
        let (prev_ind, _) = window[0];
        let (next_ind, _) = window[1];

        if prev_ind + 1 != next_ind {
            ind_splits.push(prev_ind);
        }
    }

    let ind_splits_set: HashSet<usize> = HashSet::from_iter(ind_splits);
    for ind_digit_pairs in num_chars.split_inclusive(|(i, _)| ind_splits_set.contains(i)) {
        let start_col = ind_digit_pairs
            .iter()
            .map(|(i, _)| i)
            .cloned()
            .min()
            .ok_or(format!(
                "Could not find min of number in {:?}",
                ind_digit_pairs
            ))?;

        let end_col = ind_digit_pairs
            .iter()
            .map(|(i, _)| i)
            .cloned()
            .max()
            .ok_or(format!(
                "Could not find max of number in {:?}",
                ind_digit_pairs
            ))?;

        let num = ind_digit_pairs
            .iter()
            .map(|(_, c)| c)
            .cloned()
            .collect::<String>()
            .parse::<i32>()?;

        numbers.push(Number {
            num: num,
            row: row,
            start_col: start_col as i32,
            end_col: end_col as i32,
        });
    }

    Ok(numbers)
}

fn parse_symbols(row: i32, line: &str) -> Result<Vec<Symbol>, Box<dyn Error>> {
    let mut symbols = vec![];

    let symbol_chars = line
        .match_indices(|c| !char::is_numeric(c) && c != '.')
        .collect::<Vec<_>>();

    for (i, s) in symbol_chars {
        symbols.push(Symbol {
            symb: s.to_string(),
            row: row,
            col: i as i32,
        });
    }

    Ok(symbols)
}

fn is_part_number(number: &Number, symbols: &Vec<Symbol>) -> bool {
    symbols.iter().any(|symbol| are_adjacent(number, symbol))
}

fn gear_ratio(symbol: &Symbol, numbers: &Vec<Number>) -> Option<i32> {
    let adjacent_numbers = numbers
        .iter()
        .filter(|number| are_adjacent(number, symbol))
        .collect::<Vec<&Number>>();

    if symbol.symb == "*" && adjacent_numbers.len() == 2 {
        Some(
            adjacent_numbers
                .iter()
                .map(|n| n.num)
                .fold(1, |acc, n| acc * n),
        )
    } else {
        None
    }
}

fn are_adjacent(number: &Number, symbol: &Symbol) -> bool {
    let row_low = number.row - 1;
    let row_high = number.row + 1;
    let col_low = number.start_col - 1;
    let col_high = number.end_col + 1;

    for row in row_low..=row_high {
        for col in col_low..=col_high {
            if symbol.row == row && symbol.col == col {
                return true;
            }
        }
    }

    return false;
}
