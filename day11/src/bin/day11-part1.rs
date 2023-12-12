fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day11::solve_day11_part1(input)?;
    println!("{}", result);
    Ok(())
}
