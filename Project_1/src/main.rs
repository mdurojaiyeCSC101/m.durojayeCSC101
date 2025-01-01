#![allow(warnings)]
use std::io;
fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    let mut input_4 = String::new();
    let mut input_5 = String::new();

 println!("Please enter your desired input");
 io::stdin().read_line(&mut input_1).expect("Nothing");
 let input1:i32 = input_1.trim().parse().expect("Nothing");

 io::stdin().read_line(&mut input_2).expect("Nothing");
 let input2:i32 = input_2.trim().parse().expect("Nothing");

 io::stdin().read_line(&mut input_3).expect("Nothing");
 let input3:i32 = input_3.trim().parse().expect("Nothing");

 io::stdin().read_line(&mut input_4).expect("Nothing");
 let input4:i32 = input_4.trim().parse().expect("Nothing");

 io::stdin().read_line(&mut input_5).expect("Nothing");
 let input5:i32 = input_5.trim().parse().expect("Nothing");

 

 let mut param1 = String::new();
 let mut param2 = String::new();
 let mut param3 = String::new();
 let mut param4 = String::new();
 let mut param5 = String::new();

 let input1 = &mut param1;
 println!("value of h:");
 io::stdin().read_line(&mut param1).expect("Nothing");
 let h:i32 = param1.trim().parse().expect("Nothing");

 println!("value of base1:");
 io::stdin().read_line(&mut param1).expect("Nothing");
 let base1:i32 = param1.trim().parse().expect("Nothing");

 println!("value of base2:");
 io::stdin().read_line(&mut param1).expect("Nothing");
 let base2:i32 = param1.trim().parse().expect("Nothing");


 let trapezium_area:i32 = area_of_trapezium(h,base1,base2);
 println!("The area of a Trapezium = {}",trapezium_area); 
 
 let input2 = &mut param2;
 println!("value of diagonal-1:");
 io::stdin().read_line(&mut param2).expect("Nothing");
 let diagonal_1:i32 = param2.trim().parse().expect("Nothing");

 println!("value of diagonal-2:");
 io::stdin().read_line(&mut param2).expect("Nothing");
 let diagonal_2:i32 = param2.trim().parse().expect("Nothing");

 let rhombus_area:i32 = area_of_rhombus(diagonal_1,diagonal_2);
 println!("The area of a Rhombus = {}",rhombus_area);
 
 let input3 = &mut param3;
 println!("value of base:");
 io::stdin().read_line(&mut param3).expect("Nothing");
 let base:i32 = param3.trim().parse().expect("Nothing");

 println!("value of altitude:");
 io::stdin().read_line(&mut param3).expect("Nothing");
 let altitude:i32 = param3.trim().parse().expect("Nothing");

 let parallelogram_area:i32 = area_of_parallelogram(base,altitude);
 println!("The area of a Parallelogram = {}",parallelogram_area);
 
 let input4 = &mut param4;
 println!("value of length of side:");
 io::stdin().read_line(&mut param4).expect("Nothing");
 let length_of_side:i32 = param4.trim().parse().expect("Nothing");

 let cube_area:i32 = area_of_cube(length_of_side);
 println!("The area of a Cube = {}",cube_area);
 
 let input5 = &mut param5;
 println!("value of radius:");
 io::stdin().read_line(&mut param1).expect("Nothing");
 let radius:f32 = param1.trim().parse().expect("Nothing");

 println!("value of height:");
 io::stdin().read_line(&mut param1).expect("Nothing");
 let height:f32 = param1.trim().parse().expect("Nothing");

 let cylinder_area:f32 = area_of_cylinder(radius,height);
 println!("The area of a Trapezium = {}",cylinder_area);
 

}

fn area_of_trapezium(h:i32,base1:i32,base2:i32)->i32{
    h / 2 * (base1 + base2)
}

fn area_of_rhombus(diagonal_1:i32,diagonal_2:i32)->i32{
    1 / 2 * (diagonal_1 * diagonal_2)
}
fn area_of_parallelogram(base:i32,altitude:i32)->i32{
    base * altitude
}
fn area_of_cube(length_of_side:i32)->i32{
    6 * (length_of_side * length_of_side)
}
fn area_of_cylinder(radius:f32,height:f32)->f32{
    3.142 * (radius * radius) * height
}
