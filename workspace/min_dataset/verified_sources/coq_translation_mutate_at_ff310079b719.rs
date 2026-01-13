use vstd::prelude::*;

verus! {

pub open spec fn apply_pred_to_succ(e: Expr) -> Option<Expr> {
    match e {
        Expr::Pred { e: inner } => Option::Some(Expr::Succ { e: inner }),
        _ => Option::None,
    }
}

pub open spec fn apply_or_to_and(e: Expr) -> Option<Expr> {
    match e {
        Expr::Or { e1, e2 } => Option::Some(Expr::And { e1, e2 }),
        _ => Option::None,
    }
}

pub open spec fn apply_and_to_or(e: Expr) -> Option<Expr> {
    match e {
        Expr::And { e1, e2 } => Option::Some(Expr::Or { e1, e2 }),
        _ => Option::None,
    }
}

pub open spec fn apply_eq_to_lt(e: Expr) -> Option<Expr> {
    match e {
        Expr::Eq { e1, e2 } => Option::Some(Expr::Lt { e1, e2 }),
        _ => Option::None,
    }
}


pub open spec fn apply_false_to_true(e: Expr) -> Option<Expr> {
    match e {
        Expr::Fls => Option::Some(Expr::Tru),
        _ => Option::None,
    }
}

pub open spec fn apply_negate_cond(e: Expr) -> Option<Expr> {
    match e {
        Expr::If { cond, then_br, else_br } =>
            Option::Some(Expr::If {
                cond: Box::new(Expr::Not { e: cond }),
                then_br: then_br,
                else_br: else_br,
            }),
        _ => Option::None,
    }
}

pub open spec fn apply_true_to_false(e: Expr) -> Option<Expr> {
    match e {
        Expr::Tru => Option::Some(Expr::Fls),
        _ => Option::None,
    }
}

pub open spec fn apply_succ_to_pred(e: Expr) -> Option<Expr> {
    match e {
        Expr::Succ { e: inner } => Option::Some(Expr::Pred { e: inner }),
        _ => Option::None,
    }
}

pub open spec fn apply_swap_branches(e: Expr) -> Option<Expr> {
    match e {
        Expr::If { cond, then_br, else_br } =>
            Option::Some(Expr::If {
                cond: cond,
                then_br: else_br,
                else_br: then_br,
            }),
        _ => Option::None,
    }
}


pub open spec fn apply_mutation(e: Expr, op: MutationOp) -> Option<Expr> {
    match op {
        MutationOp::NegateCondition => apply_negate_cond(e),
        MutationOp::SwapBranches => apply_swap_branches(e),
        MutationOp::TrueToFalse => apply_true_to_false(e),
        MutationOp::FalseToTrue => apply_false_to_true(e),
        MutationOp::SuccToPred => apply_succ_to_pred(e),
        MutationOp::PredToSucc => apply_pred_to_succ(e),
        MutationOp::AndToOr => apply_and_to_or(e),
        MutationOp::OrToAnd => apply_or_to_and(e),
        MutationOp::EqToLt => apply_eq_to_lt(e),
        _ => Option::None,
    }
}

pub open spec fn expr_size(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 1,
        Expr::Tru => 1,
        Expr::Fls => 1,
        Expr::If { cond, then_br, else_br } =>
            1 + expr_size(*cond) + expr_size(*then_br) + expr_size(*else_br),
        Expr::Zero => 1,
        Expr::Succ { e } => 1 + expr_size(*e),
        Expr::Pred { e } => 1 + expr_size(*e),
        Expr::IsZero { e } => 1 + expr_size(*e),
        Expr::And { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Or { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Not { e } => 1 + expr_size(*e),
        Expr::Eq { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Lt { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Add { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
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
    Eq { e1: Box<Expr>, e2: Box<Expr> },
    Lt { e1: Box<Expr>, e2: Box<Expr> },
    Add { e1: Box<Expr>, e2: Box<Expr> },
}

pub enum MutationOp {
    // Boolean mutations
    NegateCondition,      // Replace cond with not(cond)
    SwapBranches,         // Swap then/else branches
    TrueToFalse,          // Replace true with false
    FalseToTrue,          // Replace false with true

    // Relational mutations
    EqToLt,               // Replace == with <
    LtToEq,               // Replace < with ==
    NegateComparison,     // Replace op with not(op)

    // Arithmetic mutations
    IncrementConst,       // Add 1 to constant
    DecrementConst,       // Subtract 1 from constant
    SuccToPred,           // Replace succ with pred
    PredToSucc,           // Replace pred with succ
    SwapOperands,         // Swap operands of binary op

    // Logical mutations
    AndToOr,              // Replace && with ||
    OrToAnd,              // Replace || with &&

    // Dead code mutations
    RemoveBranch,         // Replace if with one branch
}


pub open spec fn mutate_at(e: Expr, op: MutationOp, loc: nat) -> Option<Expr>
    decreases e, loc
{
    if loc == 0 {
        // Apply mutation at this node
        apply_mutation(e, op)
    } else {
        // Recurse into subexpressions
        match e {
            Expr::If { cond, then_br, else_br } => {
                if loc <= expr_size(*cond) {
                    match mutate_at(*cond, op, (loc - 1) as nat) {
                        Option::Some(new_cond) => Option::Some(Expr::If {
                            cond: Box::new(new_cond),
                            then_br: then_br,
                            else_br: else_br,
                        }),
                        Option::None => Option::None,
                    }
                } else {
                    Option::None
                }
            }
            Expr::Succ { e: inner } => {
                match mutate_at(*inner, op, (loc - 1) as nat) {
                    Option::Some(new_inner) => Option::Some(Expr::Succ { e: Box::new(new_inner) }),
                    Option::None => Option::None,
                }
            }
            Expr::Not { e: inner } => {
                match mutate_at(*inner, op, (loc - 1) as nat) {
                    Option::Some(new_inner) => Option::Some(Expr::Not { e: Box::new(new_inner) }),
                    Option::None => Option::None,
                }
            }
            _ => Option::None,
        }
    }
}

} // verus!