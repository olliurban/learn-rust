fn main() {

    // Note the v data type below ...
    let guess: u32 = "42".parse().expect("Not a number");
    println!("My guess is {}", guess);

    // ... without it, rust cannot infer the correct type 
    // let guess = "42".parse().expect("Not a number"); --> compiler error
}
