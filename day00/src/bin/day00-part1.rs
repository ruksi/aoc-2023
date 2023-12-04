fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day00::solve_day00_part1(input)?;
    println!("{}", result);
    Ok(())
}
