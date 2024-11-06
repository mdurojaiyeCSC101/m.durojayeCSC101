/* RUST TO READ THE HEIGHT OF A PERSON AND THEN PRINT IF PERSON IS TALL, DWARF, OR AVERAGE HEIGHT PERSON*/

use std::io;

fn main() 
{
    let mut input = String::new();

    println!("\nEnter Your Height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf");
    }
    else 
    {
        println!("Abnormal height");
    }
}
