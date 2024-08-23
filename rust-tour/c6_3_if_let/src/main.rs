fn main() {
    let config_max = Some(3u8);

    // match
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        // _ is a wildcard pattern that will match any value
        _ => println!("The maximum is not configured"),
    }

    // if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("The maximum is not configured");
    }
}
