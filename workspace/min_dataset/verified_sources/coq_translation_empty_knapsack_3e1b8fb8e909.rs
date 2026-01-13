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

pub open spec fn solve_knapsack(items: Seq<Item>, capacity: nat) -> nat { knapsack(items, capacity, 0) }


pub proof fn empty_knapsack(capacity: nat) ensures solve_knapsack(Seq::empty(), capacity) == 0 { reveal_with_fuel(knapsack, 2); }

} // verus!