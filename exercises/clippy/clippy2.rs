// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);

    // Interesting that a for loop over an optional type would still work,
    // getting the inside of the type.
    //
    // If let pattern matching is great though!
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
