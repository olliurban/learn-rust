use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

fn main() {

    //* panic! - unrecoverable error that causes the program to abort 
    {
        //* Panic occurs when accessing a vector at an invalid index 
        let v = vec![1, 2, 3, 4];
        // ! v[4]; // this will panic and throw an error

        //* We can also panic explicitly
        // ! panic!("Panicked on purpose.");
    }

    //* Result - Recoverable errors that is supposed to be handled by the programer
    //* Use when
    //*     (1) ... writing examples, tests, protoypes
    //*     (2) ... catching bugs that should usually not occur 
    //*     (3) ... there is no suitable type to represent the error
    {
        //* 'Result' is an enum that represents two outcomes
        //*     Ok(request_value)
        //*     Err(error_value)
        //* It is commonly returned in the standard library to mark a potentially failing method
        //* We can handle it 'match'
        let f: Result<File, std::io::Error> = File::open("hello.txt"); // file may not exist
        
        //* Now we may use the result if the file was indeed opened or abort on failure
        let f = match f {
            Ok(file) => file,
            Err(err) => panic!("Problem opening the file: {:?}", err)
        };

        //* The Error struct contains further information about what went wrong
        //* So let's create a file if it cannot be opened
        let f = File::open("hello.txt"); // file may not exist
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };

        //* This code can be shortened if we take advantage of Result's methods
        //* The 'unwrap_or_else' method returns the value if OK or lets us handle the error in a lambda 
        let f = File::open("hello.txt").unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|err| {
                    panic!("Problem creating the file: {:?}", err)
                })
            } else {
                panic!("Problem opening the file: {:?}", err)
            }        
        });

        //* If we do want to panic at an error we can use 'unwrap'
        let f = File::open("hello.txt").unwrap(); // may panic

        //* For more verbosity, we can use 'expect'
        let f = File::open("hello.txt").expect("File does not exist.");

    }
}

//* When a client cares about the error thrown, a function can 'propogate' the error to him/her
fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//* The example is even shorter when using the ?-operator after a call that returns a Result
//* It behaves in the same way like the previous example, returning Ok(file) on success and Err(e) on error
fn _read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

//* These method can also be chained
fn __read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}

//* You can even use the ?-operator in the main function
//* You need to add the return type as ...
//*     -> Result<(), Box<dyn Error>>
