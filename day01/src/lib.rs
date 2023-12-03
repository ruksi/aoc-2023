use std::collections::HashMap;
use regex::{Captures, Regex};

pub fn solve_day01_part1(input: String) -> Result<String, String> {
    let result: u32 = input
        .split("\n")
        .map(|line| {
            let Some(left) = line.chars().find(|c| c.to_digit(10).is_some()) else {
                panic!("No digit found on line: {}", line)
            };
            let right = line
                .chars()
                .rfind(|c| c.to_digit(10).is_some())
                .expect("but we found one searching from the left!?");
            let as_string = vec![left, right].into_iter().collect::<String>();
            return as_string.parse::<u32>().expect("but separate characters were digits!?");
        })
        .sum();
    Ok(result.to_string())
}

pub fn solve_day01_part2(input: String) -> Result<String, String> {
    let replacements: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    // go through each line, potentially replacing _the first_ word with its digit
    let word_group = replacements
        .keys()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
        .join("|");
    let re_from_left = Regex::new(word_group.as_str()).map_err(|e| e.to_string())?;
    let lefts = input
        .split("\n")
        .map(|line| {
            re_from_left.replace(line, |caps: &Captures| {
                format!("{}", replacements
                    .get(&caps[0])
                    .expect("but the match group was made from the replacement keys!?")
                )
            }).to_string()
        })
        .collect::<Vec<String>>();

    // go through each line, potentially replacing _the last_ word with its digit
    // this is wanted because a line can have something like "eightwo" which could be
    // replaced with "8wo" or "eight2" depending on from which end we start searching
    // as we only want the last match, we reverse both the needle (word) and the haystack (line)
    let reversed_word_group = word_group.chars().rev().collect::<String>();  // word flip ↩️
    let re_from_right = Regex::new(reversed_word_group.as_str()).map_err(|e| e.to_string())?;
    let rights = input
        .split("\n")
        .map(|line| {
            let line = line.chars().rev().collect::<String>(); // line flip ↩️
            re_from_right.replace(
                &line,
                |caps: &Captures| {
                    let lookup_key = &caps[0].chars().rev().collect::<String>(); // word flop ↪️
                    format!("{}", replacements
                        .get(lookup_key.as_str())
                        .expect("but the match group was made from the replacement keys!?")
                    )
                },
            ).chars().rev().collect::<String>() // line flop ↪️
        })
        .collect::<Vec<String>>();

    // zip-combine the two lists, so we can read
    // the first digit from the lefts
    // and the last digit from the rights
    let zipped = lefts.iter().zip(rights.iter());
    let result: u32 = zipped.map(|(left, right)| {
        let Some(left) = left.chars().find(|c| c.to_digit(10).is_some()) else {
            panic!("No digit found on left-digit-ed line: {}", left)
        };
        let Some(right) = right.chars().rfind(|c| c.to_digit(10).is_some()) else {
            panic!("No digit found on right-digit-ed line: {}", right)
        };
        let as_string = vec![left, right].into_iter().collect::<String>();
        return as_string.parse::<u32>().expect("but separate characters were digits!?");
    }).sum();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day01_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/part1-example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day01_part1(input)?, "142");
        Ok(())
    }

    #[test]
    fn solve_day01_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day01_part1(input)?, "56397");
        Ok(())
    }

    #[test]
    fn solve_day01_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/part2-example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day01_part2(input)?, "281");
        Ok(())
    }

    #[test]
    fn solve_day01_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day01_part2(input)?, "55701");
        Ok(())
    }
}
