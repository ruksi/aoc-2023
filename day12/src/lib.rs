use itertools::Itertools;

pub fn solve_day12_part1(input: String) -> Result<String, String> {
    let rows = input
        .lines()
        .map(|line| SpringRow::from(line))
        .collect::<Result<Vec<_>, String>>()?;
    let valid_arrangement_sum = rows
        .iter()
        .map(|row| row.valid_permutations().unwrap().len())
        .sum::<usize>();
    Ok(valid_arrangement_sum.to_string())
}

pub fn solve_day12_part2(input: String) -> Result<String, String> {
    Ok(input)
}

#[derive(Debug)]
struct SpringRow {
    springs: String,
    damages: Vec<usize>,
}

impl SpringRow {
    fn from(input: &str) -> Result<Self, String> {
        let mut parts = input.split_whitespace();
        let springs = parts
            .next()
            .ok_or(format!("Row is missing springs: {input:?}"))?
            .to_string();
        let damages = parts
            .next()
            .ok_or(format!("Row is missing damaged groups: {input:?}"))?
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Self { springs, damages })
    }

    fn unknown_count(&self) -> usize {
        self.springs.chars().filter(|c| *c == '?').count()
    }

    fn is_valid_with(&self, fills: &String) -> Result<bool, String> {
        let fills = fills.chars().collect::<Vec<_>>();
        if fills.len() != self.unknown_count() {
            return Err(format!("Invalid fill count for the spring row: {fills:?}"));
        }

        let mut fills_iter = fills.into_iter();
        let filled_springs = self.springs.clone();
        let filled_springs = filled_springs
            .chars()
            .map(|c| match c {
                '?' => fills_iter.next().unwrap(),
                _ => c,
            })
            .collect::<String>();

        let filled_damages = filled_springs
            .split('.')
            .map(|group| group.len())
            .filter(|len| *len > 0)
            .collect::<Vec<_>>();

        Ok(filled_damages == self.damages)
    }

    fn all_permutations(&self) -> Result<Vec<String>, String> {
        let permutations = (0..self.unknown_count())
            .map(|_| vec!['.', '#'])
            .multi_cartesian_product()
            .map(|permutation| permutation.into_iter().collect::<String>())
            .collect::<Vec<_>>();
        Ok(permutations)
    }

    fn valid_permutations(&self) -> Result<Vec<String>, String> {
        // #bruteforcegang 💪
        //
        // as it took ~30 seconds to solve the part 1,
        // I'm not even going to try to solve the part 2 for now
        //
        // to solve the part 2, this needs to be optimized
        // probably something like... filter validity after each character, caching sub-results,
        // other dynamic programming trickery, reducing the search space, etc.
        let permutations = self.all_permutations()?;
        let valid_permutations = permutations
            .into_iter()
            .filter(|perm| self.is_valid_with(&perm).unwrap())
            .collect::<Vec<_>>();
        Ok(valid_permutations)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn validating_permutations() -> Result<(), String> {
        let row = SpringRow::from("???.### 1,1,3")?;
        assert_eq!(row.is_valid_with(&"#.#".into())?, true);
        assert_eq!(row.is_valid_with(&"##.".into())?, false);
        assert_eq!(row.valid_permutations()?, vec!["#.#"]);

        let row = SpringRow::from(".??..??...?##. 1,1,3")?;
        assert_eq!(row.is_valid_with(&".#.#.".into())?, false);
        assert_eq!(row.is_valid_with(&".#.##".into())?, true);
        assert_eq!(row.valid_permutations()?.len(), 4);

        Ok(())
    }

    #[test]
    fn expansion_on_part2() -> Result<(), String> {
        // ???.### 1,1,3
        // => 5 copies =>
        // ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
        Ok(())
    }

    #[test]
    fn solve_day12_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day12_part1(input)?, "21");
        Ok(())
    }

    // #[test]
    // fn solve_day12_part1_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day12_part1(input)?, "6871");
    //     Ok(())
    // }

    // #[test]
    // fn solve_day12_part2_on_example() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day12_part2(input)?, "#yolo");
    //     Ok(())
    // }

    // #[test]
    // fn solve_day12_part2_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day12_part2(input)?, "#yolo");
    //     Ok(())
    // }
}
