use vstd::prelude::*;

verus! {

pub open spec fn simplify_bool(e: Expr) -> Expr
    decreases e
{
    match e {
        Expr::And { e1, e2 } => {
            let s1 = simplify_bool(*e1);
            let s2 = simplify_bool(*e2);
            match (s1, s2) {
                // false && _ = false
                (Expr::Fls, _) => Expr::Fls,
                (_, Expr::Fls) => Expr::Fls,
                // true && e = e
                (Expr::Tru, e) => e,
                (e, Expr::Tru) => e,
                // Otherwise, keep simplified
                (e1, e2) => Expr::And { e1: Box::new(e1), e2: Box::new(e2) },
            }
        }

        Expr::Or { e1, e2 } => {
            let s1 = simplify_bool(*e1);
            let s2 = simplify_bool(*e2);
            match (s1, s2) {
                // true || _ = true
                (Expr::Tru, _) => Expr::Tru,
                (_, Expr::Tru) => Expr::Tru,
                // false || e = e
                (Expr::Fls, e) => e,
                (e, Expr::Fls) => e,
                // Otherwise, keep simplified
                (e1, e2) => Expr::Or { e1: Box::new(e1), e2: Box::new(e2) },
            }
        }

        Expr::Not { e } => {
            let s = simplify_bool(*e);
            match s {
                // not true = false
                Expr::Tru => Expr::Fls,
                // not false = true
                Expr::Fls => Expr::Tru,
                // not (not e) = e
                Expr::Not { e: inner } => *inner,
                // Otherwise, keep simplified
                other => Expr::Not { e: Box::new(other) },
            }
        }

        Expr::If { cond, then_br, else_br } => {
            let sc = simplify_bool(*cond);
            match sc {
                // if true then t else e = t
                Expr::Tru => simplify_bool(*then_br),
                // if false then t else e = e
                Expr::Fls => simplify_bool(*else_br),
                // Otherwise, simplify branches
                _ => Expr::If {
                    cond: Box::new(sc),
                    then_br: Box::new(simplify_bool(*then_br)),
                    else_br: Box::new(simplify_bool(*else_br)),
                },
            }
        }

        other => other,
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


pub open spec fn simplify_nat(e: Expr) -> Expr
    decreases e
{
    match e {
        Expr::Add { e1, e2 } => {
            let s1 = simplify_nat(*e1);
            let s2 = simplify_nat(*e2);
            match (s1, s2) {
                // 0 + e = e
                (Expr::Zero, e) => e,
                (e, Expr::Zero) => e,
                // Otherwise, keep simplified
                (e1, e2) => Expr::Add { e1: Box::new(e1), e2: Box::new(e2) },
            }
        }

        Expr::Mul { e1, e2 } => {
            let s1 = simplify_nat(*e1);
            let s2 = simplify_nat(*e2);
            match (s1, s2) {
                // 0 * e = 0
                (Expr::Zero, _) => Expr::Zero,
                (_, Expr::Zero) => Expr::Zero,
                // 1 * e = e (where 1 = succ(zero))
                (Expr::Succ { e: inner }, other) if *inner == Expr::Zero => other,
                (other, Expr::Succ { e: inner }) if *inner == Expr::Zero => other,
                // Otherwise, keep simplified
                (e1, e2) => Expr::Mul { e1: Box::new(e1), e2: Box::new(e2) },
            }
        }

        Expr::Pred { e } => {
            let s = simplify_nat(*e);
            match s {
                // pred(0) = 0
                Expr::Zero => Expr::Zero,
                // pred(succ(e)) = e
                Expr::Succ { e: inner } => *inner,
                // Otherwise, keep simplified
                other => Expr::Pred { e: Box::new(other) },
            }
        }

        Expr::Succ { e } => {
            let s = simplify_nat(*e);
            Expr::Succ { e: Box::new(s) }
        }

        Expr::If { cond, then_br, else_br } => {
            let sc = simplify_bool(*cond);
            match sc {
                Expr::Tru => simplify_nat(*then_br),
                Expr::Fls => simplify_nat(*else_br),
                _ => Expr::If {
                    cond: Box::new(sc),
                    then_br: Box::new(simplify_nat(*then_br)),
                    else_br: Box::new(simplify_nat(*else_br)),
                },
            }
        }

        other => other,
    }
}

} // verus!