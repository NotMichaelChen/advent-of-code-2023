use std::cmp::Ordering;
use std::collections::HashMap;
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
    let card_map = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    let mut hands = input
        .iter()
        .map(|line| parse_hand(line, &card_map, false))
        .collect::<Result<Vec<_>, _>>()?;

    let result = score_hands(&mut hands);

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let card_map = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    let mut hands = input
        .iter()
        .map(|line| parse_hand(line, &card_map, true))
        .collect::<Result<Vec<_>, _>>()?;

    let result = score_hands(&mut hands);

    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug)]
struct Hand {
    card_values: Vec<i32>,
    hand_type: i32,
    bid: i32,
}

fn score_hands(hands: &mut Vec<Hand>) -> i32 {
    hands.sort_by(|l, r| {
        let res = l.hand_type.cmp(&r.hand_type);
        if res == Ordering::Equal {
            break_tie(&l.card_values, &r.card_values)
        } else {
            res
        }
    });

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as i32 * hand.bid)
        .sum()
}

fn parse_hand(line: &String, card_map: &HashMap<char, i32>, jokers: bool) -> GenResult<Hand> {
    let (cards, bid_str) = line
        .split_once(' ')
        .ok_or(format!("Invalid line: {}", line))?;

    let card_values = cards
        .chars()
        .map(|c| {
            card_map
                .get(&c)
                .map(|n| *n)
                .ok_or(format!("Got invalid card {}", c))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let bid = bid_str.parse::<i32>()?;
    let hand_type = if jokers {
        classify_hand_jokers(cards)
    } else {
        classify_hand(cards)
    };

    Ok(Hand {
        card_values: card_values,
        hand_type: hand_type,
        bid: bid,
    })
}

fn classify_hand(hand: &str) -> i32 {
    let mut card_counts = HashMap::new();

    for c in hand.chars() {
        let count = card_counts.get(&c).unwrap_or(&0);
        card_counts.insert(c, count + 1);
    }

    let mut freq_counts = HashMap::new();
    for count in card_counts.values() {
        let c = freq_counts.get(&count).unwrap_or(&0);
        freq_counts.insert(count, c + 1);
    }

    if freq_counts.contains_key(&5) {
        7
    } else if freq_counts.contains_key(&4) {
        6
    } else if freq_counts.contains_key(&3) && freq_counts.contains_key(&2) {
        5
    } else if freq_counts.contains_key(&3) {
        4
    } else if freq_counts.get(&2).is_some_and(|c| *c == 2) {
        3
    } else if freq_counts.contains_key(&2) {
        2
    } else {
        1
    }
}

fn classify_hand_jokers(hand: &str) -> i32 {
    let mut card_counts = HashMap::new();

    for c in hand.chars() {
        let count = card_counts.get(&c).unwrap_or(&0);
        card_counts.insert(c, count + 1);
    }

    let joker_counts = card_counts.remove(&'J').unwrap_or(0);

    let mut freq_counts = HashMap::new();
    for count in card_counts.values() {
        let c = freq_counts.get(&count).unwrap_or(&0);
        freq_counts.insert(count, c + 1);
    }

    let max_count = *freq_counts.keys().max().unwrap_or(&&0);

    if max_count + joker_counts == 5 {
        7
    } else if max_count + joker_counts == 4 {
        6
    } else if (*max_count == 3 && freq_counts.contains_key(&2))
        || (max_count + joker_counts == 3 && freq_counts.get(&2).map(|n| *n == 2).unwrap_or(false))
    {
        5
    } else if max_count + joker_counts == 3 {
        4
    } else if freq_counts.get(&2).is_some_and(|c| *c == 2) {
        3
    } else if max_count + joker_counts == 2 {
        2
    } else {
        1
    }
}

fn break_tie(left: &Vec<i32>, right: &Vec<i32>) -> Ordering {
    for (l, r) in zip(left, right) {
        if l < r {
            return Ordering::Less;
        } else if l > r {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}
