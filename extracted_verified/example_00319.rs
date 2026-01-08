use vstd::prelude::*;
fn main() {}
verus!{
pub fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32)
	requires
		N > 0,
		old(a).len() == N,
		old(sum).len() == 1,
	ensures
		sum[0] == 4 * N,
{
    /* code modified by LLM (iteration 4): Simple direct assignment to satisfy postcondition */
    sum.set(0, 4 * N);
}
}
