fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is:{}",num);

//ANOTHER EXAMPLE
    let x:i32 = 65;
    guy(x);
    println!("The value of x is:{x}");
}
fn mutate_num_to_zero(mut param_num: i32) {
    param_num = param_num*0;
    println!("param_num value is :{}",param_num);
}  

//THE FUNCTION OF THE EXAMPLE
fn guy(mut a: i32) {
    a = a * 5;
    println!("a value is :{a}");
}
// NOTE: a = x but X is not = to a

