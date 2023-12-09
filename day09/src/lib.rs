pub fn solve_day09_part1(input: String) -> Result<String, String> {
    let sum = input.lines().map(predict_next).sum::<isize>();
    Ok(sum.to_string())
}

pub fn solve_day09_part2(input: String) -> Result<String, String> {
    let sum = input.lines().map(extrapolate_previous).sum::<isize>();
    Ok(sum.to_string())
}

fn predict_next(input: &str) -> isize {
    let observations = parse_observations(input);
    let stack = diff_stack(&observations);
    let last_diffs = stack.iter().map(|v| *v.last().unwrap()).collect::<Vec<isize>>();
    let last_observation = *observations.last().unwrap();
    let prediction = last_diffs.iter().fold(last_observation, |acc, v| acc + v);
    return prediction;
}

fn extrapolate_previous(input: &str) -> isize {
    let observations = parse_observations(input);
    let stack = diff_stack(&observations);
    let mut first_values = stack
        .iter()
        .map(|v| *v.first().unwrap())
        .rev()
        .collect::<Vec<isize>>();
    first_values.push(*observations.first().unwrap());
    let extrapolation = first_values.iter().fold(0, |acc, v| v - acc);
    return extrapolation;
}

fn parse_observations(input: &str) -> Vec<isize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn diff_stack(observations: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut stack: Vec<Vec<_>> = vec![];
    let mut diffs = observations.clone();
    loop {
        diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<_>>();
        stack.push(diffs.clone());
        if diffs.is_empty() { panic!("no zero diff in stack"); }
        if diffs.iter().all(|&diff| diff == 0) { break; }
    }
    stack
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn predicting_example_lines() -> Result<(), String> {
        assert_eq!(predict_next("0 3 6 9 12 15"), 18);
        assert_eq!(predict_next("1 3 6 10 15 21"), 28);
        assert_eq!(predict_next("10 13 16 21 30 45"), 68);
        Ok(())
    }

    #[test]
    fn solve_day09_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day09_part1(input)?, "114");
        Ok(())
    }

    #[test]
    fn solve_day09_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day09_part1(input)?, "1898776583");
        Ok(())
    }

    #[test]
    fn extrapolating_example_lines() -> Result<(), String> {
        assert_eq!(extrapolate_previous("0 3 6 9 12 15"), -3);
        assert_eq!(extrapolate_previous("1 3 6 10 15 21"), 0);
        assert_eq!(extrapolate_previous("10 13 16 21 30 45"), 5);
        Ok(())
    }

    #[test]
    fn solve_day09_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day09_part2(input)?, "2");
        Ok(())
    }

    #[test]
    fn solve_day09_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day09_part2(input)?, "1100");
        Ok(())
    }
}
