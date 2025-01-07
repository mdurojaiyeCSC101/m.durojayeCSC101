use std::io;

fn main(){
    let mut patient_count = 0;

loop {
    patient_count+=1;
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    println!("What is your Name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim().to_uppercase();


    println!("Date of birth (year): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let birth_year:i32 = input2.trim().parse().expect("not a valid number");
    let current_year = 2024;
    let age  = current_year - birth_year;

    println!("Your email address: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let email_address = input3.trim().to_uppercase();


    println!("Your phone number: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let  phone_number = input4.trim().to_uppercase();

    println!("Number of siblings: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let no_siblings:i32 = input5.trim().parse().expect("not a valid number");

    println!("Number of children: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let no_children:i32 = input6.trim().parse().expect("not a valid number");

    println!("Mediccal diagnosis: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let medical_diagnosis = input7.trim().to_uppercase();

    println!("village of residence: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let village_of_residence = input8.trim().to_uppercase();

    if medical_diagnosis == "ALZHEIMER" && age >= 50 && no_children >= 4 && village_of_residence == "AKPABOM"{
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        let amount: i32 = 1_200_000 - (1_200_000 * 20)/100;
        println!("Amount to be paid : {}", amount);
        println!("Total number of patients processed: {}", patient_count);
        continue;
    } else if medical_diagnosis == "ARRHYTHMIA" && age == 30 && no_siblings >= 4 && village_of_residence == "NGBAUJI" {
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        let amount1: i32 = 550_000 - (550_000 * 5)/100;
        println!("Amount to be paid : {}", amount1);
        println!("Total number of patients processed: {}", patient_count);
        continue;
    } else if medical_diagnosis == "CHRONIC KIDNEY DISEASE" && age >= 40 && no_children >= 3 && no_siblings >= 3 && village_of_residence == "ATABRIKANG"{
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        let amount2: i32 =1_500_000 - (1_500_000 * 15)/100;
        println!("Amount to be paid : {}", amount2);
        println!("Total number of patients processed: {}", patient_count);
        continue;
    } else if medical_diagnosis == "DIABETE" && age >= 28 || age <=45 && no_children >= 2 || no_children <= 4 && village_of_residence == "OKOROBILOM"{
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        let amount3: i32 = 800_000 - (800_000 * 10)/100;
        println!("Amount to be paid : {}", amount3);
        println!("Total number of patients processed: {}", patient_count);
        continue;
    } else if medical_diagnosis == "ARTHRITIS" && age >= 58 && no_children >= 5 && no_siblings >= 5 && village_of_residence == "EMEREMEN"{
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        let amount4: i32 = 450_000 - (450_000 * 10)/100;
        println!("Amount to be paid : {}", amount4);
        println!("Total number of patients processed: {}", patient_count);
        continue;
    } else {
        if village_of_residence == "AKPABOM" {
        println!("Name:{}", name);
        println!("Date of birth :{}", birth_year);
        println!("Email adress :{}", email_address);
        println!("Phone number :{}", phone_number);
        println!("Number of siblings :{}", no_siblings);
        println!("Number of children :{}", no_children);
        println!("Mediccal diagnosis :{}", medical_diagnosis);
        println!("Village of residence :{}", village_of_residence);
        println!("Amount to be paid : N1,200,000");
        } else if village_of_residence == "NGBAUJI" {
            println!("Name:{}", name);
            println!("Date of birth :{}", birth_year);
            println!("Email adress :{}", email_address);
            println!("Phone number :{}", phone_number);
            println!("Number of siblings :{}", no_siblings);
            println!("Number of children :{}", no_children);
            println!("Mediccal diagnosis :{}", medical_diagnosis);
            println!("Village of residence :{}", village_of_residence);
            println!("Amount to be paid : N550,000");
        } else if village_of_residence == "ATABRIKANG" {
            println!("Name:{}", name);
            println!("Date of birth :{}", birth_year);
            println!("Email adress :{}", email_address);
            println!("Phone number :{}", phone_number);
            println!("Number of siblings :{}", no_siblings);
            println!("Number of children :{}", no_children);
            println!("Mediccal diagnosis :{}", medical_diagnosis);
            println!("Village of residence :{}", village_of_residence);
            println!("Amount to be paid : N1,500,000");
        } else if village_of_residence == "OKOROBILOM"{
            println!("Name:{}", name);
            println!("Date of birth :{}", birth_year);
            println!("Email adress :{}", email_address);
            println!("Phone number :{}", phone_number);
            println!("Number of siblings :{}", no_siblings);
            println!("Number of children :{}", no_children);
            println!("Mediccal diagnosis :{}", medical_diagnosis);
            println!("Village of residence :{}", village_of_residence);
            println!("Amount to be paid : N800,000");
        } else {
            println!("Name:{}", name);
            println!("Date of birth :{}", birth_year);
            println!("Email adress :{}", email_address);
            println!("Phone number :{}", phone_number);
            println!("Number of siblings :{}", no_siblings);
            println!("Number of children :{}", no_children);
            println!("Mediccal diagnosis :{}", medical_diagnosis);
            println!("Village of residence :{}", village_of_residence);
            println!("Amount to be paid : N450,000");
        }
    }
    println!("Total number of patients processed: {}", patient_count);

     if patient_count == 100{
        break;
     }
   
}

}