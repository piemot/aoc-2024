use aoc_2024::*;

day!(part1, part2);

fn part1(input: &'static str) -> u32 {
    let mut lines = input.lines();
    let mut pairs: Vec<(u32, u32)> = vec![];
    let mut sets: Vec<Vec<u32>> = vec![];
    let mut results = vec![];

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let chars: Vec<&str> = line.split('|').collect();
        pairs.push((chars[0].parse().unwrap(), chars[1].parse().unwrap()))
    }
    while let Some(line) = lines.next() {
        sets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }

    for set in sets {
        if correctly_ordered(&set, &pairs) {
            results.push(set[set.len() / 2]);
        }
    }

    results.into_iter().sum()
}

fn correctly_ordered(set: &Vec<u32>, pairs: &Vec<(u32, u32)>) -> bool {
    for (ind, first_number) in set.iter().enumerate() {
        for ind2 in ind..set.len() {
            let second_number = set[ind2];
            if pairs
                .iter()
                .filter(|p| p.0 == second_number && p.1 == *first_number)
                .count()
                > 0
            {
                return false;
            }
        }
    }
    true
}

fn part2(input: &'static str) -> u32 {
    let mut lines = input.lines();
    let mut pairs: Vec<(u32, u32)> = vec![];
    let mut sets: Vec<Vec<u32>> = vec![];
    let mut results: Vec<u32> = vec![];

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let chars: Vec<&str> = line.split('|').collect();
        pairs.push((chars[0].parse().unwrap(), chars[1].parse().unwrap()))
    }
    while let Some(line) = lines.next() {
        sets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }

    for mut set in sets {
        if correctly_ordered(&set, &pairs) {
            // only concerned about incorrectly ordered.
            // Potential place for optimisation: we iterate
            // the Vec twice because of this check
            continue;
        }

        // Essentially a modified bubble sort :3
        // Tried testing all permutations but some lines have 20+ elements -> on the order of 10^18 permutations
        let mut swapped = true;
        let mut last_sort = set.len();
        while swapped {
            swapped = false;
            for i in 1..last_sort {
                if pairs
                    .iter()
                    .filter(|p| p.0 == set[i] && p.1 == set[i - 1])
                    .count()
                    > 0
                {
                    set.swap(i - 1, i);
                    swapped = true;
                }
            }
            last_sort -= 1;
        }
        results.push(set[set.len() / 2]);
    }

    results.into_iter().sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 143);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 123);
}
