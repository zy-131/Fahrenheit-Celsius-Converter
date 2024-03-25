use std::io;

fn main() {
    println!("Temperature Conversion Calculator\n");

    println!("Which conversion would you like?");
    println!("F for Fahrenheit to Celsius / C for Celsius to Fahrenheit: ");

    let choice = loop {
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().as_ref() {
            "C" | "F" => break choice,
            _ => {
                println!("Please input F or C");
                continue;
            }
        };
    };

    println!("\nPlease input temperature value to be converted: ");

    let temp = loop {
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse() {
            Ok(temp) => break temp,
            Err(_) => {
                println!("Please input a valid number (decimals are valid)!");
                continue;
            }
        };
    };

    println!();

    if choice.trim() == "F" {
        let res = fahrenheit_to_celsius(temp);
        println!("{temp} F\u{00b0} in Celsius is {:.2} C\u{00b0}", res);
    } else if choice.trim() == "C" {
        let res = celsius_to_fahrenheit(temp);
        println!("{temp} C\u{00b0} in Fahrenehit is {:.2} F\u{00b0}", res);
    } else {
        println!("Could not calulcate. Run again");
    }
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    let c = (temp - 32.0) * (5.0 / 9.0);
    c
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    let f = (temp * (9.0 / 5.0)) + 32.0;
    f
}
