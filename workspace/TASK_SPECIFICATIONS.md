# Verus Training Tasks: Input/Output Specifications & Evaluation

This document describes the three training tasks, their input/output formats, and evaluation criteria.

---

## Overview

| Task | Name | Input | Output | Primary Metric |
|------|------|-------|--------|----------------|
| **A** | Code → Specs | Code without specifications | Specifications to add | Positive/negative proxy pass rate |
| **B** | Specs → Code | Function signature + specs | Full verified implementation | Execution output match rate |
| **C** | Repair | Broken code | Fixed code | Verification pass rate |

---

## Task A: Code → Specifications

### Goal
Given Verus code **without** specifications, generate the appropriate `requires`, `ensures`, `decreases`, and `invariant` clauses.

### Input Format
```verus
use vstd::prelude::*;

verus! {

fn get_element(arr: Vec<u64>, i: usize) -> (ret: u64) {
    arr[i]
}

} // verus!
```

### Expected Output Format
```
requires arr.len() > 0, i < arr@.len()
ensures ret == arr@[i as int]
```

**Note**: Output is ONLY the specifications, not the full code.

### Evaluation: Positive/Negative Proxy Testing

**Primary Metric**: Test generated specs against positive and negative input/output pairs.

#### Method: Executable Proxy

1. **Positive Proxies**: Valid (input, output) pairs that the function actually produces
   - Generated specs should **accept** these (i.e., inputs satisfy `requires`, outputs satisfy `ensures`)
   
2. **Negative Proxies**: Invalid (input, output) pairs
   - Generated specs should **reject** these (i.e., precondition fails OR postcondition fails)

```
Soundness:    Specs accept all positive proxies (no false negatives)
Completeness: Specs reject all negative proxies (no false positives)
```

#### Test Cases

| Proxy Type | Example | Expected |
|------------|---------|----------|
| **Positive** | `arr=[1,2,3], i=1 → ret=2` | Specs accept |
| **Negative (bad input)** | `arr=[], i=0 → ret=?` | `requires` rejects |
| **Negative (bad output)** | `arr=[1,2,3], i=1 → ret=99` | `ensures` rejects |

### Evaluation Code
```python
def evaluate_task_a(input_code, generated_specs, original_code):
    """
    Evaluate Task A: Code → Specs
    
    Use positive/negative proxies to check spec quality.
    """
    # Generate proxies from original code execution
    positive_proxies = generate_positive_proxies(original_code, num=50)
    negative_proxies = generate_negative_proxies(original_code, num=50)
    
    # Check soundness: specs should accept all positive proxies
    soundness_score = 0
    for (inp, out) in positive_proxies:
        if specs_accept(generated_specs, inp, out):
            soundness_score += 1
    
    # Check completeness: specs should reject all negative proxies
    completeness_score = 0
    for (inp, out) in negative_proxies:
        if specs_reject(generated_specs, inp, out):
            completeness_score += 1
    
    return {
        "soundness": soundness_score / len(positive_proxies),
        "completeness": completeness_score / len(negative_proxies),
        "overall": (soundness_score + completeness_score) / (len(positive_proxies) + len(negative_proxies))
    }

def generate_positive_proxies(code, num):
    """Run code on random valid inputs, collect (input, output) pairs."""
    proxies = []
    for _ in range(num):
        inp = generate_random_input()
        out = execute_function(code, inp)
        proxies.append((inp, out))
    return proxies

def generate_negative_proxies(code, num):
    """Generate invalid pairs: bad inputs OR correct inputs with wrong outputs."""
    proxies = []
    # Type 1: Invalid inputs (violate preconditions)
    for _ in range(num // 2):
        bad_inp = generate_invalid_input()  # e.g., empty array, out-of-bounds index
        proxies.append((bad_inp, None))
    # Type 2: Valid inputs but wrong outputs
    for _ in range(num // 2):
        inp = generate_random_input()
        wrong_out = execute_function(code, inp) + mutate()  # corrupt the output
        proxies.append((inp, wrong_out))
    return proxies

def specs_accept(specs, inp, out):
    """Check if specs accept this (input, output) pair."""
    requires_ok = evaluate_requires(specs, inp)
    ensures_ok = evaluate_ensures(specs, inp, out)
    return requires_ok and ensures_ok

def specs_reject(specs, inp, out):
    """Check if specs reject this (input, output) pair."""
    return not specs_accept(specs, inp, out)
```

---

## Task B: Specifications → Code

### Goal
Given a function signature with specifications, generate a **complete implementation** that produces the same outputs as the original code.

### Input Format
```verus
fn get_element(arr: Vec<u64>, i: usize) -> (ret: u64)
requires arr.len() > 0, i < arr@.len()
ensures ret == arr@[i as int]
```

### Expected Output Format
```verus
use vstd::prelude::*;

verus! {

fn get_element(arr: Vec<u64>, i: usize) -> (ret: u64)
    requires arr.len() > 0, i < arr@.len()
    ensures ret == arr@[i as int]
{
    arr[i]
}

} // verus!
```

**Note**: Output is the FULL code including imports, specs, and body.

### Evaluation: Functional Correctness via Execution

**Primary Metric**: Compare execution behavior of generated code vs original code.

#### Method: Input/Output Comparison

1. **Extract the executable function** from both generated and original code
2. **Generate test inputs** that satisfy the `requires` clause
3. **Execute both functions** on the same inputs
4. **Compare outputs** - if they match for all test inputs, the code is functionally correct

```
Original Code:    f_original(input) → output_1
Generated Code:   f_generated(input) → output_2

Functional correctness: output_1 == output_2 for all test inputs
```

#### Test Input Generation Strategies

| Strategy | Description | Coverage |
|----------|-------------|----------|
| **Bounded random** | Random inputs within small bounds | Quick, broad |
| **Edge cases** | Min/max values, empty arrays, etc. | Corner cases |
| **Spec-guided** | Sample inputs satisfying `requires` | Valid inputs |
| **Fuzzing** | Automated random generation | High volume |

### Evaluation Code
```python
def evaluate_task_b(input_spec, generated_code, original_code):
    """
    Evaluate Task B: Specs → Code
    
    Functional correctness via execution comparison.
    """
    # 1. Extract executable functions (strip Verus annotations)
    gen_fn = extract_executable_function(generated_code)
    orig_fn = extract_executable_function(original_code)
    
    # 2. Generate test inputs satisfying requires
    test_inputs = generate_test_inputs(input_spec, num_tests=100)
    
    # 3. Compare outputs
    matches = 0
    mismatches = []
    
    for inp in test_inputs:
        try:
            gen_output = execute_function(gen_fn, inp)
            orig_output = execute_function(orig_fn, inp)
            
            if gen_output == orig_output:
                matches += 1
            else:
                mismatches.append({
                    "input": inp,
                    "expected": orig_output,
                    "actual": gen_output
                })
        except Exception as e:
            mismatches.append({"input": inp, "error": str(e)})
    
    return {
        "functionally_correct": len(mismatches) == 0,
        "match_rate": matches / len(test_inputs),
        "total_tests": len(test_inputs),
        "mismatches": mismatches[:5]  # First 5 failures
    }
```

### Practical Implementation

For Rust/Verus code, use one of these approaches:

1. **Compile and run**: Strip Verus annotations, compile with `rustc`, run tests
2. **Interpreter**: Use a Rust interpreter/REPL for quick evaluation
3. **Property-based testing**: Use `proptest` or `quickcheck` for automated input generation

```python
def extract_executable_function(verus_code):
    """Strip Verus-specific annotations to get plain Rust."""
    # Remove: requires, ensures, decreases, invariant, proof blocks
    # Keep: function signature and body
    rust_code = strip_verus_annotations(verus_code)
    return rust_code

def generate_test_inputs(spec, num_tests=100):
    """Generate inputs satisfying the requires clause."""
    # Parse requires clause
    # Generate random values within constraints
    # Example: for "requires arr.len() > 0, i < arr.len()"
    #   → generate non-empty arrays and valid indices
    pass
```

---

## Task C: Error-Guided Repair

### Goal
Given broken Verus code (missing ensures/requires/invariant/decreases/assert), generate the **fixed code** that verifies.

### Bug Types
| Bug Type | Description | Example Fix |
|----------|-------------|-------------|
| `missing_ensures` | Missing postcondition | Add `ensures result >= 0` |
| `missing_requires` | Missing precondition | Add `requires x > 0` |
| `missing_decreases` | Missing termination measure | Add `decreases n` |
| `missing_invariant` | Missing loop invariant | Add `invariant i <= n` |
| `missing_assert` | Missing proof hint | Add `assert(condition)` |

### Input Format
```verus
use vstd::prelude::*;

verus! {

// BROKEN: missing ensures clause
fn abs(x: i32) -> (ret: i32)
    requires x != i32::MIN
{
    if x < 0 { -x } else { x }
}

} // verus!
```

### Expected Output Format
```verus
use vstd::prelude::*;

verus! {

// FIXED: ensures clause added
fn abs(x: i32) -> (ret: i32)
    requires x != i32::MIN
    ensures ret >= 0, ret == x || ret == -x
{
    if x < 0 { -x } else { x }
}

} // verus!
```

**Note**: Output is the FULL fixed code.

### Evaluation

1. **Primary: Verification Pass Rate**
   - Run Verus on generated fixed code
   - Pass = exit code 0

2. **Secondary Metrics**:
   - **Fix Correctness**: Does fix address the specific bug type?
   - **Minimality**: Is fix minimal (not changing unrelated code)?
   - **Semantic Preservation**: Does fixed code have same behavior?

### Evaluation Code
```python
def evaluate_task_c(input_broken, generated_fixed, bug_type):
    # Verify fixed code
    result = run_verus(generated_fixed)
    
    # Check fix is minimal (diff size)
    diff_size = compute_diff(input_broken, generated_fixed)
    
    # Check bug type addressed
    fix_type = detect_fix_type(input_broken, generated_fixed)
    
    return {
        "verified": result.returncode == 0,
        "fix_type_correct": fix_type == bug_type,
        "diff_lines": diff_size,
        "error": result.stderr if result.returncode != 0 else None
    }
```

---

## Evaluation Pipeline

### Running Verus Verification

```python
import subprocess
import tempfile

def run_verus(code: str, timeout: int = 60) -> dict:
    """Run Verus verification on code."""
    with tempfile.NamedTemporaryFile(suffix='.rs', delete=False) as f:
        f.write(code.encode())
        temp_path = f.name
    
    try:
        result = subprocess.run(
            ['verus', '--crate-type=lib', temp_path],
            capture_output=True,
            text=True,
            timeout=timeout
        )
        return {
            "success": result.returncode == 0,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "returncode": result.returncode
        }
    except subprocess.TimeoutExpired:
        return {"success": False, "error": "timeout"}
    finally:
        os.unlink(temp_path)
```

### Error Categorization

```python
def categorize_error(stderr: str) -> str:
    """Categorize Verus error type."""
    error = stderr.lower()
    
    if "expected" in error or "parse" in error:
        return "syntax_error"
    elif "requires" in error or "precondition" in error:
        return "precondition_error"
    elif "ensures" in error or "postcondition" in error:
        return "postcondition_error"
    elif "invariant" in error:
        return "invariant_error"
    elif "decreases" in error:
        return "termination_error"
    elif "assertion" in error:
        return "assertion_error"
    else:
        return "unknown_error"
```

### Aggregate Metrics

```python
def compute_metrics(results: list) -> dict:
    """Compute aggregate metrics from evaluation results."""
    total = len(results)
    verified = sum(1 for r in results if r["verified"])
    
    error_counts = {}
    for r in results:
        if not r["verified"] and "error_type" in r:
            et = r["error_type"]
            error_counts[et] = error_counts.get(et, 0) + 1
    
    return {
        "total": total,
        "verified": verified,
        "pass_rate": verified / total if total > 0 else 0,
        "error_breakdown": error_counts
    }
```

---

## Summary Table

| Task | Input | Output | Evaluation | Correctness Guarantee |
|------|-------|--------|------------|----------------------|
| **A** | Code without specs | Just the specs | **Positive/negative proxy testing** | Specs accept valid, reject invalid behaviors |
| **B** | Signature + specs | Full implementation | **Execute & compare outputs** | Same outputs as original for all test inputs |
| **C** | Broken code | Fixed code | **Verify with Verus** | Code now passes verification |

### Key Differences

1. **Task A output is compact** - only the specifications
2. **Task B and C output is complete** - full verifiable code
3. **Task C has bug type metadata** - useful for analysis

### Dataset Statistics

| Task | Training | Validation | Test | Total |
|------|----------|------------|------|-------|
| A | 2,941 | 368 | 368 | 3,677 |
| B | 2,600 | 325 | 326 | 3,251 |
| C | 6,184 | 773 | 774 | 7,731 |
| **Total** | **11,725** | **1,466** | **1,468** | **14,659** |
