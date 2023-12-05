fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day04::solve_day04_part1(input)?;
    println!("{}", result);
    Ok(())
}
