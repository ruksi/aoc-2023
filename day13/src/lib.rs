use std::collections::HashMap;

pub fn solve_day13_part1(input: String) -> Result<String, String> {
    let result = input
        .split("\n\n")
        .map(|pattern| {
            let (column_mismatches, row_mismatches) = get_mirror_mismatches(pattern);
            let vertical_mirror = column_mismatches.iter().find(|(_, miss)| **miss == 0);
            if let Some((column_number, _)) = vertical_mirror {
                return *column_number;
            }
            let horizontal_mirror = row_mismatches.iter().find(|(_, miss)| **miss == 0);
            if let Some((row_number, _)) = horizontal_mirror {
                return *row_number * 100; // "add 100 multiplied by the number of rows"
            }
            0 // 🤷 no symmetry
        })
        .sum::<isize>();
    Ok(result.to_string())
}

pub fn solve_day13_part2(input: String) -> Result<String, String> {
    let result = input
        .split("\n\n")
        .map(|pattern| {
            let (column_mismatches, row_mismatches) = get_mirror_mismatches(pattern);
            let vertical_mirror = column_mismatches.iter().find(|(_, miss)| **miss == 1);
            if let Some((column_number, _)) = vertical_mirror {
                return *column_number;
            }
            let horizontal_mirror = row_mismatches.iter().find(|(_, miss)| **miss == 1);
            if let Some((row_number, _)) = horizontal_mirror {
                return *row_number * 100; // "add 100 multiplied by the number of rows"
            }
            0 // 🤷 no symmetry
        })
        .sum::<isize>();
    Ok(result.to_string())
}

// column number as described in the problem, 1-indexed
type ColumnNumber = isize;

// row number as described in the problem, 1-indexed
type RowNumber = isize;

// number of mismatches for the given column or row mirroring
type MismatchCount = isize;

fn get_mirror_mismatches(pattern: &str) -> (
    HashMap<ColumnNumber, MismatchCount>,
    HashMap<RowNumber, MismatchCount>,
) {
    let pattern = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_count = pattern.len() as isize;
    let column_count = pattern[0].len() as isize;

    // vertical symmetry
    let mut column_mismatches: HashMap<ColumnNumber, MismatchCount> = HashMap::new();
    // no need to check the _last_ column since it has no right neighbor
    for column in 0..(column_count - 1) {
        let mut mismatches: isize = 0;
        for scan_offset in 0..column_count {
            let left = column - scan_offset;
            let right = column + scan_offset + 1;
            if left < 0 { continue; } // left symbol out of bounds
            if left >= right { continue; }  // left symbol is right of right symbol
            if right >= column_count { continue; }  // right symbol out of bounds

            let left = left as usize;
            let right = right as usize;
            for row in 0..row_count {
                let row = row as usize;
                if pattern[row][left] != pattern[row][right] {
                    mismatches += 1;
                }
            }
        }
        // record the _column number_, not the index
        column_mismatches.insert(column + 1, mismatches);
    }

    // horizontal symmetry
    let mut row_mismatches: HashMap<RowNumber, MismatchCount> = HashMap::new();
    // no need to check the _last_ row since it has no bottom neighbor
    for row in 0..(row_count - 1) {
        let mut mismatches: isize = 0;
        for scan_offset in 0..row_count {
            let top = row - scan_offset;
            let bottom = row + scan_offset + 1;
            if top < 0 { continue; } // top symbol out of bounds
            if top >= bottom { continue; }  // top symbol is below bottom symbol
            if bottom >= row_count { continue; }  // bottom symbol out of bounds

            let top = top as usize;
            let bottom = bottom as usize;
            for column in 0..column_count {
                let column = column as usize;
                if pattern[top][column] != pattern[bottom][column] {
                    mismatches += 1;
                }
            }
        }
        // record the _row number_, not the index
        row_mismatches.insert(row + 1, mismatches);
    }

    (column_mismatches, row_mismatches)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day13_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day13_part1(input)?, "405");
        Ok(())
    }

    #[test]
    fn solve_day13_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day13_part1(input)?, "35691");
        Ok(())
    }

    #[test]
    fn solve_day13_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day13_part2(input)?, "400");
        Ok(())
    }

    #[test]
    fn solve_day13_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day13_part2(input)?, "39037");
        Ok(())
    }
}
