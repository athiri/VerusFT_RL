use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn task_code_precond (sequence : Seq < int >) -> bool { true }
spec fn task_code_postcond (sequence : Seq < int > , result : int , h_precond : bool) -> bool { if sequence . len () == 0 { result == 0 } else { true } }
fn task_code (sequence : Vec < i32 >) -> (result : i32) requires task_code_precond (sequence @ . map (| i , x | x as int)) ensures task_code_postcond (sequence @ . map (| i , x | x as int) , result as int , task_code_precond (sequence @ . map (| i , x | x as int))) { return 0 ; }

} // verus!