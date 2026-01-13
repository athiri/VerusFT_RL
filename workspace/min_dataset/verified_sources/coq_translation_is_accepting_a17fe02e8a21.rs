use vstd::prelude::*;

verus! {

pub struct Transition { pub from: nat, pub to: nat, pub label: nat }

pub struct StateMachine { pub states: nat, pub transitions: Seq<Transition>, pub initial: nat, pub accepting: Seq<nat> }


pub open spec fn is_accepting(sm: StateMachine, s: nat) -> bool decreases sm.accepting.len() {
    sm.accepting.len() > 0 && (sm.accepting[0] == s || is_accepting(StateMachine { accepting: sm.accepting.skip(1), ..sm }, s))
}

} // verus!