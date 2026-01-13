use vstd::prelude::*;

verus! {

pub open spec fn kmp_match_fuel(text: Seq<nat>, pattern: Seq<nat>, ti: nat, pi: nat, fail: Seq<nat>, fuel: nat) -> Option<nat>
    decreases fuel
{
    if fuel == 0 { None }
    else if pattern.len() == 0 { Some(0) }
    else if ti >= text.len() { None }
    else if pi >= pattern.len() { Some((ti - pi) as nat) }
    else if ti < text.len() && pi < pattern.len() && text[ti as int] == pattern[pi as int] {
        kmp_match_fuel(text, pattern, ti + 1, pi + 1, fail, (fuel - 1) as nat)
    }
    else if pi > 0 && pi <= fail.len() {
        let new_pi = if (pi - 1) < fail.len() { fail[(pi - 1) as int] } else { 0 };
        kmp_match_fuel(text, pattern, ti, new_pi, fail, (fuel - 1) as nat)
    }
    else {
        kmp_match_fuel(text, pattern, ti + 1, 0, fail, (fuel - 1) as nat)
    }
}

} // verus!