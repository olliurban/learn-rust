fn main() {

    //* Rust has two spaces to allocate memory into
    //* Like other programming languages, there is a stack and a heap for storing values and objects
    //* The stack stores fixed-size values known at compile-time e.g. a string literal 
    //* The heap is responsible for storing dynamically-sized values only known at runtime 

    //* Rust has its own notion of memory 'ownership'
    //* Ownership of memory allocated objects is strictly controlled by the ownership rules
    // ! (1) Each value is "owned" by EXACTLY ONE variable  
    // ! (2) When the owner leaves its scope, the value is "dropped"

    //* Objects on the stack are dropped when leaving their respective block 
    //* Note that the string literal has a fixed size and is immutable in Rust 
    //* Any operation on it creates a new value on the stack (copy-by-default)
    {
        let _str_stack = "Hallo, Welt";
        let _str_stack_copy = _str_stack;
    } // at this point _str_stack and _str_stack_copy are "dropped", and thus invalid

    //* Objects on the heap are deallocated once their owner gets out of scope
    //* When that happens Rust calls a special function to "drop" the value
    //* A string object is mutable, any operations may change the same object
    {
        let mut _str_heap = String::from("Hallo, Welt");
        _str_heap.push_str("!");

    } // at this point _str_heap is "dropped", and thus invalid

    //* Problems may arise when multiple variables point to the same value on the heap
    //* It is unclear when the memory should be dropped upon leaving the scope, a common problem in C/C++
    //* In Rust, an assignment automatically transfers ownership (move-by-default) according to rule (1)
    {
        let s1 = String::from("text");
        let _s2 = s1; // s1 has been moved into s2 and is no longer valid

        //* any operations on s1 will lead to an error
        // ! println!("{}", s1);
    } // at this point s1 and s2 go out of scope, s1 is already invalid, s2 gets dropped

    //* If you do want to copy objects on the heap, Rust has a notion of clonable types
    //* This is realized through the type trait 'Drop' (are handled later)
    {
        let s1 = String::from("text");
        let _s2 = s1.clone(); // s1 has been moved into s2 and is no longer valid

        //* now any operations on s1 will work as it has its own memory allocated
        println!("{}", s1);
    }

    //* The other type trait that lets you copy values is called 'Copy'
    //* Primitive types like ints or string literals are annotated by this trait
    //* More complex types can also be declared as copyable if their components also adhere to 'Copy'
    {
        let tuple = (1, "hallo");
        let _tuple_copy = tuple;
        println!("tuple({}, {})", tuple.0, tuple.1);
    }

    //* For functions, the same rules apply
    {
        let s = String::from("Hallo");
        takes_ownership(s);

        //* This will no longer work
        // ! s.push_str(", Welt");

        let mut s = String::from("Hallo");
        s = takes_and_returns_ownership(s);

        //* As we regained ownership, we can now manipulate the string
        s.push_str("Welt");
        println!("{}", s);
    }

    //* If you only want to use a variable within a function without transferring ownership, you can use 'references'
    //* They mark an object as only being "borrowed" by another variable using & as a prefix
    //* Note that the variable and the targeted parameter type must adhere to the prefix
    {
        let s = String::from("Hallo");
        let size = calculate_length(&s);

        //* Now s is still valid because we the function only borrowed s without taking ownership
        println!("{} has {} characters", s, size);
    }

    //* References are mutable by default and also introduce three more rules
    // ! Given a variable with ownership over an object
    // ! (3) There can be any number of immutable references
    // ! (4) There can only exist one mutable reference to the object
    // ! (5) An immutable reference to the object prevents the use of mutable ones  
    {
        //* This will fail, Rust doesn't want to allow possible race conditions
        let mut s = String::from("Hallo");
        let _r1 = &mut s;
        let _r2 = &mut s;
        // ! println!("{}, {}", _r1, _r2);
    }
    {
        //* This works - the lifetime of the variables don't overlap
        let mut s = String::from("Hallo");
        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
        // r1 and r2 are no longer used, their effective scope ends here 

        let r3 = &mut s; 
        println!("{}", r3);
    }

    //* Rust also detects dangling references
    //* So that this function cannot return a reference on a dynamic object leaving its scope
    // ! fn returns_dangling_str() -> &String {
    // !     let str = String::from("");
    // !     &str
    // ! } // at this point the value owned by str is dropped - the returned reference is dangling!

    //* If you want to reference only a part of your data without modifying it, 'Slices' are the way to go
    //* As a result, parts of strings or arrays are accessible as a slice using the notation
    //* &<variable>[lower..upper] 
    {
        let hallo = String::from("hallo");
        println!("First three character of {} are {}", hallo, first_3_character(&hallo));

        //* The string slice type is '&str' and also accepts values of type &String or string literals 
        let hallo = String::from("hallo"); 
        let _hallo_slice: &str = &hallo[0..3];
        let _hallo_slice: &str = "hallo";
    } 
}


fn takes_ownership(s: String) {
    s.len();
}

fn takes_and_returns_ownership(mut s: String) -> String {
    s.push_str(", ");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_3_character(s: &str) -> &str {
    if s.len() < 3 {
        &s
    } else {
        &s[0..3]
    }
}