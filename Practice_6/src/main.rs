fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is:{}",num);

    //FROM EXAMPLE IN PRACTICE_5
    let mut x:i32 = 65;
    guy(&mut x);
    println!("The value of x is {x}");
}
fn mutate_num_to_zero(param_num:&mut i32){
    *param_num = *param_num*0;
    println!("param_num value is:{}",param_num);
}
//FROM EXAMPLE IN PRACTICE_5

fn guy(a:&mut i32) {
    *a = *a * 5;
    println!("a is {a}");
}
// THIS IS WHERE a = X and X = a
