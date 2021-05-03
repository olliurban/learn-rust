

fn main() {

    //* In Rust enumerable types can be declared as 'enum'
    //* An enum can help to group associated types together
    {
        enum Building {
            School,
            Bank,
            PoliceStation
        }

        let _school = Building::School;
        let _bank = Building::Bank;
        let _police = Building::PoliceStation;
    }

    //* Like structs they can hold additional data and each variant can declare its own set of types 
    //* Any types such as integers, strings, structs or enums are valid 
    enum Building {
        School(String, String), // identified by city and name
        House(String, String, i32), // identified by address
        PoliceStation(String, i32) // identified by city and district number 
    }
    
    let _school = Building::School(String::from("Dayton"), String::from("Dayton School"));
    let _house = Building::House(String::from("Atlanta"), String::from("Washington Street"), 42);
    let _police = Building::PoliceStation(String::from("Atlanta"), 2);

    //* Enums also allow method implementations
    impl Building {
        fn tear_down(&self) {
            println!("Building goes 💥");
        }
    }
    _house.tear_down();

    //* Rust was carefully designed to prevent null values
    //* The equivalent type-safe idiom is an enum called 'Option<T>'
    //* An option can hold one of two values: 'None' and 'Some<T>'
    let _none: Option<i32> = None; // type inference doesn't work here because we don't provide a value
    let _some_number = Some(100);
    println!("some == none: {}", _none == _some_number);        // false
    println!("some == none: {}", None == _none);                // true
    println!("some == none: {}", Some(100) == _some_number);    // true
    println!("some == none: {}", Some(101) == _some_number);    // false

    //* The 'match' operator is crucial for Rust programs
    //* It brings pattern & enum variant matching to the table 
    //* Here we focus on enum variant matching
    {
        //* 'match' matches a value against a set of patterns
        //* Each pattern represents an 'arm' 
        //* An arm also declares the expressions evaluated if the pattern matches
        // ! Syntax: <pattern> => <expression>
        //* Because match itself is evaluated it delegates to the matching arm

        //* Here, 'e' represents the variable matched against First, Second, Third  
        //* Since e posses the value Enum::Second, the second arm is executed
        {
            enum Enum { First, Second, Third}
            let e = Enum::Second;
            match e {
                Enum::First     => println!("First!"),
                Enum::Second    => println!("Second!"),
                Enum::Third     => println!("Third!")
            }
        }
        
        //* We can also use multiple statements in one arm
        //* And provide a 'placeholder' arm that matches everything
        {
            enum Enum { First, Second, Third}
            let e = Enum::First;
            match e {
                Enum::First     => {
                    println!("First!");
                    println!("Easy Peasy");
                },
                _    => println!("Some other!"),
            }
        }
    }

    //* A pattern can convey additional details about the value
    //* Let's refer to the Building enum above and process its inner values upon match
    //* Note that the variable bindings become available for the corresponding code
    match _house {
        Building::School(city, name) => println!("{} at {}", name, city),
        Building::House(city, street, no) => println!("{} {}, {}", street, no, city),
        Building::PoliceStation(city, district) => println!("Police station {}, district {}", city, district)
    }

    //* A match allows us to handle Option<T> in the same way
    let name = Some("Hans");
    let no_name: Option<&str> = None;

    fn match_option(opt: Option<&str>) -> &str {
        match opt {
            Some(name) => name, 
            None => ""
        }
    }

    let _name = match_option(name); // _name == "Hans";
    let _empty = match_option(no_name); // _empty == ""

    //* For very specific patterns, match tends to inflate the code a lot
    //* The code below only wants to match a single value, otherwise do nothing
    let some_number = Some(3);
    match some_number {
        Some(1) => println!("One"),
        _ => println!("Not one")
    }

    //* The if-let construct allows us to simplify this
    //* If the value matches the pattern, execute the following block
    // ! Syntax: if let <pattern> = <expression> { <code> }
    if let Some(1) = some_number {
        println!("One");
    }

    //* An else branch can catch all other cases
    if let Some(1) = some_number {
        println!("One");
    } else {
        println!("Not one");
    }
}