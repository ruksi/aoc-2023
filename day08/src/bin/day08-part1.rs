fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day08::solve_day08_part1(input)?;
    println!("{}", result);
    Ok(())
}
