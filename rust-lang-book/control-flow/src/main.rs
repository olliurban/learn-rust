

fn main() {

    //* Similar to C, you can alter the control flow of your program using 'if', 'while', 'for'
    //* Rust also adds 'loop' as an unconditional loop
    // ! < Keep in mind that Rust is expression-based, so these constructs can be evaluated like any other expressions >

    //* Rust uses typical 'if', 'else if', 'else' to create different branches of execution
    //* Note that the conditional expression must not be in parenthesis
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //* As noted above, 'if' can be used to return a value for e.g. assignments
    let koenig = "koenig";
    let _i = if koenig.len() < 5 { 1 } else { 0 };

    //* A simple 'loop' expression repeats the code indefinetly
    //* So this would continue running forever ...
    // ! loop {
    // !    println!("Hallo");
    // ! }
    //* While this would break out of the loop after 5 iterations
    let mut j = 0;
    loop {
        println!("'Hallo, Welt' using 'loop'");
        j = j+1;
        if j == 5 { break }
    }

    //* As an assignment, it would look like this ...
    let mut k = 4;
    let mut factorial = 1;
    let factorial = loop {
        factorial = factorial*k;
        k = k-1;
        if k == 0 { break factorial; }
    };
    println!("Factorial of {} is {} using a 'loop'", 4, factorial);

    //* The last program can become even more compact using a 'while' loop ...
    let mut k = 4;
    let mut factorial = 1;
    while k > 0 {
        factorial = factorial*k;
        k = k-1;
    };
    println!("Factorial of {} is {} using a 'while' loop", 4, factorial);

    //* Another way to compute factorials is through a 'for' loop ...
    let mut factorial = 1;
    for k in (1..5).rev() {
        factorial = factorial*k;
    };
    println!("Factorial of {} is {} using a 'for' loop", 4, factorial);

    //* A 'for' loop provides easy access to list-like structures
    let _arr = [1, 2, 4, 8, 16];
    for elem in _arr.iter() {
        println!("Array content: {}", elem);
    }

}
