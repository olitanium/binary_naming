fn main() -> Result<(), &'static str> {
    let num: usize = std::env::args().nth(1)
        .map_or_else(
            || text_io::try_read!("{}").map_err(|_| "Failed_to read from stdin"),
            |x| x.parse().map_err(|_| "Please use a correct number format")
        )?;

    println!("{}", name_number(num));

    Ok(())
}

fn name_number(input: usize) -> String {
    const NAMES: [&str; 7] = [
        "two",
        "four",
        "hex",
        "byte",
        "short",
        "int",
        "long",
    ];
    
    if input == 1 {
        return "one".into();
    }
    
    let split_index = (input.ilog2() + 1).next_power_of_two() >> 1;
    
    let name = NAMES[split_index.ilog2() as usize];
    
    let upper = input >> split_index;
    let head = if upper != 1 {
        format!("({})-",name_number(upper))
    } else {
        "".into()
    };
    
    let lower = input & !(usize::MAX << split_index);
    let tail = if lower != 0 {
        format!(" {}", name_number(lower))
    } else {
        "".into()
    };

    format!("{head}{name}{tail}")
}