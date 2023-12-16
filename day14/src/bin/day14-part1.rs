fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day14::solve_day14_part1(input)?;
    println!("{}", result);
    Ok(())
}
