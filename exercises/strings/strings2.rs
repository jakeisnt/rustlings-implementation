// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    // To create a string, you need to wrap it with the constructor or use .to_string()
    let word = String::from("green"); // Try not changing this line :)
    // Generally, it's passed as the unwrapped type rather than the wrapped one
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
