use vstd::prelude::*;

verus! {

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

} // verus!