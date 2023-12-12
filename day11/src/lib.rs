use std::collections::HashSet;

use itertools::Itertools;

pub fn solve_day11_part1(input: String) -> Result<String, String> {
    let mut image = SpaceImage::from(input);
    image.light_travel_tick(1)?;
    Ok(image.galaxy_distance_sum()?.to_string())
}

pub fn solve_day11_part2(input: String) -> Result<String, String> {
    let mut image = SpaceImage::from(input);
    image.light_travel_tick(1000000 - 1)?;
    Ok(image.galaxy_distance_sum()?.to_string())
}

type Point = (isize, isize); // (x, y)

#[derive(Debug)]
struct SpaceImage {
    galaxies: HashSet<Point>,
}

impl SpaceImage {
    fn new(galaxies: HashSet<Point>) -> Self {
        SpaceImage { galaxies }
    }

    fn from(input: String) -> Self {
        let mut galaxies = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, column) in line.chars().enumerate() {
                if column == '#' {
                    galaxies.insert((x as isize, y as isize));
                }
            }
        }
        SpaceImage { galaxies }
    }

    fn last_column(&self) -> Result<isize, String> {
        self.galaxies.iter().map(|(x, _)| x).max().cloned().ok_or("No galaxies".to_string())
    }

    fn last_row(&self) -> Result<isize, String> {
        self.galaxies.iter().map(|(_, y)| y).max().cloned().ok_or("No galaxies".to_string())
    }

    fn void_rows(&self) -> Result<Vec<isize>, String> {
        let mut void_rows = Vec::new();
        for y in (0..=self.last_row()?).rev() {
            if !self.galaxies.iter().any(|(_, _y)| _y == &y) {
                void_rows.push(y);
            }
        }
        Ok(void_rows)
    }

    fn void_columns(&self) -> Result<Vec<isize>, String> {
        let mut void_columns = Vec::new();
        for x in (0..=self.last_column()?).rev() {
            if !self.galaxies.iter().any(|(_x, _)| _x == &x) {
                void_columns.push(x);
            }
        }
        Ok(void_columns)
    }

    fn light_travel_tick(&mut self, light_years: isize) -> Result<(), String> {
        let void_rows = self.void_rows()?;
        for void_y in void_rows {
            self.galaxies = self.galaxies
                .iter()
                .map(|(x, y)| if y > &void_y { (*x, y + light_years) } else { (*x, *y) })
                .collect();
        }
        let void_columns = self.void_columns()?;
        for void_x in void_columns {
            self.galaxies = self.galaxies
                .iter()
                .map(|(x, y)| if x > &void_x { (x + light_years, *y) } else { (*x, *y) })
                .collect();
        }
        Ok(())
    }

    fn galaxy_combinations(&self) -> Result<Vec<Vec<&Point>>, String> {
        let combinations = self.galaxies.iter().combinations(2).collect::<Vec<_>>();
        Ok(combinations)
    }

    fn galaxy_distance_sum(&self) -> Result<isize, String> {
        let sum_of_shortest_paths = self
            .galaxy_combinations()?
            .iter()
            .map(|pair| distance(pair[0], pair[1]))
            .sum::<isize>();
        Ok(sum_of_shortest_paths)
    }
}

fn distance(p1: &Point, p2: &Point) -> isize {
    isize::abs(p1.0 - p2.0) + isize::abs(p1.1 - p2.1)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn universe_expanding() -> Result<(), String> {
        let galaxies = HashSet::from([(0, 0), (3, 0), (0, 4), (2, 2)]);
        let mut image = SpaceImage::new(galaxies);
        assert_eq!(image.last_column()?, 3);
        assert_eq!(image.last_row()?, 4);
        assert_eq!(image.void_rows()?, vec![3, 1]);
        assert_eq!(image.void_columns()?, vec![1]);
        image.light_travel_tick(1)?;
        assert_eq!(image.galaxies, HashSet::from([(0, 0), (4, 0), (0, 6), (3, 3)]));
        Ok(())
    }

    #[test]
    fn solve_day11_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day11_part1(input)?, "374");
        Ok(())
    }

    #[test]
    fn solve_day11_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day11_part1(input)?, "9639160");
        Ok(())
    }

    #[test]
    fn solve_day11_part2_on_example_10() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        let mut image = SpaceImage::from(input);
        image.light_travel_tick(10 - 1)?;
        assert_eq!(image.galaxy_distance_sum()?, 1030);
        Ok(())
    }

    #[test]
    fn solve_day11_part2_on_example_100() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        let mut image = SpaceImage::from(input);
        image.light_travel_tick(100 - 1)?;
        assert_eq!(image.galaxy_distance_sum()?, 8410);
        Ok(())
    }

    #[test]
    fn solve_day11_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day11_part2(input)?, "752936133304");
        Ok(())
    }
}
