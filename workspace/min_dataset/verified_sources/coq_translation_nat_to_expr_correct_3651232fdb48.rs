use vstd::prelude::*;

verus! {

pub open spec fn eval_bool(e: Expr) -> Option<bool>
    decreases e
{
    match e {
        Expr::Tru => Option::Some(true),
        Expr::Fls => Option::Some(false),
        Expr::And { e1, e2 } => match (eval_bool(*e1), eval_bool(*e2)) {
            (Option::Some(b1), Option::Some(b2)) => Option::Some(b1 && b2),
            _ => Option::None,
        },
        Expr::Or { e1, e2 } => match (eval_bool(*e1), eval_bool(*e2)) {
            (Option::Some(b1), Option::Some(b2)) => Option::Some(b1 || b2),
            _ => Option::None,
        },
        Expr::Not { e } => match eval_bool(*e) {
            Option::Some(b) => Option::Some(!b),
            Option::None => Option::None,
        },
        Expr::IsZero { e } => match eval_nat(*e) {
            Option::Some(n) => Option::Some(n == 0),
            Option::None => Option::None,
        },
        Expr::If { cond, then_br, else_br } => match eval_bool(*cond) {
            Option::Some(true) => eval_bool(*then_br),
            Option::Some(false) => eval_bool(*else_br),
            Option::None => Option::None,
        },
        _ => Option::None,
    }
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
    Add { e1: Box<Expr>, e2: Box<Expr> },
    Mul { e1: Box<Expr>, e2: Box<Expr> },
}

pub open spec fn nat_to_expr(n: nat) -> Expr
    decreases n
{
    if n == 0 {
        Expr::Zero
    } else {
        Expr::Succ { e: Box::new(nat_to_expr((n - 1) as nat)) }
    }
}

pub open spec fn eval_nat(e: Expr) -> Option<nat>
    decreases e
{
    match e {
        Expr::Zero => Option::Some(0nat),
        Expr::Succ { e } => match eval_nat(*e) {
            Option::Some(n) => Option::Some(n + 1),
            Option::None => Option::None,
        },
        Expr::Pred { e } => match eval_nat(*e) {
            Option::Some(n) => if n > 0 { Option::Some((n - 1) as nat) } else { Option::Some(0nat) },
            Option::None => Option::None,
        },
        Expr::Add { e1, e2 } => match (eval_nat(*e1), eval_nat(*e2)) {
            (Option::Some(n1), Option::Some(n2)) => Option::Some(n1 + n2),
            _ => Option::None,
        },
        Expr::Mul { e1, e2 } => match (eval_nat(*e1), eval_nat(*e2)) {
            (Option::Some(n1), Option::Some(n2)) => Option::Some(n1 * n2),
            _ => Option::None,
        },
        Expr::If { cond, then_br, else_br } => match eval_bool(*cond) {
            Option::Some(true) => eval_nat(*then_br),
            Option::Some(false) => eval_nat(*else_br),
            Option::None => Option::None,
        },
        _ => Option::None,
    }
}


pub proof fn nat_to_expr_correct(n: nat)
    ensures eval_nat(nat_to_expr(n)) == Option::Some(n)
    decreases n
{
    if n == 0 {
        assert(nat_to_expr(0) == Expr::Zero);
        assert(eval_nat(Expr::Zero) == Option::Some(0nat));
    } else {
        nat_to_expr_correct((n - 1) as nat);
        let inner = nat_to_expr((n - 1) as nat);
        assert(eval_nat(inner) == Option::Some((n - 1) as nat));
        assert(nat_to_expr(n) == Expr::Succ { e: Box::new(inner) });
    }
}

} // verus!