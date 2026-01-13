use vstd::prelude::*;

verus! {

pub open spec fn mod_pow(base: nat, exp: nat, m: nat) -> nat decreases exp {
    if m == 0 { 0 }
    else if exp == 0 { (1nat % m) as nat }
    else { ((base * mod_pow(base, (exp - 1) as nat, m)) % m) as nat }
}

} // verus!