use vstd::prelude::*;

verus! {

pub open spec fn shrink_option_nat(o: Option<nat>) -> Seq<Option<nat>> {
    match o {
        Option::None => Seq::empty(),
        Option::Some(n) => {
            let shrunk_inner = shrink_nat(n);
            Seq::new(shrunk_inner.len() + 1, |i: int|
                if i == 0 {
                    Option::None
                } else {
                    Option::Some(shrunk_inner[i - 1])
                }
            )
        }
    }
}

pub open spec fn shrink_nat(n: nat) -> Seq<nat> {
    if n == 0 {
        Seq::empty()
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            seq![0nat]
        } else {
            seq![0nat, half]
        }
    }
}


pub proof fn shrink_option_includes_none(n: nat)
    ensures shrink_option_nat(Option::Some(n)).len() > 0,
            shrink_option_nat(Option::Some(n))[0] == Option::<nat>::None
{
    let result = shrink_option_nat(Option::Some(n));
    assert(result.len() == shrink_nat(n).len() + 1);
    assert(result[0] == Option::<nat>::None);
}

} // verus!