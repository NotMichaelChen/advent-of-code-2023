use std::collections::HashMap;
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
    let (instructions, network) = parse_input(input)?;
    let result = traverse_network(&instructions, &network, "AAA", |s| s == "ZZZ")?;

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let (instructions, network) = parse_input(input)?;
    let result = traverse_all_paths(&instructions, &network)?;

    println!("part 2: {}", result);

    Ok(())
}

struct Node {
    left: String,
    right: String,
}

fn parse_input(input: &Vec<String>) -> GenResult<(Vec<char>, HashMap<String, Node>)> {
    let (instructions_raw, nodes_raw) = input
        .split_first()
        .ok_or(format!("Invalid input, {:?}", input))?;

    let nodes = nodes_raw
        .split_first()
        .ok_or(format!("Invalid input, {:?}", input))?
        .1
        .iter()
        .map(|s| parse_node(s))
        .collect::<Result<HashMap<_, _>, _>>()?;

    let instructions = instructions_raw.trim().chars().collect();

    Ok((instructions, nodes))
}

fn parse_node(line: &str) -> GenResult<(String, Node)> {
    let (current_raw, node_str) = line
        .split_once('=')
        .ok_or(format!("Expected '=': {}", line))?;

    let (left_raw, right_raw) = node_str
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split_once(',')
        .ok_or(format!("Invalid node string: {}", node_str))?;

    Ok((
        current_raw.trim().to_string(),
        Node {
            left: left_raw.trim().to_string(),
            right: right_raw.trim().to_string(),
        },
    ))
}

fn traverse_all_paths(instructions: &Vec<char>, network: &HashMap<String, Node>) -> GenResult<i64> {
    let starting_nodes: Vec<&String> = network.keys().filter(|s| s.ends_with('A')).collect();

    let path_lengths = starting_nodes
        .iter()
        .map(|s| traverse_network(instructions, &network, s, |s| s.ends_with('Z')))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(path_lengths.iter().fold(1, |acc, n| lcm(acc, *n)))
}

fn traverse_network(
    instructions: &Vec<char>,
    network: &HashMap<String, Node>,
    start: &str,
    end_cond: impl Fn(&str) -> bool,
) -> GenResult<i64> {
    let mut counter = 0;
    let mut ip = 0;
    let mut current = start;

    while !end_cond(current) {
        let node = network
            .get(current)
            .ok_or(format!("Node did not exist in network: {:?}", current))?;

        current = if instructions[ip] == 'L' {
            &node.left
        } else {
            &node.right
        };

        counter += 1;
        ip += 1;
        if ip >= instructions.len() {
            ip = 0;
        }
    }

    Ok(counter)
}

// https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
fn lcm(a: i64, b: i64) -> i64 {
    a * (b / gcd(a, b))
}

// https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
