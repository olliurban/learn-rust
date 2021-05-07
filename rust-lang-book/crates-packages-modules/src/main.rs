        //* Typical Rust code is organized hierarchically
    //* We start introducing these constructs top-down

    //* A 'Package' typically encompasses your entire library or program
    //* It contains a cargo.toml for metadata and dependencies of your project
    //* The command "cargo new <project>" creates such a package

    //* The package consists of one or more 'Crates', as library or binaries
    // ! There must be at most one library crate in a package
    //* The crate encapsulates a set of functionalities
    //* Every crate has a source file as its root called 'crate root'
    //* The crate root serves as a starting point for "rustc" 
    //* For binary crates the default crate root is "src/main.rs"
    //* Further binary crates can be defined in the "src/bin" folder, e.g. "src/bin/bin-crate.rs"
    //* For library crates the crate root is "src/lib.rs"
    //* A crate can be shared by the Rust registry for others to use (see rand crate) 

    //* A crate consists of one more modules which also forms the 'module tree'
    //* The root of the tree is the crate root simply called 'crate' as represented by "src/main.rs" or "src/lib.rs"
    //* Crates represent a shareable unit of code in Rust
    //* In the cargo.toml you specify the crates you package depends on with: 
    //*     [dependencies]
    //*     <crate> = <version>
    //* 
    //* An example module tree is shown below
    mod education {
        pub mod school {
            pub fn do_school() {}
            pub fn do_learn() {}
            pub struct Plan {
                pub budget: i32,
                pub priorities: String
            }
        }
        mod university {
            mod computer_science {
                fn do_coding() {}
            }
        }

        pub mod night_school {
            pub struct Plan {
                pub budget: i32,
                pub priorities: String
            }
            pub mod special_degree {
                pub fn do_learn() {}
            }
        }
    }

fn main() {
    //* You can use common relations within the tree
    //* Since "crate" is the root module, it is the parent of "education" which in turn is the child of "crate"
    //* "school" and "university" are siblings

    //* The modules can be referred to by 'paths', relatively or absolutely
    //* Modules are inherintly private as are structs, we can change that by using the 'pub' modifier
    //* So this is why this works ...
    crate::education::school::do_school();  // absolute
    education::school::do_school();         // relative
    //* ... and this does not
    // ! crate::education::university::computer::science::do_coding();
    // ! education::university::computer::science::do_coding();

    //* You can also specify relative paths using super to refer to the parent module as a starting point
    {
        mod education {
            pub mod school {
                pub fn do_school() {
                    do_learn();
                    super::do_education();
                }
                pub fn do_learn() {}
            }
            fn do_education() {}
        }
    }

    //* We can bring other modules to our local scope with the 'use' command
    //* You should always prefer to 'use' the parent module of the function you want to include
    //* This makes it clear which scope the function is in
    {
        use education::school;
        school::do_school();
        school::do_learn();
    }

    //* Struct, enums and other items can be 'used' directly
    {
        use education::school::Plan;
        let plan = Plan {
            budget: 1_000_000,
            priorities: String::from("Redesign math class")
        };
    }

    //* If we want to use both Plan structs we run into a problem
    //* This will produce an ambigious reference 
    {
        use education::school::Plan;
        // ! use education::night_school::Plan; 
    }

    //* We can use the module ancestor that results in different paths for both structs
    let _school_plan = education::school::Plan {
        budget: 1_000_000,
        priorities: String::from("Redesign math class")
    };
    let _night_school_plan = education::night_school::Plan {
        budget: 600_000,
        priorities: String::from("Increase teacher salary")
    };

    //* Or we use the 'as' keyword to introduce the struct with another name
    {
        use education::school::Plan;
        use education::night_school::Plan as NPlan;
        let _plan = Plan {
            budget: 1_000_000,
            priorities: String::from("Redesign math class")
        };
        let _ns_plan = NPlan {
            budget: 600_000,
            priorities: String::from("Increase teacher salary")
        };
    }

    //* To expose an inner item to the external interface we can use 'pub use'
    //* This is also known as 're-exporting'
    //* Now other clients can use our school module in their code
    {
        pub use crate::education::school;
    }

    //* You can minimize redundancy in use declarations
    {
        use education::{school, night_school}; // we use ::school and ::night_school in the same line
        let _plan: Option<school::Plan> = None;
        let _ns_plan: Option<night_school::Plan> = None;
    } 

    //* It also works with including module nodes in a linear path
    {
        //* we use ::night_school and ::night_school::special_degree::do_learn in the same line
        use education::night_school::{self, special_degree::do_learn};
        let _plan: Option<night_school::Plan> = None;
        do_learn();
    }

    //* Of course, there is a glob operator include all items in a module
    {
        use education::night_school::*;
        let _plan: Option<Plan> = None;
        special_degree::do_learn();
    } 

    //* Rust programs can be separated into different files
    //* The convention is to structure the files in the same manner as the module tree
    //* Let's take the module tree at the top for example
    //* The corresponding file structure may look like this if we decide that each module has its own file
    //* See src/education.rs as an example ...
    //* 
    //* src/lib.rs (crate root)
    //* src/education.rs (mod education)
    //* src/education/school.rs (mod school)
    //* src/education/university.rs (mod university)
    //* src/education/night_school.rs (mod night_school)
    //* src/education/night_school/special_degree.rs (mod special_degree)
    //*
    //* Note that it is not neccessary to wrap each file into the corresponding module as its done automatically
}
