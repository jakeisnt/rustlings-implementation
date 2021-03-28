// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

// Looks like macros have subtle matching of values.
//
// Looks like there's a metavariable matching sublanguage:
// https://doc.rust-lang.org/reference/macros-by-example.html.
// Very cool, but I wish I didn't have to learn so much to get things to work!
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
