// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // Instantiate a new vector
    let vec0 = Vec::new();

    // Give mutable reference to vec2 to vec2. vec2 is 'borrowing' vec2
    let mut vec2 = &vec0;

    // fill_vec takes ownership of the vector made by vec2.to_vec(),
    // but this is fine because it's a copy rather than the same reference.
    let mut vec1 = fill_vec(vec2.to_vec());

    // Returns a copy of vec2 with a single element added
    vec2.to_vec().push(20);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);



    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);


    println!("{} has length {} content `{:?}`", "vec1", vec2.len(), vec2);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
