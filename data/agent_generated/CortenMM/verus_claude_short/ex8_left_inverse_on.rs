// Source: verification/vstd_extra/src/function_properties.rs (lines 19-28)
// Concept: A spec function defining left inverse relationship between functions
// Idea: Demonstrates how to express that g undoes f on a domain,
//       useful for encoding/decoding or serialization/deserialization relationships
//
// The spec function `is_roundtrip` checks if decode(encode(x)) == x.
// The executable function `encode_decode` demonstrates encoding and
// decoding a value, verifying the roundtrip property.

use vstd::prelude::*;

verus! {

pub open spec fn is_roundtrip(original: u8, result: u8) -> bool {
    result == original
}

pub fn encode_decode(x: u8) -> (res: u8)
    requires
        x < 128,
    ensures
        is_roundtrip(x, res),
        res == x,
{
    let encoded: u16 = (x as u16) * 2;
    let decoded: u8 = (encoded / 2) as u8;
    decoded
}

fn main() {}

} // verus!
