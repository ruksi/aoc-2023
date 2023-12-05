fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day03::solve_day03_part1(input)?;
    println!("{}", result);
    Ok(())
}
