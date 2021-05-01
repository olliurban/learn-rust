
//* Simple function definition
fn _simple_function() {

}

//* Function with parameters - MUST include types
fn _parametric_function(x: i32, y: i32) {
    println!("parameter 1 is {} and parameter 2 is {}", x, y);
}

//* Rust makes a distinction between expressions and functions
//* Statements have no computable value, so this ...
// ! let x = (let y = 6) 
//* will result in an error because there is no value provided for x
//* However, statements such as assignments contain expressions 

//* Functions can return values in two ways 
//* (1) by having an expression at the end of the block or (2) explicitly with 'return'
//* The return type must be added as '-> <type>' after the parameter list 
fn _returning_function_by_expr(num: i32) -> bool {
    num % 2 == 0 // Note the missing ';'
}
fn _returning_function_by_return(num: i32) -> bool {
    return num % 2 == 0;
}


fn main() {
    _simple_function();
    _parametric_function(12, 14);

    let even1 = _returning_function_by_expr(20);
    let even2 = _returning_function_by_return(20);

    assert_eq!(even1, even2);
}
