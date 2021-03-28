// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // It's good that macros have restricted syntax;
    // the macro flexibility of Lisp ends up becoming unreadable fast.
    my_macro!();
}
