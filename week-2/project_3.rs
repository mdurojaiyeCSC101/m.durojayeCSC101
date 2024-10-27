fn main(){
   let p:f64 = 510_000.00;
   let r:f64 = 5.0;
   let n:f64 = 3.0;

   /*Formula for Depreciation
   A=P[1-(R/100)]^n */
   let a = p * (1.0 - (r/100.0)).powf(n);
   let rounded = a.round();
   println!("New price of TV = {}", a);
   println!("{} rounded is {}", a, rounded)

}