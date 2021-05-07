//* We will work along a sorting function as the subject for this chapter
fn bubble_sort(a: &Vec<i32>) -> Vec<i32> {
    let mut res = a.clone();

    for i in 0 .. res.len()-1 {
        for j in 0 .. res.len()-i-1 {
            if res[j] > res[j+1] {
                res.swap(j, j+1);
            }
        }
    }

    res
}


fn main() {

    //* First, lets test our bubble sort implementation
    let unordered = vec![10, 8, 6, 2, 1, 7, 9, 4, 5, 3];
    println!("unordered set after sort: {:?}", bubble_sort(&unordered));

    let ordered = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("ordered set after sort: {:?}", bubble_sort(&ordered));

    //* Generics - abstract types that allow substitution with concrete ones 
    //* 
    //* BIG NOTE:
    //* Generics have much in common with C++ templates:
    //*     - Code is generated at compile-time
    //*     - Generic types can be further specialized by 'Traits' and concrete type specialization
    //*
    //* However, with generics you can also declare method for a specific types
    //* The method will thus only show up if your instance has the correct type
    {
        //* The generic version for bubble sort
        fn _bubble_sort<T>(_a: &Vec<T>) -> Vec<T> { vec![] }

        //* Unfortunately, this part would not compile 
        //* Because it is not clear what greater than means for a generic type
        //* More later ...
        // ! if res[j] > res[j+1] {
        // !     res.swap(j, j+1);
        // ! }

        //* We define a generic types for a struct using <...>
        struct Point<T, S> {
            x: T,
            y: S,
        }

        //* The impl block is annotated as well
        //* Note that the you provide the types twice in this generic case
        impl<T,S> Point<T,S> {
            fn as_tuple(&self) -> (&T,&S) {
                (&self.x, &self.y)
            }
        }

        //* Generic specialization allows to provide finer control over the exposed API
        //* You can implement the distance method only for Point types with <f32, f32>
        impl Point<f32, f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            } 
        }

        //* Then we can instantiate the struct as ...
        let _origin_mixed: Point<f32, i32> = Point { x: 0.0, y: 0 };
        let _origin_float: Point<f32, f32> = Point { x: 0.0, y: 0.0 };
        let _shorter = Point { x: 0.0, y: 0.0 };
        let _point_tuple = _shorter.as_tuple();
        
        //* The specialized function defined above will now only work on Point<f32, f32>
        _origin_float.distance_from_origin();
        // ! _origin_mixed.distance_from_origin();


        // Generic enums, we've already seen Option<T> and Result<T, E>
        enum _Option<T> {
            Some(T),
            None
        }
        enum _Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    //* Traits - expected functionality provided by generic types similar to 'interfaces'
    //* Note that can only be implemented in the crate they were defined
    {
        pub enum Message { Hi, RequestAttendance, Bye}

        //* Define a trait
        pub trait Sender {
            fn send(&self) -> Message;
        }
        pub trait Receiver {
            fn receive(&self, msg: &Message);
        }

        pub struct Bob {}
        pub struct SchoolBoard {}

        //* Let clients implement the trait as part of their interface
        impl Sender for SchoolBoard {
            fn send(&self) -> Message {
                Message::Hi
            }
        }
        impl Receiver for Bob {
            fn receive(&self, msg: &Message) {
                match msg {
                    Message::Hi => println!("Good afternoon, mister and misses."),
                    Message::RequestAttendance => println!("I'll be there."),
                    Message::Bye => println!("Ciao."),
                }
            }
        }

        let bob = Bob {};
        let school_board = SchoolBoard {};
        
        let msg = school_board.send();
        bob.receive(&msg);

        //* Default implementation for type traits
        {
            pub trait Sender {
                fn send(&self) -> Message {
                    Message::Hi
                }
            }
            pub trait Receiver {
                fn receive(&self, msg: &Message) {
                    // do nothing
                }
            }
        }

        //* The real strength of traits is that you can use them as generic substitutes
        //* For methods, trait parameters can be used by all types that adhere to the trait contract
        //* There are two approaches to declare trait parameters
        //* Firstly, there is 'impl trait', a short-hand for the second option
        pub fn issue_message(sender: &impl Sender) -> Message {
            sender.send()
        }
        pub fn complex_interaction(partner: &(impl Sender + Receiver)) {
            partner.receive(&partner.send());
        }

        //* Secondly, you can use the 'trait bound' syntax which is what the compiler actually understands 
        pub fn _issue_message<T: Sender>(sender: &T) -> Message {
            sender.send()
        }
        pub fn _complex_interaction<T: Sender + Receiver>(partner: &T) {
            partner.receive(&partner.send());
        }

        //* A more clear distinction can be provided by using 'when'
        pub fn __complex_interaction<T>(partner: &T) 
            where T: Sender + Receiver
        {
            partner.receive(&partner.send());
        }

        //* Traits as return types
        pub fn create_sender() -> impl Sender {
            SchoolBoard {}
        } 

        //* However, it's not allowed to return more than one concrete type of value
        //* The Rust compiler prevents this behaviour
        pub fn _create_sender(organization: bool) -> impl Sender {
            // ! if organization {
                SchoolBoard {}
            // ! } else {
            // !     Bob {}
            // ! }
        } 

        //* With these tools we can make our sorting function truly generic
        {
            fn bubble_sort<T: PartialOrd + Clone>(a: &Vec<T>) -> Vec<T> {
                let mut res = a.clone();

                for i in 0 .. res.len()-1 {
                    for j in 0 .. res.len()-i-1 {
                        if res[j] > res[j+1] {
                            res.swap(j, j+1);
                        }
                    }
                }

                res
            }

            let nums = vec![10, 8, 6, 2, 1, 7, 9, 4, 5, 3];
            println!("unordered set after sort: {:?}", bubble_sort(&nums));
        
            let chars = vec!['g', 'z', 'b', 'q', 'p', 'ö', 'a', 'c', 'd', 'e'];
            println!("ordered set after sort: {:?}", bubble_sort(&chars));
        
        }

        //* Traits can be specialized in the same manner as concrete types did
        //* This is again conditionally implemented
        struct Point<T, S> {
            x: T,
            y: S,
        }
        impl<T: Eq, S: Eq> Point<T, S> {
            fn is_equal(&self, other: Point<T, S>) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        let p1 = Point { x: 2, y: 5};
        let p2 = Point { x: 2, y: 5};
        assert!(p1.is_equal(p2));

        let p1 = Point { x: 2, y: 6};
        let p2 = Point { x: 2, y: 5};
        assert!(!(p1.is_equal(p2)));

        //* Blanket implementation - add functionality to all types that satisfy a certain trait bound
        //* In simpler terms:
        //*     A blanket implementation specifies a condition in the form of traits TR[]
        //*     Every type T which adheres to the traits in TR[] is covered
        //*     In the body you define methods that are bound to the covered types T 
        //*
        //* Example:
        pub trait CanSend {
            fn can_send();
        }

        impl<T: Sender> CanSend for T+S {
            fn can_send() {
                
            }
        }
    }
}
