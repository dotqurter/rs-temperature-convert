use std::io;

fn main() {
    println!("Input temperature: ");
    let mut item_string = String::new();

    io::stdin() // Reading the terminal input.
        .read_line(&mut item_string)
        .unwrap();

    // Hacky way of telling if the input is Fahrenheit or Celsius for the calculation and end result.
    let measure: bool;
    if item_string.contains('F') { measure = true; }
    else if item_string.contains('C') { measure = false; }
    else { panic!("No measure given!"); }
    
    let num: f32 = item_string // Convert the input to a number.
        .trim() // Remove the newline
        .replace(&['F','C'][..], "")
        .parse::<f32>()
        .expect("Not a number!");

    let final_answer = match measure {
        true  => ((num - 32.0) * 0.55),
        false => ((num * 1.8) + 32.0), 
    };
    
    let msre: &'static str = match measure {
        true => ("C"),
        false => ("F"),
    };
    
    println!("{}{}", final_answer, msre);
}
