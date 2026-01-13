use vstd::prelude::*;

fn main() {}
verus! {

fn absolute_value(x: i32) -> (result: u32)
    requires
        x > i32::MIN,
    ensures
        x >= 0 ==> result == x as u32,
        x < 0 ==> result == (-x) as u32,
        result as int == if x >= 0 { x as int } else { -x as int },
{
    if x >= 0 {
        x as u32
    } else {
        (-x) as u32
    }
}

} // verus!
