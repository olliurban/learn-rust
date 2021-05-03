

fn main() {
    //* Structs are composite object structures that encapsulate related data
    //* To declare a struct you associate its name with a list of variable declarations
    struct Struct {
        _field1: i32,
        _field2: bool,
        _field3: String
    }
    
    //* Then you can instantiate it using new values ...
    let mut _my_struct = Struct {
        _field1: 1,
        _field2: false,
        _field3: String::from("s")
    };
    //* ... or existing ones
    let _my_second_struct = Struct {
        _field1: 10,
        _field2: _my_struct._field2,
        _field3: _my_struct._field3.clone()
    };

    //* Rust also offers 'field init shorthand' ...
    let _field1 = 20;
    let _field2 = true;
    let mut _shorthand_struct = Struct {
        _field1, // == _field1: _field1
        _field2, // == _field2: _field2
        _field3: String::from("normal")
    };
    //* ... or 'struct update syntax' as shorter initialization alternatives
    let mut _shorthand_struct = Struct {
        _field3: String::from("normal"),
        .._shorthand_struct // uses the previous struct to initialize the 'remaining' fields: _field1, _field2
    };
    
    //* you can access them in your program via 'dot operator'
    _my_struct._field1 = 12;
    if !_my_struct._field2 { _my_struct._field3.push_str("truct"); }

    //* Struct can also be used to give tuples a more descriptive name
    struct Color(i32, i32, i32);
    let _red = Color(255, 0, 0);

    //* An empty struct is called 'unit-like' and behaves similar to the 'unit type' ()
    //* It's useful for applying type traits without needing a data field
    struct Empty {}
    let _e = Empty {};

    //* Now if we want to print a struct we can experiment with a few options
    //* Unfortunately, structs cannot be printed by default
    //* They are missing the type trait 'std::fmt:Display' to be printed like that ...
    // ! println!("{}", _my_struct);  

    //* The compiler suggests the use of {:?} instead of {} for structs
    //* This will not work, since our struct doesn't implement the 'Debug' trait
    // ! println!("{:?}", _my_struct);
    
    //* Luckily, we can derive from the default trait that enables basic printing
    {
        #[derive(Debug)]
        struct Struct {
            num: i32,
            name: String
        }

        let printable = Struct {
            num: 12,
            name: String::from("Hans")
        };
        println!("{:?}", printable);

        //* For larger structs, we should use {:#?} instead of {:?}
        println!("{:#?}", printable); // this will put each member in a separate line
    }

    //* Struct methods can be implemented by writing ...
    {
        #[derive(Debug)]
        struct Book {
            title: String,
            author: String,
            year: u16
        }

        //* Provide one or more 'impl' blocks to implement methods
        //* 'self' refers to the invoking object
        //* Note that self is taken by reference, we only want to borrow not own the invoking object
        impl Book {
            fn get_banner(&self) -> String {
                format!("{} by {}, {}", self.title, self.author, self.year)
            }
            fn has_title(&self, title: &str) -> bool {
                self.title == title
            }
        }

        let harry_flotter = Book {
            title: String::from("Harry Flotter"),
            author: String::from("J.K. Rowling"),
            year: 2030
        };
        
        println!("{}", harry_flotter.get_banner());

        let other_title = "Harry Potter";
        println!("{:?} has title {}: {}", harry_flotter, other_title, harry_flotter.has_title(other_title));

        //* You can also implement 'associated function' that resemble static functions in C/C++
        //* Note the missing self parameter, the method is decoupled from an invoking object
        impl Book {
            fn new(title: String, author: String, year: u16) -> Book {
                Book {
                    title,
                    author,
                    year
                }
            }
        }
        let _moby_sick = Book::new(String::from("Moby Sick!"), String::from("Henry Dude"), 2030);



    }
}