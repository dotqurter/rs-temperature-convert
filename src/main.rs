use std::io;
use regex::Regex;

fn main() {
    println!("Input temperature: ");
    let mut item_string = String::new();

    io::stdin() // Reading the terminal input.
        .read_line(&mut item_string)
        .unwrap();

    // Hacky way of telling if the input is Fahrenheit or Celsius for the calculation and end result.
    let measure: bool;
    if item_string.contains(&['F', 'f'][..]) { measure = true; }
    else if item_string.contains(&['C', 'c'][..]) { measure = false; }
    else { panic!("No measure given!"); }
    
    let num: f32 = item_string // Convert the input to a number.
        .trim() // Remove the newline
        .replace(&['F', 'f', 'C', 'c'][..], "")
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


fn strip_letters(st: String) -> Vec<f32> {
    let re = Regex::new(r"(\d+)").unwrap();
    
    re.find_iter(&st)
    //uses a mapped filter to grab the value at the iterator, pushes to str, and parses
    //then collects into the vector
	.filter_map(|nums| nums.as_str().parse().ok())
	.collect()
}
