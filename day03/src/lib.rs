use regex::Regex;

pub fn solve_day03_part1(input: String) -> Result<String, String> {
    let schematic = Schematic::new(&input);
    Ok(schematic.part_numbers.iter().map(|pn| pn.value).sum::<usize>().to_string())
}

struct Schematic {
    grid: Vec<Vec<String>>,
    part_numbers: Vec<PartNumber>,
}

impl Schematic {
    fn new(input: &String) -> Self {
        let grid = input
            .lines()
            .map(|row| row.chars().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // we do the part number finding here because we still have schematic rows
        // as strings, and we don't need them as strings after that
        let symbol_re = Regex::new(r"[^\d\.]").expect("Bad red regex");
        let number_re = Regex::new(r"\d+").expect("Bad number regex");
        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(row_index, row)| {
                let row_part_numbers = number_re
                    .captures_iter(row)
                    .filter_map(|num_capture| {
                        let Some(num_match) = num_capture.get(0) else { return None; };
                        let num_start = num_match.start();
                        let num_end = num_match.end() - 1;
                        let is_part_number = adjacents(row_index, num_start, num_end)
                            .into_iter()
                            .any(|adj| {
                                match grid.get(adj.0).and_then(|row| row.get(adj.1)) {
                                    Some(cell) => symbol_re.is_match(cell),
                                    _ => false,
                                }
                            });
                        match is_part_number {
                            true => {
                                let number = num_match
                                    .as_str()
                                    .parse::<usize>()
                                    .expect("but we matched \\d!?");
                                Some(PartNumber {
                                    value: number,
                                    row: row_index,
                                    start: num_start,
                                    end: num_end,
                                })
                            }
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>();
                row_part_numbers
            }).collect::<Vec<_>>();

        Self { grid, part_numbers }
    }

    fn asterisks(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(col_index, cell)| {
                        match cell.as_str() {
                            "*" => Some((row_index, col_index)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

struct PartNumber {
    value: usize,
    row: usize,
    start: usize,
    end: usize,
}

impl PartNumber {
    fn is_adjacent_to(&self, position: &(usize, usize)) -> bool {
        adjacents(self.row, self.start, self.end).contains(position)
    }
}

fn adjacents(row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    // previous row
    if row > 0 {
        if start > 0 { positions.push((row - 1, start - 1)); }
        for i in start..=end + 1 { positions.push((row - 1, i)); }
    }

    // this row
    if start > 0 { positions.push((row, start - 1)); }
    positions.push((row, end + 1));

    // next row
    if start > 0 { positions.push((row + 1, start - 1)); }
    for i in start..=end + 1 { positions.push((row + 1, i)); }

    positions
}

pub fn solve_day03_part2(input: String) -> Result<String, String> {
    let schematic = Schematic::new(&input);
    let sum_of_gear_ratios = schematic
        .asterisks()
        .iter()
        .map(|asterisk| {
            let adj_part_numbers = schematic.part_numbers
                .iter()
                .filter(|part_number| part_number.is_adjacent_to(asterisk))
                .collect::<Vec<_>>();
            if adj_part_numbers.len() != 2 { return 0; } // this asterisk is not "gear" ❌⚙️
            adj_part_numbers
                .iter()
                .map(|part_number| part_number.value)
                .product::<usize>()
        })
        .sum::<usize>();
    Ok(sum_of_gear_ratios.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn adjacent_works() -> Result<(), String> {
        assert_eq!(adjacents(0, 0, 3), vec![
            // the previous row is out of grid
            (0, 4), // the current row is half-out of grid
            (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), // the next row has one position out of grid
        ]);
        assert_eq!(adjacents(2, 3, 4), vec![
            (1, 2), (1, 3), (1, 4), (1, 5), // the previous row
            (2, 2), (2, 5),  // the current row
            (3, 2), (3, 3), (3, 4), (3, 5), // the next row
        ]);
        Ok(())
    }

    #[test]
    fn solve_day03_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day03_part1(input)?, "4361");
        Ok(())
    }

    #[test]
    fn solve_day03_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day03_part1(input)?, "530849");
        Ok(())
    }

    #[test]
    fn solve_day03_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day03_part2(input)?, "467835");
        Ok(())
    }

    #[test]
    fn solve_day03_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day03_part2(input)?, "84900879");
        Ok(())
    }
}
