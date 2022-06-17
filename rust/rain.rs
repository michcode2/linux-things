extern crate termsize;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::{thread, time};

fn main() {
    //init
    println!("\u{001b}[36;48;1;9m");
    let window = termsize::get().unwrap();
    let mut chars = vec![vec![' '; window.cols.into()]; window.rows.into()];
    let delay = time::Duration::from_millis(50);
    
    chars = first_pass(&mut chars, 250, 10);
    display(&chars);
    loop{
        chars = drip(&chars);
        display(&chars);
        thread::sleep(delay)
    }
}
fn first_pass(characters: &mut Vec<Vec<char>>, start_chance: u8, drip_chance: u8) -> Vec<Vec<char>>{ // sets the initial number of drops

    for x in 0..characters.len()-5{
        for y in 0..characters[0].len(){
            if rand::random::<u8>() > start_chance{
                characters[x][y] = random_char();
                
            }else{ // makes it more likely for more characters under each character
                if x > 1 && characters[x-1][y] !=' '{
                    if rand::random::<u8>() > drip_chance{
                        characters[x][y] = random_char();
                    }
                }
            }
        }
    }
    characters.to_vec()
}

fn drip(input: &Vec<Vec<char>>)-> Vec<Vec<char>>{
    let mut output = vec![vec![' '; input[0].len()]; input.len()];

    //copy the existing data between the vecs
    for i in 0..input.len()-1{
        for j in 0..input[0].len(){
            output[i+1][j] = input[i][j];
        }
    }
    output = first_pass(&mut output, 254, 225);

    output

}

fn display(data: &Vec<Vec<char>>){
    for x in 0..data.len(){
        println!();
    }
    let mut output = String::new();
    
    for x in 0..data.len(){
        for y in 0..data[0].len(){
            output.push(data[x][y]);
        }
        
    }
    println!("{output}");
}
fn random_char() -> char{
    let mut rng = thread_rng();
    rng.sample(Alphanumeric) as char

}
