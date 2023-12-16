use std::collections::HashSet;
use std::fmt;

pub fn solve_day14_part1(input: String) -> Result<String, String> {
    let mut platform = Platform::from(input.as_str());
    platform.tilt_north();
    let load = calculate_load(&platform.round_rocks, platform.height);
    Ok(load.to_string())
}

pub fn solve_day14_part2(input: String) -> Result<String, String> {
    let mut platform = Platform::from(input.as_str());
    let load = platform.load_after_cycles(1_000_000_000);
    match load {
        Some(load) => Ok(load.to_string()),
        None => Err("No cycles found".to_string()),
    }
}

type Point = (isize, isize); // (x, y)

#[derive(Debug)]
struct Platform {
    round_rocks: HashSet<Point>,
    square_rocks: HashSet<Point>,
    width: isize,
    height: isize,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for y in 0..self.height {
            if y > 0 { output.push('\n'); }
            for x in 0..self.width {
                let symbol = if self.round_rocks.contains(&(x, y)) {
                    'O'
                } else if self.square_rocks.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                output.push(symbol);
            }
        }
        write!(f, "{}", output)
    }
}

impl Platform {
    fn from(input: &str) -> Self {
        let mut round_rocks = HashSet::new();
        let mut square_rocks = HashSet::new();
        let mut width = 0;
        let height = input.lines().collect::<Vec<_>>().len() as isize;
        for (y, line) in input.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                if symbol == 'O' {
                    round_rocks.insert((x as isize, y as isize));
                }
                if symbol == '#' {
                    square_rocks.insert((x as isize, y as isize));
                }
                if width == 0 { width = line.len() as isize; }
            }
        }
        Platform { round_rocks, square_rocks, width, height }
    }

    fn load_after_cycles(&mut self, cycles: usize) -> Option<isize> {
        let mut loads = vec![];
        for curr_cycle in 0..cycles {
            self.cycle();
            loads.insert(0, calculate_load(&self.round_rocks, self.height));
            let pattern_len = detect_pattern(&loads);
            if let Some(pattern_len) = pattern_len {
                // fast-forward ⏩️
                let reminder = (cycles - curr_cycle) % pattern_len;
                let x = loads[(loads.len() - 1) - reminder];
                return Some(x);
            }
        }
        None
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        for i in 0..self.height {
            self.round_rocks = self.round_rocks
                .clone()
                .into_iter()
                .map(|(x, y)| {
                    if y == i {
                        for py in (0..y).rev() {
                            if self.square_rocks.contains(&(x, py)) { return (x, py + 1); }
                            if self.round_rocks.contains(&(x, py)) { return (x, py + 1); }
                        }
                        (x, 0)
                    } else {
                        (x, y)
                    }
                }).collect::<HashSet<_>>();
        }
    }

    fn tilt_west(&mut self) {
        for i in 0..self.width {
            self.round_rocks = self.round_rocks
                .clone()
                .into_iter()
                .map(|(x, y)| {
                    if x == i {
                        for px in (0..x).rev() {
                            if self.square_rocks.contains(&(px, y)) { return (px + 1, y); }
                            if self.round_rocks.contains(&(px, y)) { return (px + 1, y); }
                        }
                        (0, y)
                    } else {
                        (x, y)
                    }
                }).collect::<HashSet<_>>();
        }
    }

    fn tilt_south(&mut self) {
        for i in (0..self.height).rev() {
            self.round_rocks = self.round_rocks
                .clone()
                .into_iter()
                .map(|(x, y)| {
                    if y == i {
                        for ny in (y + 1)..self.height {
                            if self.square_rocks.contains(&(x, ny)) { return (x, ny - 1); }
                            if self.round_rocks.contains(&(x, ny)) { return (x, ny - 1); }
                        }
                        (x, self.height - 1)
                    } else {
                        (x, y)
                    }
                }).collect::<HashSet<_>>();
        }
    }

    fn tilt_east(&mut self) {
        for i in (0..self.width).rev() {
            self.round_rocks = self.round_rocks
                .clone()
                .into_iter()
                .map(|(x, y)| {
                    if x == i {
                        for nx in (x + 1)..self.width {
                            if self.square_rocks.contains(&(nx, y)) { return (nx - 1, y); }
                            if self.round_rocks.contains(&(nx, y)) { return (nx - 1, y); }
                        }
                        (self.width - 1, y)
                    } else {
                        (x, y)
                    }
                }).collect::<HashSet<_>>();
        }
    }
}

fn detect_pattern(seq: &Vec<isize>) -> Option<usize> {
    let pattern_max_len = seq.len() / 2;
    for x in 2..pattern_max_len {
        if seq[0..x] == seq[x..2 * x] {
            return Some(x);
        }
    }
    None
}

fn calculate_load(round_rocks: &HashSet<Point>, height: isize) -> isize {
    round_rocks
        .iter()
        .map(|(_, y)| height - y)
        .sum::<isize>()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parsing_platform() -> Result<(), String> {
        let mut platform = Platform::from("O..\n.#.\n..O\nOO.");
        assert_eq!(platform.round_rocks.len(), 4);
        assert_eq!(platform.square_rocks.len(), 1);
        assert_eq!(platform.width, 3);
        assert_eq!(platform.height, 4);
        assert_eq!(format!("{}", platform), "O..\n.#.\n..O\nOO.".to_string());

        platform.tilt_north();
        assert_eq!(format!("{}", platform), "O.O\nO#.\n.O.\n...".to_string());
        assert_eq!(calculate_load(&platform.round_rocks, platform.height), 13);

        platform.cycle();
        assert_eq!(format!("{}", platform), "..O\nO#.\n..O\n..O".to_string());
        Ok(())
    }

    #[test]
    fn solve_day14_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day14_part1(input)?, "136");
        Ok(())
    }

    #[test]
    fn solve_day14_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day14_part1(input)?, "108826");
        Ok(())
    }

    #[test]
    fn solve_day14_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day14_part2(input)?, "64");
        Ok(())
    }

    // #[test]
    // fn solve_day14_part2_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day14_part2(input)?, "#yolo");
    //     // 100106, 99704 and 99318 are too high...
    //     // the example works, but my input doesn't, so there probably is a bug somewhere 🤷
    //     Ok(())
    // }
}
