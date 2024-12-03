use aoc_2024::*;

day!(part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportStatus {
    Invalid,
    Valid,
}

impl ReportStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, ReportStatus::Valid)
    }
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        if value > 0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

fn part1(input: &'static str) -> usize {
    let mut vec = Vec::with_capacity(10);
    let valid_reports = input.lines().filter(|line| {
        let values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
        vec.clear();
        vec.extend(values);

        check_report(&vec).is_valid()
    });

    valid_reports.count()
}

fn check_report(report: &Vec<i32>) -> ReportStatus {
    let direction: Direction = (report[0] - report[1]).into();
    let mut last_value = None;
    for value in report {
        if let Some(last) = last_value {
            let diff: i32 = last - value;
            if diff.abs() > 3 || diff == 0 || Direction::from(last - *value) != direction {
                return ReportStatus::Invalid;
            }
        }
        last_value = Some(*value);
    }
    ReportStatus::Valid
}

fn part2(input: &'static str) -> usize {
    let mut vec = Vec::with_capacity(10);
    let valid_reports = input.lines().filter(|line| {
        let values = line.split_whitespace().map(|v| v.parse::<i32>().unwrap());
        vec.clear();
        vec.extend(values);

        if check_report(&vec).is_valid() {
            return true;
        }

        (0..vec.len()).any(|index| {
            let mut vec2 = vec.clone();
            vec2.remove(index);
            check_report(&vec2).is_valid()
        })
    });

    valid_reports.count()
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 2);
    test_day!(test_part2 -> part2(SAMPLE_INPUT), 4);
}
