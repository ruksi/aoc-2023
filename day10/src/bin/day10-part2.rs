fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day10::solve_day10_part2(input)?;
    println!("{}", result);
    Ok(())
}
