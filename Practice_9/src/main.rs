fn main() {

    let arr:[i32;4] = [10,20,30,40];
    println!("array is  {:?}",arr);
    println!("array size is :{}",arr.len());

    for val in arr.iter(){
        println!("value is :{}",val);
    }
    
    //USING MY EXAMPLE FROM PRACTICE_8

    let vbox:[&str;6] = ["ben 10","tom&jerry","spider-man","lab-rats","Mighty-Med","Kick-init"];
    println!(" The Best TV shows of my time are {:?}",vbox);
    println!("They are the Top {} on my favourite list",vbox.len());

    for x in vbox.iter(){
        println!("The show is {}",x);
    }
    
//ANOTHER EXAMPLE WITH NUMBERS

  let num = [0,1,2,3,4,5,6,7,8,9];
  println!("The first ten numbers are {:?}",num);
  println!("Their total number is {}", num.len());

  for t in num.iter(){
    println!("Number {t}");
  }


}
