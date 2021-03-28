// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// Oops! 'Slice' is the technical term for these 'unwrapped' strings.
// That makes sense - you're taking a slice of the value by dereferencing it,
// and can access just a part of the structure rather than a whole structure
// by doing some fancy pointer management in the background.
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // Surprised that this is a String! Guess you really have to read the docs here.
    // I suppose it makes sense for convenience though!
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
