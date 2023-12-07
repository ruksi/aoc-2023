pub fn solve_day06_part1(input: String) -> Result<String, String> {
    let records = RaceRecord::many_from(&input)?;
    let multiplied = records
        .iter()
        .map(|r| r.count_ways_to_set_new_record())
        .product::<usize>();
    Ok(multiplied.to_string())
}

pub fn solve_day06_part2(input: String) -> Result<String, String> {
    let input = input.replace(" ", "");
    let input = input.replace(":", ": ");
    solve_day06_part1(input)
}

#[derive(Debug)]
struct RaceRecord {
    race_duration: usize,
    record_distance: usize,
}

impl RaceRecord {
    fn new(race_duration: usize, record_distance: usize) -> Self {
        Self { race_duration, record_distance }
    }

    fn many_from(input: &str) -> Result<Vec<Self>, String> {
        let mut lines = input.lines();

        let mut time_row = lines.next().ok_or("Missing time row")?.split_whitespace();
        time_row.next(); // discard "Time:"
        let times = time_row
            .map(|t| t.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut distance_row = lines.next().ok_or("Missing distance row")?.split_whitespace();
        distance_row.next(); // discard "Distance:"
        let distances = distance_row
            .map(|t| t.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let records = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| {
            Self { race_duration: time, record_distance: distance }
        }).collect::<Vec<_>>();

        Ok(records)
    }

    fn do_we_break_the_record_by_holding_ms(&self, hold_ms: usize) -> bool {
        let distance = calculate_distance(self.race_duration, hold_ms);
        distance > self.record_distance
    }

    fn count_ways_to_set_new_record(&self) -> usize {
        let min_ms_fn = |x| self.do_we_break_the_record_by_holding_ms(x);
        let min_ms = binary_search(0, self.race_duration, min_ms_fn);
        let max_ms_fn = |x| !self.do_we_break_the_record_by_holding_ms(x);
        let max_ms = binary_search(0, self.race_duration, max_ms_fn);
        max_ms - min_ms
    }
}

fn calculate_distance(total_ms: usize, hold_ms: usize) -> usize {
    let diff_ms = if total_ms >= hold_ms { total_ms - hold_ms } else { 0 };
    diff_ms * hold_ms
}

fn binary_search<F>(mut min: usize, mut max: usize, mut f: F) -> usize
    where F: FnMut(usize) -> bool
{
    while min < max {
        let mid = min + (max - min) / 2;
        if f(mid) {
            max = mid;
        } else {
            min = mid + 1;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn calculate_distances_like_described_in_readme() -> Result<(), String> {
        assert_eq!(calculate_distance(7, 0), 0);
        assert_eq!(calculate_distance(7, 1), 6);
        assert_eq!(calculate_distance(7, 2), 10);
        assert_eq!(calculate_distance(7, 3), 12);
        assert_eq!(calculate_distance(7, 4), 12);
        assert_eq!(calculate_distance(7, 5), 10);
        assert_eq!(calculate_distance(7, 6), 6);
        assert_eq!(calculate_distance(7, 7), 0);
        assert_eq!(calculate_distance(7, 8), 0); // #yolo ⏬⏬⏬
        Ok(())
    }

    #[test]
    fn does_holding_this_long_break_the_record() -> Result<(), String> {
        let record = RaceRecord::new(7, 9);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(0), false);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(1), false);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(2), true);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(3), true);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(4), true);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(5), true);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(6), false);
        assert_eq!(record.do_we_break_the_record_by_holding_ms(7), false);
        Ok(())
    }

    #[test]
    fn test_our_binary_search() -> Result<(), String> {
        let record = RaceRecord::new(7, 9);
        let min_ms_fn = |x| record.do_we_break_the_record_by_holding_ms(x);
        let min_ms = binary_search(0, record.race_duration, min_ms_fn);
        let max_ms_fn = |x| !record.do_we_break_the_record_by_holding_ms(x);
        let max_ms = binary_search(0, record.race_duration, max_ms_fn);
        assert_eq!(min_ms, 2);
        assert_eq!(max_ms, 6); // apply -1 later if one wanted to get "the last one that wins"
        Ok(())
    }

    #[test]
    fn solve_day06_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day06_part1(input)?, "288");
        Ok(())
    }

    #[test]
    fn solve_day06_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day06_part1(input)?, "128700");
        Ok(())
    }

    #[test]
    fn solve_day06_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day06_part2(input)?, "71503");
        Ok(())
    }

    #[test]
    fn solve_day06_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day06_part2(input)?, "39594072");
        Ok(())
    }
}
