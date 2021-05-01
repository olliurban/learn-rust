use rand::Rng;

// Pick a number from [a,b): random!(0, 101) --> number between 0 and 100
macro_rules! random {
    ($a:expr, $b:expr) => {
        rand::thread_rng().gen_range($a, $b)
    };
}

fn main() {

    //* Note the v data type below ...
    let guess: u32 = "42".parse().expect("Not a number");
    println!("My guess is {}", guess);

    //* ... without it, rust cannot infer the correct type 
    // ! let guess = "42".parse().expect("Not a number");

    //* There are multiple integer types with 1, 2, 4, 8 byte(s)
    let _unsigned_int_8_bit: u8;
    let _signed_int_8_bit: i8;
    // ...
    let _unsigned_int_128_bit: u128;
    let _signed_int_128_bit: i128;
    
    //* For architecture-specific types, e.g. 32 or 64 bit, use
    let _unsigned_arch_int: usize;
    let _signed_arch_int: isize;

    //* The integer literals can be expressed in bases 2, 8, 10 and 16 ...
    let _binary = 0b011011010;
    let _oct = 0o112;
    let _decimal = 15000;
    let _hex = 0xabcde;

    //* ... and can contain camel case as well.
    let _camel_case_int = 15_000;

    //* Although Rust types integer literals as 'u32', bytes ('u8') can be used as ...
    let _byte = b'N';

    //* Overflows are checked in debug mode causing a runtime error
    //* In release mode the runtime checks are inactive
    //* So this would produce a runtime error in debug but not in release mode 
    // ! let _invalid_overflow: u8 = random(250, 251) + 6;

    //* If you know what you're doing, handle them explicitly!
    let _wrapped        = u8::wrapping_add(random!(250, 251), 6);
    let _overflow       = u8::checked_add(random!(250, 251), 6);
    let _overflowing    = u8::overflowing_add(random!(250, 251), 6);
    let _saturating     = u8::saturating_add(random!(250, 251), 6);

    println!("250 + 6 is {} for a byte - wrapped_add",          _wrapped);
    println!("250 + 6 is an overflow: {} - checked_add",        _overflow == Option::None);
    println!("250 + 6 is {}, overflow? {} - overflowing_add",   _overflowing.0, _overflowing.1);
    println!("250 + 6 saturates at {} - saturating_add",        _saturating);

    //* Floating-point types (IEEE-754 standard) come in two flavors and you can write them as ...
    let _float_32_bit: f32 = 2.0;
    let _float_64_bit: f64 = 3.0;

    //* Rust supports basic numerical operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    //* The 'bool' type represents binary logic
    let _t = true;
    let _f: bool = false;

    //* Rust utilizes 4-byte Unicode characters 
    let _c: char = 's';
    let _omega: char = 'Ω';
    println!("The character for omega is {}", _omega);

    //* A tuple is an ordered set of values with 'distinct' types
    let _tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tuple;
    println!("The first tuple element is {} by destructuring", _x);
    println!("The first tuple element is {} by dot operator", _tuple.0);

    //* An array is an ordered set of values of a 'single' type
    let _arr1 = [1, 2, 3, 4, 5];
    let _arr2: [i32; 5] = [1, 2, 3, 4 ,5];
    
    //* If you want to initialize an array with the same values you can use ...
    let _init_arr = [10; 5]; // [10, 10, 10, 10, 10]

    //* Access your array through the []-operator (zero-based)
    let _first = _init_arr[0];
    println!("The first value of the _initArr is {}", _first);

    //* Upon accessing an invalid index in your array, Rust throws an error like so
    let _too_high = random!(5, 6);
    // ! let _invalid_access = _init_arr[_too_high];
}
