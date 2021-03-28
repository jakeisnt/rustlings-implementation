// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // Need & to borrow the reference, which allows us to take a slice of it.
    // Without '&', it's a boxed value - which we cannot take a slice of.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
