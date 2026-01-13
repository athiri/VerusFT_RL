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


pub open spec fn count_killed(tests: Seq<Test>, original: Expr, mutants: Seq<Expr>) -> nat
    decreases mutants.len()
{
    if mutants.len() == 0 {
        0
    } else {
        let killed = if mutant_killed(tests, original, mutants.last()) { 1nat } else { 0nat };
        killed + count_killed(tests, original, mutants.drop_last())
    }
}

} // verus!