fn main() {
// HOW TO WRITE AN ARRAY with DATATYPE and SIZE

    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("array is {:?}",arr1);
    println!("array size is:{}",arr1.len());

// HOW TO WRITE AN ARRAY without DATATYPE but with SIZE

    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\nArray without data type");
    println!("array is {:?}",arr2);
    println!("array size is :{}",arr2.len());

// HOW TO WRITE AN ARRAY with DEFAULT VALUES and SIZE

    let arr3:[i32;8] = [-1;8];//-> THIS means (-1) will occur (8) times in the array
    println!("\nArray with default values");
    println!("array is {:?}",arr3);//-> THE {:?} syntax of the println!() function is used to print all values in the array
    println!("array size is :{}",arr3.len());//->.len() calculates the number of elements in the array

  let vbox = ["ben 10","tom&jerry","spider-man","lab-rats","Mighty-Med","Kick-init"];
  println!("\n This array is made up of string slices");
  println!(" The Best TV shows of my time are {:?}",vbox);
  println!("They are the Top {} on my favourite list",vbox.len());

}
