use vstd::prelude::*;

verus! {

pub open spec fn get_bool(v: Value) -> bool {
    match v { Value::VBool { b } => b, _ => false }
}

pub open spec fn closure_val(x: Id, ty: Ty, body: Expr, env: Env) -> Value {
    Value::VClosure { x, ty, body: Box::new(body), env }
}

pub open spec fn env_extend(env: Env, x: Id, v: Value) -> Env { env.insert(x, v) }

pub open spec fn is_bool_val(v: Value) -> bool {
    match v { Value::VBool { .. } => true, _ => false }
}


pub open spec fn get_nat(v: Value) -> nat {
    match v { Value::VNat { n } => n, _ => 0 }
}

pub open spec fn is_nat_val(v: Value) -> bool {
    match v { Value::VNat { .. } => true, _ => false }
}

pub open spec fn nat_val(n: nat) -> Value { Value::VNat { n } }


pub open spec fn eval_err() -> EvalResult {
    EvalResult::Err
}

pub open spec fn bool_val(b: bool) -> Value { Value::VBool { b } }

pub open spec fn eval_ok(v: Value) -> EvalResult {
    EvalResult::Ok { v }
}

pub open spec fn env_lookup(env: Env, x: Id) -> Option<Value> {
    if env.dom().contains(x) { Option::Some(env[x]) } else { Option::None }
}


pub open spec fn get_value(r: EvalResult) -> Value
    recommends is_ok(r)
{
    match r {
        EvalResult::Ok { v } => v,
        EvalResult::Err => bool_val(false),  // arbitrary default
    }
}

pub open spec fn eval(env: Env, e: Expr, fuel: nat) -> EvalResult
    decreases fuel, e
{
    if fuel == 0 {
        eval_err()  // Out of fuel
    } else {
        match e {
            // E-Var: Look up variable in environment
            Expr::Var { x } => {
                match env_lookup(env, x) {
                    Option::Some(v) => eval_ok(v),
                    Option::None => eval_err(),
                }
            }

            // E-BoolConst: Boolean constants evaluate to themselves
            Expr::BoolConst { b } => eval_ok(bool_val(b)),

            // E-NatConst: Natural constants evaluate to themselves
            Expr::NatConst { n } => eval_ok(nat_val(n)),

            // E-Plus: Addition
            Expr::Plus { e1, e2 } => {
                let r1 = eval(env, *e1, (fuel - 1) as nat);
                if !is_ok(r1) {
                    eval_err()
                } else {
                    let v1 = get_value(r1);
                    if !is_nat_val(v1) {
                        eval_err()
                    } else {
                        let r2 = eval(env, *e2, (fuel - 1) as nat);
                        if !is_ok(r2) {
                            eval_err()
                        } else {
                            let v2 = get_value(r2);
                            if !is_nat_val(v2) {
                                eval_err()
                            } else {
                                eval_ok(nat_val(get_nat(v1) + get_nat(v2)))
                            }
                        }
                    }
                }
            }

            // E-If: Conditional evaluation
            Expr::If { cond, then_br, else_br } => {
                let r_cond = eval(env, *cond, (fuel - 1) as nat);
                if !is_ok(r_cond) {
                    eval_err()
                } else {
                    let v_cond = get_value(r_cond);
                    if !is_bool_val(v_cond) {
                        eval_err()
                    } else if get_bool(v_cond) {
                        eval(env, *then_br, (fuel - 1) as nat)
                    } else {
                        eval(env, *else_br, (fuel - 1) as nat)
                    }
                }
            }

            // E-Lam: Lambda abstractions evaluate to closures
            Expr::Lam { x, ty, body } => {
                eval_ok(closure_val(x, ty, *body, env))
            }

            // E-App: Function application
            Expr::App { e1, e2 } => {
                let r1 = eval(env, *e1, (fuel - 1) as nat);
                if !is_ok(r1) {
                    eval_err()
                } else {
                    let v1 = get_value(r1);
                    if !is_closure_val(v1) {
                        eval_err()
                    } else {
                        let r2 = eval(env, *e2, (fuel - 1) as nat);
                        if !is_ok(r2) {
                            eval_err()
                        } else {
                            let v2 = get_value(r2);
                            match v1 {
                                Value::VClosure { x, ty: _, body, env: closure_env } => {
                                    let new_env = env_extend(closure_env, x, v2);
                                    eval(new_env, *body, (fuel - 1) as nat)
                                }
                                _ => eval_err(),  // Unreachable
                            }
                        }
                    }
                }
            }

            // E-Eq: Equality comparison (only for nats)
            Expr::Eq { e1, e2 } => {
                let r1 = eval(env, *e1, (fuel - 1) as nat);
                if !is_ok(r1) {
                    eval_err()
                } else {
                    let v1 = get_value(r1);
                    if !is_nat_val(v1) {
                        eval_err()
                    } else {
                        let r2 = eval(env, *e2, (fuel - 1) as nat);
                        if !is_ok(r2) {
                            eval_err()
                        } else {
                            let v2 = get_value(r2);
                            if !is_nat_val(v2) {
                                eval_err()
                            } else {
                                eval_ok(bool_val(get_nat(v1) == get_nat(v2)))
                            }
                        }
                    }
                }
            }

            // E-Lt: Less-than comparison
            Expr::Lt { e1, e2 } => {
                let r1 = eval(env, *e1, (fuel - 1) as nat);
                if !is_ok(r1) {
                    eval_err()
                } else {
                    let v1 = get_value(r1);
                    if !is_nat_val(v1) {
                        eval_err()
                    } else {
                        let r2 = eval(env, *e2, (fuel - 1) as nat);
                        if !is_ok(r2) {
                            eval_err()
                        } else {
                            let v2 = get_value(r2);
                            if !is_nat_val(v2) {
                                eval_err()
                            } else {
                                eval_ok(bool_val(get_nat(v1) < get_nat(v2)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub open spec fn is_closure_val(v: Value) -> bool {
    match v { Value::VClosure { .. } => true, _ => false }
}

pub open spec fn lam_expr(x: Id, ty: Ty, body: Expr) -> Expr {
    Expr::Lam { x, ty, body: Box::new(body) }
}


pub type Id = nat;

pub type Var = nat;

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Expr {
    Var { x: Id },
    BoolConst { b: bool },
    NatConst { n: nat },
    Plus { e1: Box<Expr>, e2: Box<Expr> },
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    App { e1: Box<Expr>, e2: Box<Expr> },
    Lam { x: Id, ty: Ty, body: Box<Expr> },
    Eq { e1: Box<Expr>, e2: Box<Expr> },
    Lt { e1: Box<Expr>, e2: Box<Expr> },
}

pub enum Value {
    VBool { b: bool },
    VNat { n: nat },
    VClosure { x: Id, ty: Ty, body: Box<Expr>, env: Env },
}

pub type Env = Map<Id, Value>;

pub enum EvalResult {
    Ok { v: Value },
    Err,
}

pub open spec fn is_ok(r: EvalResult) -> bool {
    match r {
        EvalResult::Ok { .. } => true,
        EvalResult::Err => false,
    }
}


pub proof fn lam_eval_closure(env: Env, x: Id, ty: Ty, body: Expr, fuel: nat)
    requires fuel >= 1
    ensures
        is_ok(eval(env, lam_expr(x, ty, body), fuel)),
        is_closure_val(get_value(eval(env, lam_expr(x, ty, body), fuel))),
{
}

} // verus!