std::io
println!("Please your name.");
let mut name = String::new()
io::stdin
.read_line(&mut name);
.expect("Input invalid");
println!("My name is {}",name);

