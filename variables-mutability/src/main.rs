fn main() {
    
    //* variables are immutable by default ...
    let x = 5;
    //* ... thus variables cannot be reassigned
    // ! x = 6;
    println!("The value of x is {}", x);

    //* the 'mut' keyword marks the variable as mutable ...
    let mut x = 5;
    //* ... now it can be reassigned
    x = 6;
    println!("The value of x is {}", x);
    
    //* For more explicit immutablity declare as 'const'
    //* You MUST declare a type otherwise rust will throw an error
    //* Allowed values must be constant expression (evaluated at compile-time)
    const PI: f32 = 3.14; 

    //* Variables can be redeclared known as 'shadowing'
    //* We effectively create another variable
    //* Its for e.g. transforming an otherwise immutable variable and helps with naming
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
