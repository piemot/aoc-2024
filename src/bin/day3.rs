use aoc_2024::*;
use logos::Logos;
use regex::Regex;

const MUL_EXPR: &str = r"mul\((?<v1>\d{1,3}),(?<v2>\d{1,3})\)";

// This realistically didn't need Logos, but it was fun to play around with!
#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("do()")]
    ResumeParsing,

    #[token("don't()")]
    StopParsing,

    #[regex(r"mul\((?<v1>\d{1,3}),(?<v2>\d{1,3})\)")]
    Multiply,
}

day!(part1, part2);

fn part1(input: &'static str) -> u32 {
    let re = Regex::new(MUL_EXPR).unwrap();

    re.captures_iter(input)
        .map(|c| (c.name("v1").unwrap(), c.name("v2").unwrap()))
        .map(|(v1, v2)| (v1.as_str().parse().unwrap(), v2.as_str().parse().unwrap()))
        .map(|(v1, v2): (u32, u32)| v1 * v2)
        .sum()
}

fn part2(input: &'static str) -> u32 {
    let re = Regex::new(MUL_EXPR).unwrap();
    let mut lex = Token::lexer(input);
    let mut can_run = true;
    let mut count: u32 = 0;

    // while let Some(Ok(val)) = lex.next() { // this oneliner doesn't work for some reason ugh
    while let Some(val) = lex.next() {
        // ignore parsing errors between tokens
        if let Ok(val) = val {
            match val {
                Token::ResumeParsing => {
                    can_run = true;
                }
                Token::StopParsing => {
                    can_run = false;
                }
                Token::Multiply => {
                    if can_run {
                        let captures = re.captures(lex.slice()).unwrap();
                        let (v1, v2) = (captures.name("v1").unwrap(), captures.name("v2").unwrap());
                        let (v1, v2): (u32, u32) =
                            (v1.as_str().parse().unwrap(), v2.as_str().parse().unwrap());
                        count += v1 * v2;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE_INPUT: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INPUT_2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    test_day!(test_part1 -> part1(SAMPLE_INPUT), 161);
    test_day!(test_part2 -> part2(SAMPLE_INPUT_2), 48);
}
