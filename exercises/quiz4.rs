// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!


// This works, but it's corny. Let's see if we can do better.
// #[macro_use]
// mod macros {
//     macro_rules! my_macro {
//         ("world!") => {
//             "Hello world!"
//         };
//         ("goodbye!") => {
//             "Hello goodbye!"
//         };
//     }
// }


#[macro_use]
mod macros {
    macro_rules! my_macro {
        // Macros can use other macros - nice!
        // Can't forget the '$' metavariable references.
        ($arg:expr) => {
            format!("Hello {}", $arg)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
