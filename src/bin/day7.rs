use std::iter::repeat_n;

use aoc_2024::*;
use itertools::Itertools;

day!(part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn part1(input: &'static str) -> u64 {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let (result, values) = line.split_once(": ").unwrap();
        let result: u64 = result.parse().unwrap();
        let values: Vec<u64> = values.split(' ').map(|d| d.parse().unwrap()).collect();

        let op_len = values.len() - 1;
        println!("Running {} -> {:?} n={}", result, values, op_len);

        // let ops =
        //     .into_iter()
        //     .permutations(op_len);

        let ops = repeat_n([Operator::Add, Operator::Multiply].into_iter(), op_len)
            .multi_cartesian_product();

        for operation_set in ops {
            let mut operation_set = operation_set.iter();
            let mut values = values.iter();

            let mut acc = *values.next().unwrap();
            for value in values {
                match operation_set.next().unwrap() {
                    Operator::Add => acc = acc.saturating_add(*value),
                    Operator::Multiply => acc = acc.saturating_mul(*value),
                    Operator::Concat => unreachable!("Not considererd in `ops`"),
                }
            }

            if acc == result {
                total += result;
                break;
            }
        }
    }
    total
}

fn part2(input: &'static str) -> u64 {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let (result, values) = line.split_once(": ").unwrap();
        let result: u64 = result.parse().unwrap();
        let values: Vec<u64> = values.split(' ').map(|d| d.parse().unwrap()).collect();

        let op_len = values.len() - 1;

        let ops = repeat_n(
            [Operator::Add, Operator::Multiply, Operator::Concat].into_iter(),
            op_len,
        )
        .multi_cartesian_product();

        for operation_set in ops {
            let mut operation_set = operation_set.iter();
            let mut values = values.iter();
            let mut acc = *values.next().unwrap();
            for value in values {
                match operation_set.next().unwrap() {
                    // FIXME: Saturation is technically not *correct*.
                    Operator::Add => acc = acc.saturating_add(*value),
                    Operator::Multiply => acc = acc.saturating_mul(*value),
                    Operator::Concat => {
                        let val = acc.to_string() + &((*value).to_string());
                        acc = val.parse().unwrap_or(u64::MAX);
                    }
                }
            }

            if acc == result {
                total += result;
                break;
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 3749);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 11387);
}
