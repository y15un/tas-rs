#![allow(dead_code, unused_assignments, unused_mut, unused_variables)]

use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[test]
fn ts_basics() {
    {
        // ```ts
        // console.log("Hello, world!");
        // ```
        println!("Hello, world!");
    }
    {
        // ```ts
        // console.log(`2 + 2 = ${2 + 2}`); // Prints: 2 + 2 = 4
        // ```
        println!("2 + 2 = {}", 2 + 2); // Prints: 2 + 2 = 4
    }
    {
        // ```ts
        // console.assert(2 === 2); // Does nothing
        // console.assert(0 === 2); // Raises an exception
        // ```
        assert!(2 == 2); // Does nothing
                         // assert!(0 == 2); // Raises a panic
    }
    {
        // ```ts
        // throw Error("Error message goes here.");
        // ```
        // panic!("Error message goes here.");
    }
    {
        // ```ts
        // function add(a: number, b: number) {
        //     return a + b;
        // }
        // ```
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
    {
        // ```ts
        // let add = (a, b) => a + b;
        // ```
        let add = |a: i32, b: i32| a + b;
    }
    {
        // ```ts
        // function f() {
        //     var x = 1;
        //     {
        //         var x = 2;
        //     }
        //     console.log(x); // Prints 2
        // }
        // ```
        fn f() {
            let mut x = 1;
            {
                x = 2;
            }
            println!("{}", x); // Prints 2
        }
        f();
    }
    {
        // ```ts
        // function f() {
        //     let x = 1;
        //     {
        //         let x = 2;
        //     }
        //     console.log(x); // Prints 1
        // }
        // ```
        fn f() {
            let mut x = 1;
            {
                let mut x = 2;
            }
            println!("{}", x); // Prints 1
        }
        f();
    }
    {
        // ```ts
        // let a: Array<number> = [1, 2, 3];
        // ```
        let a: Vec<i32> = vec![1, 2, 3];
    }
    {
        // ```ts
        // let result = "hello world".match(/[a-z]+/);
        // console.assert(result[0] === "hello");
        // ```
        let result = Regex::new(r"^[a-z]+")
            .expect("(most likely) syntax error")
            .find_iter("hello world")
            .collect::<Vec<_>>();
        assert!(result[0].as_str() == "hello");
    }
    {
        // ```ts
        // class Pair {
        //     public first: number;
        //     public second: number;
        //
        //     constructor(first: number, second: number) {
        //         this.first = first;
        //         this.second = second;
        //     }
        // }
        // ```
        //
        // -- OR --
        //
        // ```ts
        // class Pair {
        //     constructor(public first: number,
        //                 public second: number) {}
        // }
        // ```
        struct Pair {
            pub first: i32,
            pub second: i32,
        }
        impl Pair {
            pub fn new(first: i32, second: i32) -> Self {
                Self { first, second }
            }
        }

        // ```ts
        // let origin = new Pair(0, 0);
        // ```
        let origin = Pair::new(0, 0);
    }
    {
        // ```ts
        // class Pair {
        //     static zero = 0;
        //
        //     static origin() {
        //         return new Pair(0, 0);
        //     }
        //
        //     constructor(public first: number,
        //                 public second: number) {}
        //
        //     toString() {
        //         return `(${this.first}, ${this.second})`;
        //     }
        // }
        // ```
        struct Pair {
            pub first: i32,
            pub second: i32,
        }
        impl Display for Pair {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "({}, {})", self.first, self.second)
            }
        }
        impl Pair {
            pub fn origin() -> Self {
                Self::new(0, 0)
            }

            pub fn new(first: i32, second: i32) -> Self {
                Self { first, second }
            }
        }
    }
}
