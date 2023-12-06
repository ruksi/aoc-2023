fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day05::solve_day05_part2(input)?;
    println!("{}", result);
    Ok(())
}
