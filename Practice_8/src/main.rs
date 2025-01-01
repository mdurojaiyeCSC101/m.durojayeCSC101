fn main() {

    let city_arr:[&str;5] = ["Abuja","Portharcourt","Maiduguri","Kano","Lagos"];
    println!("array is {:?}",city_arr);
    println!("array size is :{}",city_arr.len());

    for index in 0..5 {
        println!("City index {} is located in : {}",index,city_arr[index]);
    }

    //USING MY EXAMPLE FROM PRACTICE_7

    let vbox:[&str;6] = ["ben 10","tom&jerry","spider-man","lab-rats","Mighty-Med","Kick-init"];
    println!(" The Best TV shows of my time are {:?}",vbox);
    println!("They are the Top {} on my favourite list",vbox.len());

    //THE USE OF THE for..in{} STATEMENT
    println!("My order of arrangement is:");
    for x in 0..6 {
        println!("No.{} {}",x + 1,vbox[x]);
    }


}
