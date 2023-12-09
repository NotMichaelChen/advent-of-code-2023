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
    let chunks: Vec<&[String]> = input.split(|s| s.is_empty()).collect();

    let (raw_seeds, raw_maps) = chunks.split_first().ok_or("Expected non-empty input")?;

    let seeds = parse_seeds(&raw_seeds[0])?;
    let mut raw_entries: Vec<Vec<Entry>> = raw_maps
        .iter()
        .map(|chunk| parse_map(&chunk.to_vec()))
        .collect::<Result<Vec<_>, _>>()?;

    for entries in &mut raw_entries {
        entries.sort_by_key(|entry| entry.source);
    }

    let maps: Vec<Map> = raw_entries
        .iter()
        .map(|entries| Map {
            entries_by_source: entries.to_vec(),
        })
        .collect();

    let mut values = seeds;
    for map in maps {
        let new_values = values
            .iter()
            .map(|value| apply_map(*value, &map))
            .collect::<Result<Vec<_>, _>>()?;
        values = new_values;
    }

    let result = values
        .iter()
        .min()
        .ok_or("Expected values to be nonempty")?;

    println!("part 1: {}", result);

    Ok(())
}

fn part2(input: &Vec<String>) -> GenResult<()> {
    let chunks: Vec<&[String]> = input.split(|s| s.is_empty()).collect();

    let (raw_seeds, raw_maps) = chunks.split_first().ok_or("Expected non-empty input")?;

    let seeds = parse_seed_ranges(&raw_seeds[0])?;
    let transforms: Vec<Vec<Transform>> = raw_maps
        .iter()
        .map(|chunk| parse_transforms(&chunk.to_vec()))
        .collect::<Result<Vec<_>, _>>()?;

    let final_seeds = apply_maps(&seeds, &transforms);

    let result = final_seeds
        .iter()
        .map(|r| r.start)
        .min()
        .ok_or("Expected final_seeds to be nonempty")?;

    println!("part 2: {}", result);

    Ok(())
}

#[derive(Debug)]
struct Map {
    entries_by_source: Vec<Entry>,
}

#[derive(Debug, Clone)]
struct Entry {
    source: i64,
    target: i64,
    len: i64,
}

#[derive(Copy, Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Transform {
    start: i64,
    end: i64,
    amount: i64,
}

fn parse_seeds(line: &String) -> GenResult<Vec<i64>> {
    let (_, nums) = line
        .split_once(':')
        .ok_or(format!("Could not find ':' in line: {}", line))?;

    Ok(nums
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn parse_seed_ranges(line: &String) -> GenResult<Vec<Range>> {
    let parsed_nums = parse_seeds(line)?;

    Ok(parsed_nums
        .chunks(2)
        .map(|c| {
            c.get(0)
                .zip(c.get(1))
                .ok_or(format!("Chunk was not size 2: {:?}", c))
                .map(|(s, e)| Range {
                    start: *s,
                    end: *s + *e - 1,
                })
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn parse_map(raw_map: &Vec<String>) -> GenResult<Vec<Entry>> {
    let (_, entries) = raw_map
        .split_first()
        .ok_or("Expected non-empty vec in raw_map")?;

    let mut parsed: Vec<Entry> = vec![];
    for entry in entries.iter() {
        let nums = entry
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        let entry_fields = match (nums.get(0), nums.get(1), nums.get(2)) {
            (Some(target), Some(source), Some(len)) => Ok((target, source, len)),
            _ => Err(format!("Entry did not contain 3 elements: {}", entry)),
        }?;

        parsed.push(Entry {
            source: *entry_fields.1,
            target: *entry_fields.0,
            len: *entry_fields.2,
        });
    }

    Ok(parsed)
}

fn parse_transforms(raw_map: &Vec<String>) -> GenResult<Vec<Transform>> {
    let (_, entries) = raw_map
        .split_first()
        .ok_or("Expected non-empty vec in raw_map")?;

    let mut parsed: Vec<Transform> = vec![];
    for entry in entries.iter() {
        let nums = entry
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        let entry_fields = match (nums.get(0), nums.get(1), nums.get(2)) {
            (Some(target), Some(source), Some(len)) => {
                let start = *source;
                let end = source + len - 1;
                let amount = target - source;
                Ok((start, end, amount))
            }
            _ => Err(format!("Entry did not contain 3 elements: {}", entry)),
        }?;

        parsed.push(Transform {
            start: entry_fields.0,
            end: entry_fields.1,
            amount: entry_fields.2,
        });
    }

    Ok(parsed)
}

fn apply_map(input: i64, map: &Map) -> GenResult<i64> {
    let partition_index = map
        .entries_by_source
        .partition_point(|entry| entry.source <= input);
    if partition_index == 0 {
        return Ok(input);
    }

    let index = partition_index - 1;

    let entry = map.entries_by_source.get(index).ok_or(format!(
        "partitioned index OOB, index={} len={}",
        index,
        map.entries_by_source.len()
    ))?;

    if input < entry.source + entry.len {
        let dist = input - entry.source;
        Ok(entry.target + dist)
    } else {
        Ok(input)
    }
}

fn apply_maps(seeds: &Vec<Range>, maps: &Vec<Vec<Transform>>) -> Vec<Range> {
    maps.iter().fold(seeds.to_vec(), |ranges, transforms| {
        process_ranges(&ranges, transforms)
    })
}

fn process_ranges(ranges: &Vec<Range>, transforms: &Vec<Transform>) -> Vec<Range> {
    ranges
        .iter()
        .map(|range| apply_transforms(range, transforms))
        .flatten()
        .collect()
}

fn apply_transforms(range: &Range, transforms: &Vec<Transform>) -> Vec<Range> {
    let new_ranges: Vec<(Range, i64)> = transforms
        .iter()
        .filter_map(|t| apply_transform(range, t))
        .collect();

    let mut original_splits = split_original_range(range, &new_ranges);
    let mut transformed_ranges: Vec<Range> = new_ranges
        .iter()
        .map(|(r, amount)| Range {
            start: r.start + amount,
            end: r.end + amount,
        })
        .collect();

    original_splits.append(&mut transformed_ranges);
    original_splits
}

fn split_original_range(range: &Range, affected_ranges: &Vec<(Range, i64)>) -> Vec<Range> {
    if affected_ranges.is_empty() {
        vec![range.clone()]
    } else {
        let dummy_first_range = Range {
            start: range.start - 1,
            end: range.start - 1,
        };
        let dummy_last_range = Range {
            start: range.end + 1,
            end: range.end + 1,
        };

        let sorted_ranges: &mut Vec<Range> =
            &mut affected_ranges.iter().map(|(r, _)| r).cloned().collect();
        sorted_ranges.sort_by_key(|r| r.start);

        let mut all_ranges = vec![];
        all_ranges.push(dummy_first_range);
        all_ranges.append(sorted_ranges);
        all_ranges.push(dummy_last_range);

        let mut original_splits: Vec<Range> = vec![];
        for (lower, upper) in all_ranges.windows(2).map(|pair| (pair[0], pair[1])) {
            if upper.start - lower.end > 1 {
                original_splits.push(Range {
                    start: lower.end + 1,
                    end: upper.start - 1,
                });
            }
        }

        original_splits
    }
}

fn apply_transform(range: &Range, transform: &Transform) -> Option<(Range, i64)> {
    let overlap = find_overlap((range.start, range.end), (transform.start, transform.end));

    overlap.map(|(start, end)| {
        (
            Range {
                start: start,
                end: end,
            },
            transform.amount,
        )
    })
}

fn find_overlap(left: (i64, i64), right: (i64, i64)) -> Option<(i64, i64)> {
    let (lower, upper) = if left.0 < right.0 {
        (left, right)
    } else {
        (right, left)
    };

    let (_, lower_high) = lower;
    let (upper_low, upper_high) = upper;

    if lower_high < upper_low {
        None
    } else if lower_high >= upper_high {
        Some(upper)
    } else {
        Some((upper_low, lower_high))
    }
}
