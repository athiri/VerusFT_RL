use vstd::prelude::*;

verus! {

pub struct Matrix { pub rows: nat, pub cols: nat, pub data: Seq<Seq<nat>> }

pub open spec fn zero_matrix(r: nat, c: nat) -> Matrix {
    Matrix { rows: r, cols: c, data: Seq::new(r, |_i: int| Seq::new(c, |_j: int| 0nat)) }
}

pub open spec fn valid_matrix(m: Matrix) -> bool {
    m.data.len() == m.rows && forall|i: nat| #![auto] i < m.rows ==> m.data[i as int].len() == m.cols
}


pub proof fn zero_valid(r: nat, c: nat) ensures valid_matrix(zero_matrix(r, c)) {}

} // verus!