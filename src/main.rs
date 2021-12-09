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
    
    let num = strip_letters(item_string);
    
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


fn strip_letters(st: String) -> f32 {
    let re = Regex::new(r"(\d{1,})").unwrap();
    
    *re.find_iter(&st)
    //uses a mapped filter to grab the value at the iterator, pushes to str, and parses
    //then collects into the vector
	    .filter_map(|nums| nums.as_str().parse().ok())
	    .collect::<Vec<f32>>() //... which immediately gets removed, as we only care about the first value
        .first()
        .expect("Invalid argument attempted to strip.")
}


#[cfg(test)]
mod tests {
    #[test]
    fn letter_strip() {
        assert_eq!(32.0, super::strip_letters("32F".to_string()));
        assert_eq!(64.0, super::strip_letters("64C".to_string()));
    }
}
