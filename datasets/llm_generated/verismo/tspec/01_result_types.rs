// Variation: Result and Error Types
// From: source/verismo/src/tspec/mod.rs (ResultOrErr, ResultWithErr)
// Demonstrates: Result type variants with error handling

use vstd::prelude::*;

verus! {

#[is_variant]
pub enum ResultOrError<T, E> {
    Ok(T),
    Error(E),
}

impl<T, E> ResultOrError<T, E> {
    pub open spec fn is_ok(&self) -> bool {
        matches!(*self, ResultOrError::Ok(_))
    }

    pub open spec fn is_error(&self) -> bool {
        matches!(*self, ResultOrError::Error(_))
    }

    pub fn check_is_ok(&self) -> (result: bool)
        ensures
            result <==> self.is_ok(),
    {
        matches!(self, ResultOrError::Ok(_))
    }

    pub fn check_is_error(&self) -> (result: bool)
        ensures
            result <==> self.is_error(),
    {
        matches!(self, ResultOrError::Error(_))
    }
}

impl<T: Copy, E: Copy> ResultOrError<T, E> {
    pub open spec fn get_value(&self) -> T {
        match self {
            ResultOrError::Ok(v) => *v,
            ResultOrError::Error(_) => arbitrary(),
        }
    }

    pub open spec fn get_error(&self) -> E {
        match self {
            ResultOrError::Ok(_) => arbitrary(),
            ResultOrError::Error(e) => *e,
        }
    }

    pub fn unwrap_or(&self, default: T) -> (result: T)
        ensures
            result == match self {
                ResultOrError::Ok(v) => *v,
                ResultOrError::Error(_) => default,
            },
    {
        match self {
            ResultOrError::Ok(v) => *v,
            ResultOrError::Error(_) => default,
        }
    }
}

#[is_variant]
pub enum ResultWithError<T, E> {
    Ok(T),
    Error(T, E),
}

impl<T, E> ResultWithError<T, E> {
    pub open spec fn is_ok(&self) -> bool {
        matches!(*self, ResultWithError::Ok(_))
    }

    pub open spec fn is_error(&self) -> bool {
        matches!(*self, ResultWithError::Error(_, _))
    }

    pub fn check_is_ok(&self) -> (result: bool)
        ensures
            result <==> self.is_ok(),
    {
        matches!(self, ResultWithError::Ok(_))
    }

    pub fn check_is_error(&self) -> (result: bool)
        ensures
            result <==> self.is_error(),
    {
        matches!(self, ResultWithError::Error(_, _))
    }
}

impl<T: Copy, E: Copy> ResultWithError<T, E> {
    pub open spec fn get_result(&self) -> T {
        match self {
            ResultWithError::Ok(v) => *v,
            ResultWithError::Error(v, _) => *v,
        }
    }

    pub open spec fn get_error(&self) -> E {
        match self {
            ResultWithError::Ok(_) => arbitrary(),
            ResultWithError::Error(_, e) => *e,
        }
    }

    pub fn get(&self) -> (result: &T)
        ensures
            *result == self.get_result(),
    {
        match self {
            ResultWithError::Ok(v) => v,
            ResultWithError::Error(v, _) => v,
        }
    }

    pub open spec fn has_error_code(&self, err: E) -> bool
        where E: PartialEq
    {
        match self {
            ResultWithError::Ok(_) => false,
            ResultWithError::Error(_, e) => *e == err,
        }
    }
}

pub struct Operation<T, E> {
    pub result: ResultOrError<T, E>,
    pub completed: bool,
}

impl<T, E> Operation<T, E> {
    pub fn new(result: ResultOrError<T, E>) -> (ret: Self)
        ensures
            ret.result is Ok <==> result is Ok,
            !ret.completed,
    {
        Operation {
            result,
            completed: false,
        }
    }

    pub fn mark_completed(&mut self)
        ensures
            self.completed,
            self.result is Ok <==> old(self).result is Ok,
    {
        self.completed = true;
    }

    pub fn is_completed(&self) -> (result: bool)
        ensures
            result == self.completed,
    {
        self.completed
    }

    pub open spec fn is_success(&self) -> bool {
        self.result.is_ok() && self.completed
    }

    pub fn check_is_success(&self) -> (result: bool)
        ensures
            result <==> self.is_success(),
    {
        self.result.check_is_ok() && self.completed
    }
}

fn test_result_types() {
    let ok_result: ResultOrError<u64, u8> = ResultOrError::Ok(42);
    let is_ok = ok_result.check_is_ok();
    let is_error = ok_result.check_is_error();

    let err_result: ResultOrError<u64, u8> = ResultOrError::Error(1);
    let err_is_ok = err_result.check_is_ok();
    let err_is_error = err_result.check_is_error();

    let value = ok_result.unwrap_or(0);

    let with_err: ResultWithError<u64, u8> = ResultWithError::Error(100, 2);
    let with_err_is_ok = with_err.check_is_ok();
    let with_err_is_error = with_err.check_is_error();
    let result_val = with_err.get();

    let mut op = Operation::new(ok_result);
    let completed = op.is_completed();
    op.mark_completed();
    let is_success = op.check_is_success();

    proof {
        assert(is_ok);
        assert(!is_error);
        assert(!err_is_ok);
        assert(err_is_error);
        assert(value == 42);
        assert(!with_err_is_ok);
        assert(with_err_is_error);
        assert(*result_val == 100);
        assert(!completed);
        assert(is_success);
    }
}

} // verus!

fn main() {
    test_result_types();
}
