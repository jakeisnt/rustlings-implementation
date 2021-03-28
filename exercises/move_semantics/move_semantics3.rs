// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

fn main() {
    let vec0 = Vec::new();

    // This implicitly creates a copy of the vector.
    let mut vec1 = fill_vec(vec0);

    // Does not work; vec0 has been moved into fill_vec and vec1,
    // so it can't be accessed outside of the vec1 reference.
    // println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);

    // vec1 now holds the only allowed reference to vec0,
    // so it can do whatever with it.
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Making the argument mutable here allows the vector added to be mutated.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
