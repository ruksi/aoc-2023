pub fn solve_day00_part1(input: String) -> Result<String, String> {
    Ok(input)
}

pub fn solve_day00_part2(input: String) -> Result<String, String> {
    Ok(input)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day00_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day00_part1(input)?, "#yolo");
        Ok(())
    }

    // #[test]
    // fn solve_day00_part1_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day00_part1(input)?, "#yolo");
    //     Ok(())
    // }

    // #[test]
    // fn solve_day00_part2_on_example() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day00_part2(input)?, "#yolo");
    //     Ok(())
    // }

    // #[test]
    // fn solve_day00_part2_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day00_part2(input)?, "#yolo");
    //     Ok(())
    // }
}
