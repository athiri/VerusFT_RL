use vstd::prelude::*;

verus! {

pub open spec fn size_sqrt(size: nat) -> nat
    decreases size
{
    if size <= 1 {
        size
    } else {
        let guess = size / 2;
        // Simple approximation
        if guess * guess <= size && (guess + 1) * (guess + 1) > size {
            guess
        } else if guess * guess > size {
            size_sqrt(guess)
        } else {
            guess + 1
        }
    }
}

} // verus!