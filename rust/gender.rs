use std::process::Command;
use std::{str, env};

fn main() {
    // setup 
    
    //height of window
    let output = Command::new("tput").arg("lines").output().unwrap().stdout;
    let lines = ints_to_string(&output);
    
    //width of window
    let output = Command::new("tput").arg("cols").output().unwrap().stdout;
    let cols = ints_to_string(&output);
    
    //how many lines each bit of the flag should take up
    let stripes = 5;
    let stripe_height: u16 = (lines as f32/stripes as f32).floor() as u16;
    
    //colours for each cell and the helpers for the text
    let codes = ["81;30", "218;30", "15;30", "218;30", "81;30"];
    let letters:Vec<char>;
    let mut counter = 0;

    let mut out = String::new();

    
    let input_string: Vec<String> = env::args().collect();
    //default to a blank flag, otherwise format the arguments to make them look nice
    if input_string.len() > 1{
        letters = generate_text(&input_string[1..]);
    }else{
        letters = vec![' '];
    }
    
    for a in 0..stripes{
        out = out + &format!("\u{001b}[48;5;{}m", codes[a as usize]);//change the colour
            for _j in 0..stripe_height{
                for _i in 0..cols{
                    out.push(letters[counter]);
                    //move onto the next letter and wrap if necessary
                    counter+=1;
                    if counter == letters.len(){
                        counter = 0;
                    }

                }
            }
        out.push('\n');
    }
    out = out + &"\u{001b}[0m\n";
    print!("{out}");

}


fn ints_to_string(data: &Vec<u8>) -> u16 { // makes stdio from a list of ascii codes to integers
    let string_version = str::from_utf8(data).unwrap();//do the conversion
    let no_whitespace = string_version.replace("\n", "");//get rid of the newline
    let int_ver: u16 = no_whitespace.parse().unwrap();//convert the string to integers and return it
    int_ver
}


fn generate_text(words: &[String]) -> Vec<char>{
    let mut combined = String::new();
    
    //put the indicies into the buffer with a space between each of them
    for i in 0..words.len(){
        combined = combined + &words[i];
        combined = combined + " ";
    }

    let splitted:Vec<char> = combined.chars().collect(); // make the string into an array of characters 
    let mut out = Vec::new();

    //put a space between each letter for fun
    for i in 0..splitted.len(){
        out.push(splitted[i]);
        out.push(' ');
    }
    out
}
