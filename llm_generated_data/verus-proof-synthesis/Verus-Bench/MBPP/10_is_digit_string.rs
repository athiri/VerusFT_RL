use vstd::prelude::*;

fn main() {
    // Write a function in Rust to check if a byte string represents only digits.

    assert!(is_digit_string(b"12345"));
    assert!(!is_digit_string(b"123a5"));
    assert!(is_digit_string(b"999"));
}

verus! {

spec fn is_digit_spec(c: u8) -> bool {
    c >= 48 && c <= 57
}

fn is_digit(c: u8) -> (result: bool)
    ensures
        result == is_digit_spec(c),
{
    c >= 48 && c <= 57
}

fn is_digit_string(text: &[u8]) -> (result: bool)
    ensures
        result == (forall|i: int| 0 <= i < text.len() ==> (#[trigger] is_digit_spec(text[i]))),
{
    let mut index = 0;
    while index < text.len()
        invariant
            0 <= index <= text.len(),
            forall|k: int| 0 <= k < index ==> (#[trigger] is_digit_spec(text[k])),
        decreases text.len() - index,
    {
        if !is_digit(text[index]) {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!
