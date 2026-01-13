use vstd::prelude::*;

verus! {

pub struct Matrix { pub rows: nat, pub cols: nat, pub data: Seq<Seq<nat>> }

pub open spec fn valid_matrix(m: Matrix) -> bool {
    m.data.len() == m.rows && forall|i: nat| #![auto] i < m.rows ==> m.data[i as int].len() == m.cols
}

pub open spec fn identity(n: nat) -> Matrix {
    Matrix { rows: n, cols: n, data: Seq::new(n, |i: int| Seq::new(n, |j: int| if i == j { 1nat } else { 0nat })) }
}


pub proof fn identity_valid(n: nat) ensures valid_matrix(identity(n)) {}

} // verus!