# Evaluation Pipeline Implementation Plan

This document provides a concrete, step-by-step plan to implement the evaluation pipelines for Tasks A, B, and C.

---

## Overview

| Task | Evaluation Method | Implementation Complexity | Priority |
|------|-------------------|---------------------------|----------|
| **C** | Verus verification | ✅ Already implemented | P0 |
| **B** | Execute & compare outputs | Medium | P1 |
| **A** | Positive/negative proxy testing | Hard | P2 |

**Recommended order**: C → B → A (easiest to hardest)

---

## Task C: Verus Verification (✅ DONE)

Already implemented in `evaluate_pipeline.py`. Just needs testing.

### Current Implementation
```python
def evaluate_task_c(generated_code):
    result = verify_with_verus(generated_code)
    return {
        "verified": result.returncode == 0,
        "error_type": categorize_error(result.stderr)
    }
```

### TODO
- [ ] Test on 10 samples from `task_c_test.jsonl`
- [ ] Verify error categorization is accurate
- [ ] Add fix minimality check (diff size)

---

## Task B: Execute & Compare Outputs (P1)

### Goal
Compare execution behavior of generated code vs original code on test inputs.

### Implementation Steps

#### Step 1: Create Verus → Rust Stripper
Strip Verus-specific annotations to get executable Rust.

```python
# File: verus_to_rust.py

import re

def strip_verus_annotations(verus_code: str) -> str:
    """Convert Verus code to plain Rust by removing verification constructs."""
    
    # Remove verus! { } wrapper
    code = re.sub(r'verus!\s*\{', '', verus_code)
    code = re.sub(r'\}\s*//\s*verus!', '', code)
    
    # Remove spec/proof/ghost functions entirely
    code = re.sub(r'(pub\s+)?(open\s+)?(closed\s+)?spec\s+fn\s+\w+.*?(?=\n(pub|fn|spec|proof|ghost|$))', '', code, flags=re.DOTALL)
    code = re.sub(r'proof\s+fn\s+\w+.*?(?=\n(pub|fn|spec|proof|ghost|$))', '', code, flags=re.DOTALL)
    
    # Remove requires/ensures/decreases/invariant clauses
    code = re.sub(r'\n\s*requires\s+[^{]+', '', code)
    code = re.sub(r'\n\s*ensures\s+[^{]+', '', code)
    code = re.sub(r'\n\s*decreases\s+[^{]+', '', code)
    code = re.sub(r'\n\s*invariant\s+[^{]+', '', code)
    
    # Remove proof blocks: proof { ... }
    code = re.sub(r'proof\s*\{[^}]*\}', '', code)
    
    # Remove assert-by blocks
    code = re.sub(r'assert\s*\([^)]+\)\s*by\s*\{[^}]*\}', '', code)
    
    # Remove ghost variables and tracked/ghost keywords
    code = re.sub(r'\bghost\b', '', code)
    code = re.sub(r'\btracked\b', '', code)
    
    # Remove view operators (@)
    code = re.sub(r'(\w+)@', r'\1', code)
    
    # Remove vstd imports, replace with std
    code = re.sub(r'use vstd::prelude::\*;', 'use std::vec::Vec;', code)
    code = re.sub(r'use vstd::\w+::\*;', '', code)
    
    # Clean up extra whitespace
    code = re.sub(r'\n\s*\n\s*\n', '\n\n', code)
    
    return code.strip()
```

#### Step 2: Create Rust Executor
Compile and run Rust code with test inputs.

```python
# File: rust_executor.py

import subprocess
import tempfile
import os
import json
from pathlib import Path

def compile_rust_to_binary(rust_code: str, fn_name: str) -> str:
    """Compile Rust code to executable, return path to binary."""
    
    # Create a test harness that calls the function
    harness = f'''
{rust_code}

fn main() {{
    // Read JSON input from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let args: serde_json::Value = serde_json::from_str(&input).unwrap();
    
    // Call function and output result
    let result = {fn_name}(/* parse args */);
    println!("{{}}", serde_json::to_string(&result).unwrap());
}}
'''
    
    with tempfile.TemporaryDirectory() as tmpdir:
        src_path = Path(tmpdir) / "main.rs"
        bin_path = Path(tmpdir) / "test_binary"
        
        src_path.write_text(harness)
        
        result = subprocess.run(
            ["rustc", str(src_path), "-o", str(bin_path)],
            capture_output=True, text=True
        )
        
        if result.returncode != 0:
            raise CompilationError(result.stderr)
        
        return str(bin_path)


def execute_function(binary_path: str, inputs: dict) -> any:
    """Execute compiled function with given inputs."""
    result = subprocess.run(
        [binary_path],
        input=json.dumps(inputs),
        capture_output=True, text=True, timeout=5
    )
    
    if result.returncode != 0:
        raise ExecutionError(result.stderr)
    
    return json.loads(result.stdout)
```

#### Step 3: Create Test Input Generator
Generate inputs satisfying the `requires` clause.

```python
# File: input_generator.py

import random
from typing import List, Dict, Any

def generate_test_inputs(fn_signature: str, requires_clause: str, num_tests: int = 100) -> List[Dict]:
    """Generate test inputs satisfying requires clause."""
    
    # Parse function parameters
    params = parse_function_params(fn_signature)
    
    # Parse constraints from requires
    constraints = parse_requires(requires_clause)
    
    inputs = []
    attempts = 0
    max_attempts = num_tests * 10
    
    while len(inputs) < num_tests and attempts < max_attempts:
        attempts += 1
        candidate = {}
        
        for param_name, param_type in params.items():
            candidate[param_name] = generate_value_for_type(param_type)
        
        # Check if satisfies requires
        if satisfies_constraints(candidate, constraints):
            inputs.append(candidate)
    
    return inputs


def generate_value_for_type(rust_type: str) -> Any:
    """Generate random value for a Rust type."""
    
    if rust_type in ["i32", "i64"]:
        return random.randint(-100, 100)
    elif rust_type in ["u32", "u64", "usize"]:
        return random.randint(0, 100)
    elif rust_type == "bool":
        return random.choice([True, False])
    elif rust_type.startswith("Vec<"):
        inner = rust_type[4:-1]
        length = random.randint(0, 10)
        return [generate_value_for_type(inner) for _ in range(length)]
    else:
        raise NotImplementedError(f"Type {rust_type} not supported")


def parse_requires(requires_clause: str) -> List[dict]:
    """Parse requires clause into constraints."""
    # e.g., "arr.len() > 0, i < arr.len()"
    # Returns: [{"type": "gt", "left": "arr.len()", "right": 0}, ...]
    # TODO: Implement proper parsing
    return []
```

#### Step 4: Integrate into Evaluation Pipeline
```python
# File: evaluate_task_b.py

def evaluate_task_b(input_spec: str, generated_code: str, original_code: str) -> dict:
    """
    Evaluate Task B: Specs → Code via execution comparison.
    """
    # 1. Strip Verus annotations
    gen_rust = strip_verus_annotations(generated_code)
    orig_rust = strip_verus_annotations(original_code)
    
    # 2. Extract function name
    fn_name = extract_function_name(input_spec)
    
    # 3. Compile both
    try:
        gen_binary = compile_rust_to_binary(gen_rust, fn_name)
        orig_binary = compile_rust_to_binary(orig_rust, fn_name)
    except CompilationError as e:
        return {"success": False, "error": "compilation_error", "details": str(e)}
    
    # 4. Generate test inputs
    requires = extract_requires(input_spec)
    test_inputs = generate_test_inputs(input_spec, requires, num_tests=50)
    
    # 5. Compare outputs
    matches = 0
    mismatches = []
    
    for inp in test_inputs:
        try:
            gen_out = execute_function(gen_binary, inp)
            orig_out = execute_function(orig_binary, inp)
            
            if gen_out == orig_out:
                matches += 1
            else:
                mismatches.append({
                    "input": inp,
                    "expected": orig_out,
                    "actual": gen_out
                })
        except Exception as e:
            mismatches.append({"input": inp, "error": str(e)})
    
    return {
        "success": len(mismatches) == 0,
        "match_rate": matches / len(test_inputs),
        "total_tests": len(test_inputs),
        "mismatches": mismatches[:5]
    }
```

### Deliverables
- [ ] `verus_to_rust.py` - Verus annotation stripper
- [ ] `rust_executor.py` - Rust compilation and execution
- [ ] `input_generator.py` - Test input generation
- [ ] `evaluate_task_b.py` - Integration module
- [ ] Unit tests for each module

### Estimated Effort: 2-3 days

---

## Task A: Positive/Negative Proxy Testing (P2)

### Goal
Test generated specs against positive (valid) and negative (invalid) input/output pairs.

### Implementation Steps

#### Step 1: Spec Parser
Parse generated specs to extract `requires` and `ensures` clauses.

```python
# File: spec_parser.py

import re
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class ParsedSpec:
    requires: List[str]  # List of precondition expressions
    ensures: List[str]   # List of postcondition expressions
    decreases: Optional[str]
    invariants: List[str]

def parse_specs(spec_text: str) -> ParsedSpec:
    """Parse specification text into structured format."""
    
    requires = []
    ensures = []
    decreases = None
    invariants = []
    
    # Split by keywords
    lines = spec_text.strip().split('\n')
    
    for line in lines:
        line = line.strip()
        if line.startswith('requires'):
            # Extract comma-separated conditions
            conds = line[8:].strip()
            requires.extend([c.strip() for c in conds.split(',')])
        elif line.startswith('ensures'):
            conds = line[7:].strip()
            ensures.extend([c.strip() for c in conds.split(',')])
        elif line.startswith('decreases'):
            decreases = line[9:].strip()
        elif line.startswith('invariant'):
            invariants.append(line[9:].strip())
    
    return ParsedSpec(requires, ensures, decreases, invariants)
```

#### Step 2: Spec Evaluator
Evaluate specs against concrete input/output pairs.

```python
# File: spec_evaluator.py

from typing import Any, Dict, Tuple

def evaluate_requires(requires: List[str], inputs: Dict[str, Any]) -> Tuple[bool, str]:
    """
    Check if inputs satisfy all requires clauses.
    Returns (satisfied, failing_clause).
    """
    for clause in requires:
        try:
            # Convert Verus syntax to Python
            py_expr = verus_to_python_expr(clause, inputs)
            if not eval(py_expr):
                return False, clause
        except Exception as e:
            return False, f"{clause} (error: {e})"
    
    return True, ""


def evaluate_ensures(ensures: List[str], inputs: Dict[str, Any], output: Any) -> Tuple[bool, str]:
    """
    Check if output satisfies all ensures clauses.
    Returns (satisfied, failing_clause).
    """
    # Add output to evaluation context
    context = {**inputs, "ret": output, "result": output}
    
    for clause in ensures:
        try:
            py_expr = verus_to_python_expr(clause, context)
            if not eval(py_expr):
                return False, clause
        except Exception as e:
            return False, f"{clause} (error: {e})"
    
    return True, ""


def verus_to_python_expr(verus_expr: str, context: Dict) -> str:
    """Convert Verus expression to Python."""
    expr = verus_expr
    
    # Replace Verus operators
    expr = expr.replace('==>', ' or not ')  # implication
    expr = expr.replace('&&', ' and ')
    expr = expr.replace('||', ' or ')
    expr = expr.replace('!', ' not ')
    
    # Replace array access: arr@[i as int] → arr[i]
    expr = re.sub(r'(\w+)@\[(\w+)\s+as\s+int\]', r'\1[\2]', expr)
    expr = re.sub(r'(\w+)@\.len\(\)', r'len(\1)', expr)
    
    # Replace forall/exists (simplified - only works for simple cases)
    # forall |i: int| 0 <= i < n ==> P(i)
    forall_match = re.match(r'forall\s*\|(\w+):\s*int\|\s*(.+)==>\s*(.+)', expr)
    if forall_match:
        var, range_cond, prop = forall_match.groups()
        # Convert to Python all() expression
        # This is a simplification - real implementation needs proper parsing
        expr = f"all({prop.replace(var, 'x')} for x in range(100) if {range_cond.replace(var, 'x')})"
    
    return expr
```

#### Step 3: Proxy Generator
Generate positive and negative test cases.

```python
# File: proxy_generator.py

from typing import List, Tuple, Dict, Any

def generate_positive_proxies(
    original_code: str,
    fn_name: str,
    num: int = 50
) -> List[Tuple[Dict[str, Any], Any]]:
    """
    Generate positive (input, output) pairs by executing original code.
    """
    # Strip Verus annotations
    rust_code = strip_verus_annotations(original_code)
    binary = compile_rust_to_binary(rust_code, fn_name)
    
    # Extract function signature for input generation
    signature = extract_function_signature(original_code, fn_name)
    
    proxies = []
    for _ in range(num):
        # Generate random valid input
        inp = generate_random_input_for_signature(signature)
        
        try:
            out = execute_function(binary, inp)
            proxies.append((inp, out))
        except:
            continue  # Skip inputs that cause runtime errors
    
    return proxies


def generate_negative_proxies(
    original_code: str,
    fn_name: str,
    positive_proxies: List[Tuple[Dict, Any]],
    num: int = 50
) -> List[Tuple[Dict[str, Any], Any]]:
    """
    Generate negative (input, output) pairs:
    - Type 1: Invalid inputs (that should fail requires)
    - Type 2: Valid inputs with wrong outputs (that should fail ensures)
    """
    proxies = []
    
    # Type 1: Invalid inputs (half)
    signature = extract_function_signature(original_code, fn_name)
    for _ in range(num // 2):
        bad_inp = generate_invalid_input(signature)
        proxies.append((bad_inp, None))  # Output doesn't matter
    
    # Type 2: Valid inputs with corrupted outputs (half)
    for inp, out in positive_proxies[:num // 2]:
        corrupted_out = corrupt_output(out)
        proxies.append((inp, corrupted_out))
    
    return proxies


def corrupt_output(output: Any) -> Any:
    """Corrupt an output value to create a negative proxy."""
    if isinstance(output, int):
        return output + 1  # Off by one
    elif isinstance(output, bool):
        return not output
    elif isinstance(output, list):
        if len(output) > 0:
            return output[:-1]  # Remove last element
        return [0]  # Add element to empty
    else:
        return output
```

#### Step 4: Integrate into Evaluation Pipeline
```python
# File: evaluate_task_a.py

def evaluate_task_a(
    input_code: str,
    generated_specs: str,
    original_code: str
) -> dict:
    """
    Evaluate Task A: Code → Specs via proxy testing.
    """
    # 1. Parse generated specs
    specs = parse_specs(generated_specs)
    
    # 2. Extract function info
    fn_name = extract_function_name(input_code)
    
    # 3. Generate proxies
    positive_proxies = generate_positive_proxies(original_code, fn_name, num=50)
    negative_proxies = generate_negative_proxies(original_code, fn_name, positive_proxies, num=50)
    
    # 4. Test soundness (specs should accept positive proxies)
    soundness_passes = 0
    soundness_failures = []
    
    for inp, out in positive_proxies:
        req_ok, req_fail = evaluate_requires(specs.requires, inp)
        ens_ok, ens_fail = evaluate_ensures(specs.ensures, inp, out)
        
        if req_ok and ens_ok:
            soundness_passes += 1
        else:
            soundness_failures.append({
                "input": inp, "output": out,
                "requires_failed": req_fail if not req_ok else None,
                "ensures_failed": ens_fail if not ens_ok else None
            })
    
    # 5. Test completeness (specs should reject negative proxies)
    completeness_passes = 0
    completeness_failures = []
    
    for inp, out in negative_proxies:
        req_ok, _ = evaluate_requires(specs.requires, inp)
        ens_ok, _ = evaluate_ensures(specs.ensures, inp, out) if out is not None else (True, "")
        
        if not req_ok or not ens_ok:
            completeness_passes += 1  # Correctly rejected
        else:
            completeness_failures.append({
                "input": inp, "output": out,
                "reason": "should have been rejected but was accepted"
            })
    
    # 6. Compute metrics
    soundness = soundness_passes / len(positive_proxies) if positive_proxies else 0
    completeness = completeness_passes / len(negative_proxies) if negative_proxies else 0
    
    return {
        "soundness": soundness,
        "completeness": completeness,
        "overall": (soundness + completeness) / 2,
        "positive_tested": len(positive_proxies),
        "negative_tested": len(negative_proxies),
        "soundness_failures": soundness_failures[:5],
        "completeness_failures": completeness_failures[:5]
    }
```

### Deliverables
- [ ] `spec_parser.py` - Parse generated specs
- [ ] `spec_evaluator.py` - Evaluate specs against input/output pairs
- [ ] `proxy_generator.py` - Generate positive/negative proxies
- [ ] `evaluate_task_a.py` - Integration module
- [ ] Unit tests for each module

### Estimated Effort: 4-5 days

---

## Implementation Timeline

| Week | Task | Deliverables |
|------|------|--------------|
| **Week 1** | Task C testing + Task B foundation | Test Task C on dataset, implement `verus_to_rust.py` |
| **Week 2** | Task B completion | `rust_executor.py`, `input_generator.py`, `evaluate_task_b.py` |
| **Week 3** | Task A foundation | `spec_parser.py`, `spec_evaluator.py` |
| **Week 4** | Task A completion + integration | `proxy_generator.py`, `evaluate_task_a.py`, unified CLI |

---

## File Structure (Proposed)

```
evaluation/
├── __init__.py
├── core/
│   ├── verus_to_rust.py      # Verus → Rust conversion
│   ├── rust_executor.py       # Compile and run Rust
│   └── input_generator.py     # Generate test inputs
├── task_a/
│   ├── spec_parser.py         # Parse specs
│   ├── spec_evaluator.py      # Evaluate specs
│   ├── proxy_generator.py     # Generate proxies
│   └── evaluate.py            # Main evaluation
├── task_b/
│   └── evaluate.py            # Execution comparison
├── task_c/
│   └── evaluate.py            # Verus verification
├── cli.py                     # Unified command-line interface
└── tests/
    ├── test_verus_to_rust.py
    ├── test_spec_parser.py
    └── test_integration.py
```

---

## Testing Strategy

### Unit Tests
- [ ] Test `strip_verus_annotations` on 20 diverse examples
- [ ] Test `parse_specs` on 20 diverse specs
- [ ] Test `generate_test_inputs` produces valid inputs
- [ ] Test `verus_to_python_expr` on common patterns

### Integration Tests
- [ ] Run Task C on 10 samples, verify results match manual verification
- [ ] Run Task B on 10 samples, verify output comparison works
- [ ] Run Task A on 10 samples, verify proxy testing works

### End-to-End Tests
- [ ] Evaluate baseline model on all three tasks
- [ ] Compare against expected baseline performance
- [ ] Ensure metrics are reproducible

---

## Known Challenges

1. **Verus → Rust conversion is lossy**
   - Some Verus code uses features not directly translatable to standard Rust
   - Mitigation: Focus on common patterns, skip untranslatable cases

2. **Spec evaluation requires expression parsing**
   - `forall`/`exists` quantifiers are hard to evaluate
   - Mitigation: Use bounded quantifier checking, skip complex cases

3. **Test input generation needs constraint solving**
   - Complex `requires` clauses are hard to satisfy randomly
   - Mitigation: Start simple, use bounded search, add symbolic methods later

4. **Execution may have side effects**
   - Some functions modify state
   - Mitigation: Run in isolated environment, focus on pure functions first

---

## Success Criteria

| Metric | Target |
|--------|--------|
| Task C pass rate (baseline) | Measure and record |
| Task B match rate (baseline) | > 50% on simple functions |
| Task A soundness (baseline) | > 70% |
| Task A completeness (baseline) | > 50% |
| Test coverage | > 80% for core modules |
| Samples evaluated | 100+ per task |
