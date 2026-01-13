use vstd::prelude::*;

verus! {

pub proof fn lemma_add_succ_r(n: nat, m: nat)
    ensures add(n, m + 1) == add(n, m) + 1
    decreases n
{
    if n == 0 {
    } else {
        lemma_add_succ_r((n - 1) as nat, m);
    }
}

pub proof fn lemma_add_0_r(n: nat)
    ensures add(n, 0) == n
    decreases n
{
    if n == 0 {
    } else {
        lemma_add_0_r((n - 1) as nat);
    }
}

pub proof fn lemma_add_comm(n: nat, m: nat)
    ensures add(n, m) == add(m, n)
    decreases n
{
    if n == 0 {
        // add(0,m)=m and add(m,0)=m
        lemma_add_0_r(m);
    } else {
        // IH: add(n-1, m) = add(m, n-1)
        lemma_add_comm((n - 1) as nat, m);

        // Need: add(n, m) = add(m, n)
        // LHS: add(n,m) = add(n-1,m)+1
        // RHS: add(m,n) = add(m,n-1)+1
        lemma_add_succ_r(m, (n - 1) as nat);
    }
}


pub proof fn lemma_add_comm_alt(n: nat, m: nat)
    ensures add(n, m) == add(m, n)
    decreases m
{
    if m == 0 {
        lemma_add_0_r(n);
    } else {
        lemma_add_comm_alt(n, (m - 1) as nat);
        lemma_add_succ_r(n, (m - 1) as nat);
        // add(m,n) unfolds on m, so we also need succ on the other side
        // via commutativity on smaller m:
        lemma_add_comm((m - 1) as nat, n);
    }
}

} // verus!