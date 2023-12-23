fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day16::solve_day16_part2(input)?;
    println!("{}", result);
    Ok(())
}
