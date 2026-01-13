use vstd::prelude::*;

verus! {

pub struct Item { pub weight: nat, pub value: nat }


pub open spec fn knapsack(items: Seq<Item>, capacity: nat, i: nat) -> nat
    decreases items.len() - i
{
    if i >= items.len() { 0 }
    else if items[i as int].weight > capacity { knapsack(items, capacity, i + 1) }
    else {
        let skip = knapsack(items, capacity, i + 1);
        let take = items[i as int].value + knapsack(items, (capacity - items[i as int].weight) as nat, i + 1);
        if take > skip { take } else { skip }
    }
}

} // verus!