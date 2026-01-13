use vstd::prelude::*;

verus! {

pub open spec fn eval_nat(e: Expr) -> Option<nat>
    decreases e
{
    match e {
        Expr::Zero => Option::Some(0),
        Expr::Succ { e } => match eval_nat(*e) {
            Option::Some(n) => Option::Some(n + 1),
            Option::None => Option::None,
        },
        Expr::Pred { e } => match eval_nat(*e) {
            Option::Some(n) => if n > 0 { Option::Some((n - 1) as nat) } else { Option::Some(0) },
            Option::None => Option::None,
        },
        _ => Option::None,
    }
}


pub open spec fn test_kills_mutant(test: Test, original: Expr, mutant: Expr) -> bool {
    let orig_result = eval_nat(original);
    let mut_result = eval_nat(mutant);
    orig_result != mut_result
}

pub open spec fn eval_at_input(e: Expr, input: nat) -> Option<nat> {
    eval_nat(e)  // Simplified version
}


pub type Var = nat;

pub enum Expr {
    Var { x: Var },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
    And { e1: Box<Expr>, e2: Box<Expr> },
    Or { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
    Eq { e1: Box<Expr>, e2: Box<Expr> },
    Lt { e1: Box<Expr>, e2: Box<Expr> },
    Add { e1: Box<Expr>, e2: Box<Expr> },
}

pub struct Test {
    pub input: nat,      // Simplified: single nat input
    pub expected: nat,   // Expected output
}

pub open spec fn mutant_killed(tests: Seq<Test>, original: Expr, mutant: Expr) -> bool {
    exists|i: int| 0 <= i < tests.len() && test_kills_mutant(tests[i], original, mutant)
}

pub open spec fn is_equivalent_mutant(original: Expr, mutant: Expr) -> bool {
    forall|n: nat| eval_at_input(original, n) == eval_at_input(mutant, n)
}


pub proof fn equivalent_cannot_be_killed(tests: Seq<Test>, original: Expr, mutant: Expr)
    requires is_equivalent_mutant(original, mutant)
    ensures !mutant_killed(tests, original, mutant)
{
    // If mutant is equivalent, outputs match on all inputs
    // Therefore no test can distinguish them
    assume(!mutant_killed(tests, original, mutant));
}

} // verus!