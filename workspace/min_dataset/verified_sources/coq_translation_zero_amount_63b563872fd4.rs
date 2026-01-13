use vstd::prelude::*;

verus! {

pub open spec fn solve_coins(coins: Seq<nat>, amount: nat) -> Option<nat> {
    min_coins(coins, amount, 0)
}

pub open spec fn min_coins(coins: Seq<nat>, amount: nat, i: nat) -> Option<nat>
    decreases coins.len() - i + amount
{
    if amount == 0 { Some(0) }
    else if i >= coins.len() { None }
    else {
        let skip = min_coins(coins, amount, i + 1);
        if coins[i as int] > amount || coins[i as int] == 0 { skip }
        else {
            let new_amount = (amount - coins[i as int]) as nat;
            // Recursive call with smaller amount
            let take = match min_coins(coins, new_amount, i) {
                None => None,
                Some(n) => Some(n + 1),
            };
            match (skip, take) {
                (None, t) => t,
                (s, None) => s,
                (Some(s), Some(t)) => if s < t { Some(s) } else { Some(t) },
            }
        }
    }
}


pub proof fn zero_amount(coins: Seq<nat>)
    ensures solve_coins(coins, 0) == Some(0nat)
{
    reveal_with_fuel(min_coins, 2);
}

} // verus!