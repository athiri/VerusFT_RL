use vstd::prelude::*;

verus! {

pub open spec fn count_ways(coins: Seq<nat>, amount: nat, i: nat) -> nat
    decreases coins.len() - i + amount
{
    if amount == 0 { 1 }
    else if i >= coins.len() { 0 }
    else {
        let skip = count_ways(coins, amount, i + 1);
        if coins[i as int] > amount || coins[i as int] == 0 { skip }
        else {
            let new_amount = (amount - coins[i as int]) as nat;
            skip + count_ways(coins, new_amount, i)
        }
    }
}

} // verus!