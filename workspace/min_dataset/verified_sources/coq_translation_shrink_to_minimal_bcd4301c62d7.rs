use vstd::prelude::*;

verus! {

pub open spec fn find_smaller_counterexample(shrinks: Seq<Expr>, prop: spec_fn(Expr) -> bool, default: Expr) -> Expr
    decreases shrinks.len()
{
    if shrinks.len() == 0 {
        default  // No smaller counterexample found
    } else {
        let first = shrinks[0];
        if is_counterexample(first, prop) {
            first  // Found a smaller counterexample
        } else {
            find_smaller_counterexample(shrinks.drop_first(), prop, default)
        }
    }
}

pub open spec fn shrink_expr(e: Expr) -> Seq<Expr>
    decreases e
{
    match e {
        // Atoms don't shrink
        Expr::Var { .. } => seq![],
        Expr::Tru => seq![],
        Expr::Fls => seq![],
        Expr::Zero => seq![],

        // Unary operators: try removing the operator
        Expr::Succ { e: inner } => {
            let shrunk_inner = shrink_expr(*inner);
            // Direct subexpression + shrunk versions
            let result = seq![*inner];
            result.add(shrunk_inner.map(|_i: int, x: Expr| Expr::Succ { e: Box::new(x) }))
        }

        Expr::Pred { e: inner } => {
            let shrunk_inner = shrink_expr(*inner);
            let result = seq![*inner];
            result.add(shrunk_inner.map(|_i: int, x: Expr| Expr::Pred { e: Box::new(x) }))
        }

        Expr::IsZero { e: inner } => {
            let shrunk_inner = shrink_expr(*inner);
            // Direct removal gives bool
            let result = seq![Expr::Tru, Expr::Fls];
            result.add(shrunk_inner.map(|_i: int, x: Expr| Expr::IsZero { e: Box::new(x) }))
        }

        // Binary operators: try operands or shrink operands
        Expr::App { e1, e2 } => {
            // Can shrink to either operand
            let result = seq![*e1, *e2];
            result
        }

        Expr::Add { e1, e2 } => {
            // Shrink to operands
            seq![*e1, *e2, Expr::Zero]
        }

        // If expression: try branches
        Expr::If { cond, then_br, else_br } => {
            // Direct branches (simpler alternatives)
            // Just return the branches as potential shrinks
            seq![*then_br, *else_br, *cond]
        }

        // Not expression: shrink inner
        Expr::Not { e: inner } => {
            seq![*inner]
        }

        // Lambda: shrink body
        Expr::Lam { x, ty, body } => {
            let shrunk_body = shrink_expr(*body);
            shrunk_body.map(|_i: int, b: Expr| Expr::Lam { x, ty, body: Box::new(b) })
        }
    }
}


pub type Var = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Expr {
    Var { x: Var },
    Lam { x: Var, ty: Ty, body: Box<Expr> },
    App { e1: Box<Expr>, e2: Box<Expr> },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
    Add { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
}

pub open spec fn is_counterexample(e: Expr, prop: spec_fn(Expr) -> bool) -> bool {
    !prop(e)
}

pub open spec fn expr_size(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 1,
        Expr::Lam { body, .. } => 1 + expr_size(*body),
        Expr::App { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Tru => 1,
        Expr::Fls => 1,
        Expr::If { cond, then_br, else_br } =>
            1 + expr_size(*cond) + expr_size(*then_br) + expr_size(*else_br),
        Expr::Zero => 1,
        Expr::Succ { e } => 1 + expr_size(*e),
        Expr::Pred { e } => 1 + expr_size(*e),
        Expr::IsZero { e } => 1 + expr_size(*e),
        Expr::Add { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Not { e } => 1 + expr_size(*e),
    }
}


pub open spec fn shrink_to_minimal(e: Expr, prop: spec_fn(Expr) -> bool) -> Expr
    decreases expr_size(e)
{
    if !is_counterexample(e, prop) {
        e  // Not a counterexample
    } else {
        let shrinks = shrink_expr(e);
        find_smaller_counterexample(shrinks, prop, e)
    }
}

} // verus!