use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = (Alzheimer, Arrhythmia, Chronic_Kidney_Disease, Diabetes, Arthrites);
    let Alzheimer:i32 = 1_200_000;
    let Arrhythmia:i32 = 550_000;
    let Chronic_Kidney_Disease:i32 = 1_500_000;
    let Diabetes:i32 = 800_000;
    let Arthrites:i32 = 450_000;
    let mut input8 = Akpabom, Ngbauji, Atabrikang, Okorobilom, Emeremen;
println!("Enter your name");
io::stdin().read_line(&mut input1).expect("Nothing");

println!("Enter yor Date of Birth");
io::stdin().read_line(&mut input2).expect("Nothing");
let date_of_birth:f32 = input2.trim().parse().expect("Not a Valid Integer");

println!("Enter your E-mail Address");
io::stdin().read_line(&mut input3).expect("Nothing");

println!("Enter your Phone Number");
io::stdin().read_line(&mut input4).expect("Nothing");
let phone_number:f32 = input4.trim().parse().expect("Not a Valid Integer");

println!("Enter the number of siblings you have");
io::stdin().read_line(&mut input5).expect("Nothing");
let number_of_siblings:i32 = input5.trim().parse().expect("Not a Valid Integer");

println!("Enter the number of children you have");
io::stdin().read_line(&mut input6).expect("Nothing");
let number_of_children:i32 = input6.trim().parse().expect("Not a Valid Integer");

println!("Enter your Medical diagnosis");
io::stdin().read_line(&mut input7).expect("Nothing");

println!("Enter your Village of residence");
io::stdin().read_line(&mut input8).expect("Nothing");
 
let year_of_birth:i32 = input2.trim().parse().expect("Not a Valid Integer");
// TAKE THE YEAR FROM THE DATE OF BIRTH AND SUBTRACT FROM 2024
let age = 2024 - (year_of_birth);

if  age > 50 & Alzheimer & number_of_children > 4 & Akpabom {
    let mut new balance:i32 = 80 / 100 * 1_200_000;
    println!("Recieves 20% discount",new balance);
} 

if  age = 30 & Arrhythmia & number_of_siblings > 4 & Ngbauji {
    let new balance:i32 = 95 / 100 * 550_000;
    println!("Recieves 5% discount",new balance);
} 

if  age > 40 & Chronic_Kidney_Disease & number_of_children > 3 & number_of_siblings > 3 & Atabrikang {
    let new balance:i32 = 85 / 100 * 1_500_000;
    println!("Recieves 20% discount",new balance);
} 

if  28 < age > 45 & Diabetes & number_of_children = 2,3,4 & Okorobilom {
    let new balance:i32 = 90 / 100 * 1_200_000;
    println!("Recieves 15% discount",new balance);
} 

if  age > 58 & Arthrites & number_of_children > 5 & number_of_siblings > 5 & Emeremen {
    let new balance:i32 = 90 / 100 * 1_200_000;
    println!("Recieves 10% discount",new balance);
} // APPLIES TO THE FIRST 100 PEOPLE 
}