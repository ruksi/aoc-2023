fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day02::solve_day02_part1(input)?;
    println!("{}", result);
    Ok(())
}
