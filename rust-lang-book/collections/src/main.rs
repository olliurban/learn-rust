use std::collections::*;

fn main() {
    
    
    //* Vectors - stores values of the same type contiguously in memory
    {
        let mut v: Vec<i32> = Vec::new();
        
        //* Add items to the vector
        v.push(1);
        v.push(2);
        v.push(3);
        
        //* The same can be achieved by an easier initialization
        let mut v: Vec<i32> = vec![1, 2, 3];

        //* Reading specific values through array-indexing
        //* Causes an error if the index is invalid
        let second: &i32 = &v[1];
        println!("Second element via array-index is {}", second);

        //* Its safer to use get and handle this case explicitly
        let second: Option<&i32> = v.get(1);
        match second {
            Some(&item) => println!("Second element via Vec::get is {}", item),
            None => ()
        }

        //* Ownership issues can come up if immutable refs are used after the vector is changed
        //* A change can mean that the allocated memory has been moved, rendering the reference invalid
        //* This a NEW borrow rule special to collections
        let _first = &v[0];  // immutable borrow on the memory
        v.push(4);          // v may grow and possibly changes memory layout 
        // ! println!("The first element is: {}", first);

        //* Iterating over the values 
        //* Changing the vector content requires mutability on the vector
        //* Changing an mutable referencec also requires dereferencing to access its value 
        for i in &mut v {
            *i = 10;
        }
        
        println!("Vector ");
        for i in &v {
            println!("\t({})", i);
        }

        //* To store more than one type use enums
        enum T {
            Int(i32),
            Float(f32),
            Text(String)
        }
        let _a: Vec<T> = vec![T::Int(20), T::Float(23.0), T::Text("Hello".to_string())];

    } // vector gets dropped including ALL its contents

    //* String - internally represented as a vector of u8 bytes
    {
        //* Both String and &str refer to character strings in UTF-8 encoding
        //* Aside from that there are specialized strings like OsString, OsStr

        //* Create a string
        let mut _s = String::new(); // empty string 
        let mut _s = String::from("Hello");
        let mut _s = "Hello".to_string();

        //* UTF-8 allows for a variety of symbols and characters
        String::from("السلام عليكم");
        String::from("Dobrý den");
        String::from("Hello");
        String::from("שָׁלוֹם");
        String::from("नमस्ते");
        String::from("こんにちは");
        String::from("안녕하세요");
        String::from("你好");
        String::from("Olá");
        String::from("Здравствуйте");
        String::from("Hola");

        //* Append to a string
        _s.push_str(", World");
        _s.push('!');
        println!("{}", _s); // "Hello, World!"

        //* Concatentation with '+'
        //* Implemented in String as:
        //*     fn add(self, &str) -> String {...}
        //* It takes ownership of the first argument and returns ownership of the concatenated string
        let a = "foo".to_string();
        let b = "bar".to_string();
        let _combined = a + &b; // Note that Rust coerced the &String into &str automatically

        //* Concatenation with 'format!'
        //* For when you combine multiple strings in a complex way
        //* It also doesn't take ownership of the parameters
        let a = "foo".to_string();
        let b = "bar".to_string();
        let _combined = format!("{} {}", a, b);

        //* Indexing strings encoded in UTF-8 is not supported by Rust 
        //* A UTF-8 symbol is not always 1 byte long, it can vary 
        //* As a result, a string with 10 characters can have a length of 20 
        //* This makes indexing unfavourable because it depends on the symbols in the string
        let s1 = String::from("hello");
        // ! let h = s1[0];

        //* Instead Rust gives you the option to slice exactly the part that you want
        //* The following code returns exactly the Зд part of the string
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{}", s); // "Зд"

        //* Latin strings should cause less problems and a more direct conversion
        let hello = "Hello";
        let s = &hello[0..4];
        println!("{}", s); // "Hell"

        //* Iteration over strings
        println!("Print नमस्ते as characters:");
        for c in "नमस्ते".chars() {
            println!("\t{}", c);
        }
        println!("Print नमस्ते as bytes:");
        for b in "नमस्ते".bytes() {
            println!("\t{}", b);
        }
    }

    //* Hash Maps - stores (key, value) pairs where the key is unique 
    //* The default hash function is cryptographically strong but can be changed
    {
        //* Create a new hash map that takes ids as a handle for texts
        let mut text_handle = HashMap::new();
        text_handle.insert(1, "germany".to_string()); // takes ownership of the string
        text_handle.insert(2, "france".to_string());
        text_handle.insert(3, "united kingdom".to_string());

        //* Combining two vecs into a has map
        let ids = vec![1, 2, 3];
        let countries = vec!["germany".to_string(), "france".to_string(), "united kingdom".to_string()];
        let mut _text_handle: HashMap<i32, String> = ids.into_iter().zip(countries.into_iter()).collect();

        //* We can also insert references into our map
        let mut text_handle = HashMap::new();
        text_handle.insert(1, "germany".to_string()); // stores a reference to the string
        text_handle.insert(2, "france".to_string());
        text_handle.insert(3, "united kingdom".to_string());

        //* Get access to the items in the map
        let country = text_handle.get(&1);
        match country {
            Some(_) => println!("I found the germans!"),
            None => ()
        }

        //* Iterate over the map
        for (key, value) in &text_handle {
            println!("({}, {})", key, value);
        }

        //* Overwrite existing values in map
         text_handle.insert(1, "uganda".to_string());

         //* Insert on non-existant key
         text_handle.entry(2).or_insert("spain".to_string());
         text_handle.entry(4).or_insert("usa".to_string());

         //* Update existing entry by modifying the value
        text_handle.entry(3)
            .and_modify(|val| {
                val.push_str(" of great britain");
            });
        println!("{:?}", text_handle);
    }


}
