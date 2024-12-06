use aoc_2024::*;
use rustc_hash::FxHashMap;

day!(part1, part2);

fn parse_input(input: &'static str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(str::split_whitespace)
        .map(|mut values| {
            let (Some(a), Some(b)) = (values.next(), values.next()) else {
                panic!("Invalid input")
            };
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .unzip()
}

fn part1(input: &'static str) -> u32 {
    let (mut list_a, mut list_b) = parse_input(input);

    list_a.sort_unstable();
    list_b.sort_unstable();

    list_a.iter().zip(list_b).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part2(input: &'static str) -> u32 {
    let (list_a, list_b) = parse_input(input);

    list_a
        .iter()
        .map(|a| a * list_b.iter().filter(|b| a == *b).count() as u32)
        .sum()
}

// An alternate implementation of part 2, optimised to be O(n)
// at the cost of the allocation of the FxHashMap.
#[allow(dead_code)]
fn part2_topt(input: &'static str) -> u32 {
    let (list_a, list_b) = parse_input(input);

    let mut map = FxHashMap::default();
    for b in list_b {
        *map.entry(b).or_insert(0) += 1;
    }

    list_a.iter().map(|a| a * map.get(a).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 11);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 31);
}
