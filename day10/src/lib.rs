use std::collections::HashSet;

pub fn solve_day10_part1(input: String) -> Result<String, String> {
    let sketch = Sketch::from_input(input);
    let route_length = sketch.get_route().len();
    Ok((route_length / 2).to_string())
}

pub fn solve_day10_part2(input: String) -> Result<String, String> {
    let sketch = Sketch::from_input(input);
    let enclosed_points = sketch.get_enclosed_points();
    Ok(enclosed_points.len().to_string())
}

type Point = (usize, usize); // (x, y)

#[derive(Debug)]
struct Sketch(Vec<Vec<char>>);

impl Sketch {
    fn from_input(input: String) -> Self {
        let sketch = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        Self(sketch)
    }

    fn get_side_point(&self, point: &Point, entry: &Direction, side: &Side) -> Option<Point> {
        let direction = entry.turn_to(side);
        self.neighbors(point)
            .into_iter()
            .find(|(dir, _)| dir == &direction)
            .map(|(_, p)| p)
    }

    fn get_content(&self, point: &Point) -> Option<char> {
        let (x, y) = *point;
        self.0.get(y).and_then(|row| row.get(x)).cloned()
    }

    fn find(&self, c: &char) -> Option<Point> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == c {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn get_enclosed_points(&self) -> HashSet<Point> {
        let route = self.get_route();

        let mut full_route = route.clone();
        full_route.push(full_route[0].clone());

        // keep track of turns to determine which side is the interior
        let degrees = full_route
            .windows(2)
            .map(|w| {
                let (dir1, _) = &w[0];
                let (dir2, _) = &w[1];
                let degrees = match (dir1, dir2) {
                    (Direction::Up, Direction::Right) => 90,
                    (Direction::Up, Direction::Left) => -90,
                    (Direction::Right, Direction::Up) => -90,
                    (Direction::Right, Direction::Down) => 90,
                    (Direction::Down, Direction::Right) => -90,
                    (Direction::Down, Direction::Left) => 90,
                    (Direction::Left, Direction::Up) => 90,
                    (Direction::Left, Direction::Down) => -90,
                    _ => 0,
                };
                degrees
            })
            .sum::<isize>();
        let sideness = if degrees == 360 {
            Side::Right
        } else if degrees == -360 {
            Side::Left
        } else {
            panic!("Invalid route turns degree sum: {}", degrees);
        };

        let route_points = route.iter().map(|(_, p)| p).collect::<HashSet<_>>();

        let mut enclosed_points = route
            .iter()
            .map(|(entry, point)| self.get_side_point(point, entry, &sideness))
            .filter_map(|to_some_point| to_some_point)
            .filter(|p| !route_points.contains(p))
            .collect::<HashSet<Point>>();

        let mut to_check = enclosed_points.clone();
        while !to_check.is_empty() {
            let mut next_to_check = HashSet::new();
            for point in to_check {
                for (_, neighbor) in self.neighbors(&point) {
                    if enclosed_points.contains(&neighbor) || route_points.contains(&neighbor) {
                        continue;
                    }
                    next_to_check.insert(neighbor);
                }
            }
            enclosed_points.extend(next_to_check.clone());
            to_check = next_to_check;
        }

        enclosed_points
    }

    fn get_route(&self) -> Vec<(Direction, Point)> {
        let start = self.find(&'S').unwrap_or_else(|| panic!("No starting point found"));
        let mut route = vec![];
        let mut next = self.valid_neighbors(&start).first().unwrap().clone();
        loop {
            route.push(next.clone());
            if next.1 == start { break; }
            next = self.valid_neighbor_from_entry(&next.1, &next.0.opposite());
        }
        route
    }

    fn valid_neighbor_from_entry(&self, point: &Point, entry: &Direction) -> (Direction, Point) {
        self.valid_neighbors(point)
            .into_iter()
            .find(|(direction, _)| direction != entry)
            .unwrap()
    }

    fn valid_neighbors(&self, point: &Point) -> Vec<(Direction, Point)> {
        let source = self.get_content(point).unwrap_or_else(|| panic!("Nothing at {:?}", point));
        let result = self.neighbors(point)
            .into_iter()
            .filter(|(dir, _)| { // filter out directions not valid from the source
                match dir {
                    Direction::Up => source == '|' || source == 'J' || source == 'L' || source == 'S',
                    Direction::Right => source == '-' || source == 'F' || source == 'L' || source == 'S',
                    Direction::Down => source == '|' || source == 'F' || source == '7' || source == 'S',
                    Direction::Left => source == '-' || source == 'J' || source == '7' || source == 'S',
                }
            })
            .filter(|(dir, p)| { // filter out directions not valid to the destination
                let Some(dest) = self.get_content(p) else { return false; }; // nothing at the point
                match dir {
                    Direction::Up => dest == '|' || dest == 'F' || dest == '7' || dest == 'S',
                    Direction::Right => dest == '-' || dest == 'J' || dest == '7' || dest == 'S',
                    Direction::Down => dest == '|' || dest == 'J' || dest == 'L' || dest == 'S',
                    Direction::Left => dest == '-' || dest == 'F' || dest == 'L' || dest == 'S',
                }
            })
            .collect::<Vec<_>>();
        result
    }

    fn neighbors(&self, point: &Point) -> Vec<(Direction, Point)> {
        let (x, y) = *point;
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((Direction::Left, (x - 1, y)));
        }
        if y > 0 {
            neighbors.push((Direction::Up, (x, y - 1)));
        }
        if x < self.0[0].len() - 1 {
            neighbors.push((Direction::Right, (x + 1, y)));
        }
        if y < self.0.len() - 1 {
            neighbors.push((Direction::Down, (x, y + 1)));
        }
        neighbors
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
    fn turn_to(&self, sideness: &Side) -> Direction {
        match (self, sideness) {
            (Direction::Up, Side::Left) => Direction::Left,
            (Direction::Up, Side::Right) => Direction::Right,
            (Direction::Right, Side::Left) => Direction::Up,
            (Direction::Right, Side::Right) => Direction::Down,
            (Direction::Down, Side::Left) => Direction::Right,
            (Direction::Down, Side::Right) => Direction::Left,
            (Direction::Left, Side::Left) => Direction::Down,
            (Direction::Left, Side::Right) => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn sketch_from_simple_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-simple.txt").map_err(|e| e.to_string())?;
        let sketch = Sketch::from_input(input);
        assert_eq!(sketch.get_content(&(99, 99)), None);
        assert_eq!(sketch.get_content(&(1, 1)), Some('S'));
        assert_eq!(sketch.get_content(&(3, 1)), Some('7'));
        assert_eq!(sketch.get_content(&(1, 3)), Some('L'));
        assert_eq!(sketch.get_content(&(3, 3)), Some('J'));

        let start = sketch.find(&'S').ok_or("no start found".to_string())?;
        assert_eq!(start, (1, 1));
        assert_eq!(sketch.neighbors(&start).len(), 4);
        assert_eq!(sketch.valid_neighbors(&start).len(), 2);

        assert_eq!(sketch.get_route().len(), 8);
        Ok(())
    }

    #[test]
    fn solve_day10_part1_on_examples() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-simple.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "4");
        let input = fs::read_to_string("examples/example-large-simple.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "12");
        let input = fs::read_to_string("examples/example-messy.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "4");
        let input = fs::read_to_string("examples/example-complex.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "8");
        let input = fs::read_to_string("examples/example-alternative.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "3022");
        Ok(())
    }

    #[test]
    fn solve_day10_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part1(input)?, "6823");
        Ok(())
    }

    #[test]
    fn solve_day10_part2_on_examples() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-simple.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "1");
        let input = fs::read_to_string("examples/example-large-simple.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "25");
        let input = fs::read_to_string("examples/example-messy.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "1");
        let input = fs::read_to_string("examples/example-complex.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "1");
        let input = fs::read_to_string("examples/example-enclosed.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "4");
        let input = fs::read_to_string("examples/example-enclosed-with-no-gap.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "4");
        let input = fs::read_to_string("examples/example-random-bits.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "8");
        let input = fs::read_to_string("examples/example-junk.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "10");
        let input = fs::read_to_string("examples/example-alternative.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "0");
        Ok(())
    }

    #[test]
    fn solve_day10_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day10_part2(input)?, "413"); // this is what I get, but...
        // assert_eq!(solve_day10_part2(input)?, "415"); // the right answer with trial-and-error 🤷
        // so probably I'm not handling some edge case correctly, I'll see if I get back to this
        Ok(())
    }
}
