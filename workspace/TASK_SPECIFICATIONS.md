# Verus Training Tasks: Input/Output Specifications & Evaluation

This document describes the three training tasks, their input/output formats, and evaluation criteria.

---

## Overview

| Task | Name | Input | Output | Primary Metric |
|------|------|-------|--------|----------------|
| **A** | Code → Specs | Code without specifications | Specifications to add | Three-tier: Template → LLM-guided → Proxy |
| **B** | Specs → Code | Function signature + specs | Full verified implementation | Fuzzing + Kani model checking |
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

### Evaluation: Template-Based Specification Equivalence

**Primary Metric**: Use Verus verification to prove generated specs are semantically equivalent to original specs.

#### Method: Equivalence Template

Use the verifier itself to check logical equivalence between original and generated specifications. This provides **universal coverage** (all inputs) rather than sampling.

```
If Verus verifies:  pre_original <==> pre_gen  AND  post_original <==> post_gen
Then: Generated specs are semantically equivalent to original specs
```

#### Verus Equivalence Template

```verus
use vstd::prelude::*;

verus! {

// Original precondition (from ground truth)
spec fn pre_original(arr: Seq<u64>, i: int, ret: u64) -> bool {
    arr.len() > 0 && i < arr.len() as int  // (#PRE) - filled from original spec
}

// Generated precondition (from model output)
spec fn pre_gen(arr: Seq<u64>, i: int, ret: u64) -> bool {
    i >= 0 && i < arr.len() as int && arr.len() > 0  // (#PRE) - filled from generated spec
}

// Prove precondition equivalence
proof fn pre_eq(arr: Seq<u64>, i: int, ret: u64)
    ensures pre_original(arr, i, ret) <==> pre_gen(arr, i, ret)
{
}

// Original postcondition
spec fn post_original(arr: Seq<u64>, i: int, ret: u64) -> bool
    recommends pre_original(arr, i, ret)
{
    ret == arr[i]  // (#POST) - filled from original spec
}

// Generated postcondition
spec fn post_gen(arr: Seq<u64>, i: int, ret: u64) -> bool
    recommends pre_original(arr, i, ret)
{
    ret == arr[i as int]  // (#POST) - filled from generated spec
}

// Prove postcondition equivalence
proof fn post_eq(arr: Seq<u64>, i: int, ret: u64)
    requires pre_original(arr, i, ret)
    requires pre_gen(arr, i, ret)
    ensures post_original(arr, i, ret) <==> post_gen(arr, i, ret)
{
}

} // verus!
```

#### Template Evaluation Code

```python
import subprocess
import tempfile
import re
from typing import Tuple, Optional, Dict

def extract_requires(spec: str) -> str:
    """Extract requires clause from specification."""
    match = re.search(r'requires\s+(.+?)(?=ensures|decreases|\{|$)', spec, re.DOTALL)
    if match:
        return match.group(1).strip().rstrip(',')
    return "true"

def extract_ensures(spec: str) -> str:
    """Extract ensures clause from specification."""
    match = re.search(r'ensures\s+(.+?)(?=decreases|\{|$)', spec, re.DOTALL)
    if match:
        return match.group(1).strip().rstrip(',')
    return "true"

def generate_equiv_check_code(
    param_types: str,  # e.g., "arr: Seq<u64>, i: int, ret: u64"
    original_spec: str,
    generated_spec: str
) -> str:
    """Generate Verus code to check spec equivalence."""
    
    pre_orig = extract_requires(original_spec)
    pre_gen = extract_requires(generated_spec)
    post_orig = extract_ensures(original_spec)
    post_gen = extract_ensures(generated_spec)
    
    # Build parameter list without types for function calls
    params = ", ".join(p.split(":")[0].strip() for p in param_types.split(","))
    
    template = f'''use vstd::prelude::*;

verus! {{

spec fn pre_original({param_types}) -> bool {{
    {pre_orig}
}}

spec fn pre_gen({param_types}) -> bool {{
    {pre_gen}
}}

proof fn pre_eq({param_types})
    ensures pre_original({params}) <==> pre_gen({params})
{{
}}

spec fn post_original({param_types}) -> bool
    recommends pre_original({params})
{{
    {post_orig}
}}

spec fn post_gen({param_types}) -> bool
    recommends pre_original({params})
{{
    {post_gen}
}}

proof fn post_eq({param_types})
    requires pre_original({params})
    requires pre_gen({params})
    ensures post_original({params}) <==> post_gen({params})
{{
}}

}} // verus!
'''
    return template

def equiv_test_spec(
    original_spec: str,
    generated_spec: str,
    param_types: str,
    verus_path: str = "verus",
    verbose: int = 0
) -> Dict[str, bool]:
    """
    Test if generated spec is equivalent to original spec using Verus verification.
    
    Returns:
        Dict with 'equivalent', 'pre_equiv', 'post_equiv' boolean results
    """
    check_code = generate_equiv_check_code(param_types, original_spec, generated_spec)
    
    if verbose >= 2:
        print("=== Generated equivalence check code ===")
        print(check_code)
    
    result = run_verus(check_code)
    verified = result["success"]
    
    if verbose >= 1:
        print(f"stdout: {result['stdout']}")
        print(f"stderr: {result['stderr']}")
    
    return {
        "equivalent": verified,
        "pre_equiv": verified,
        "post_equiv": verified,
        "check_code": check_code,
        "verus_output": result
    }
```

### Secondary Evaluation: LLM-Guided Equivalence Checking

**Second Priority**: When template equivalence fails to verify automatically, use an LLM to iteratively find either a counter-example or construct a proof of equivalence.

#### Method: LLM + Verifier Iteration

1. **Input**: Ground truth spec + generated spec + verifier error from template method
2. **LLM Task**: Analyze the specs and either:
   - **Find a counter-example**: Concrete inputs where specs differ
   - **Construct a proof**: Provide hints/assertions to help Verus verify equivalence
3. **Iterate**: LLM refines based on local verifier execution feedback

```
┌─────────────────────────────────────────────────────────────┐
│  Ground Truth Spec + Generated Spec + Verifier Error        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  LLM Analysis: Are these specs semantically equivalent?     │
│  - If NO: Provide counter-example (input where they differ) │
│  - If YES: Provide proof hints (assertions, lemmas)         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Local Verus Execution: Validate LLM's answer               │
│  - Counter-example: Check if specs actually differ          │
│  - Proof hints: Check if equivalence now verifies           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                   ┌──────────┴──────────┐
                   │                     │
              [Success]             [Failure]
                   │                     │
                   ▼                     ▼
              Return result      Iterate with feedback
                                 (max N iterations)
```

#### LLM Prompt Template

```python
LLM_EQUIV_PROMPT = """
You are analyzing two Verus specifications for semantic equivalence.

## Function Signature
{signature}

## Ground Truth Specification
{ground_truth_spec}

## Generated Specification  
{generated_spec}

## Previous Verifier Output (if any)
{verifier_error}

## Task
Determine if these specifications are semantically equivalent.

1. If they are NOT equivalent:
   - Provide a concrete counter-example: specific input values where the specs behave differently
   - Explain which spec accepts/rejects the input and why

2. If they ARE equivalent but verification failed:
   - Provide proof hints (assertions, lemmas, or intermediate steps) to help Verus verify
   - The hints should be inserted into the proof functions

## Output Format
```json
{
  "verdict": "equivalent" | "not_equivalent" | "unknown",
  "counter_example": {
    "inputs": {"arr": [...], "i": ..., ...},
    "ground_truth_result": true/false,
    "generated_result": true/false,
    "explanation": "..."
  },
  "proof_hints": [
    "assert(...);",
    "// lemma call or intermediate assertion"
  ],
  "reasoning": "Step-by-step analysis..."
}
```
"""
```

#### LLM-Guided Evaluation Code

```python
import json
from typing import Dict, List, Optional

def llm_guided_equiv_check(
    signature: str,
    ground_truth_spec: str,
    generated_spec: str,
    param_types: str,
    verus_path: str = "verus",
    max_iterations: int = 3,
    verbose: int = 0
) -> Dict:
    """
    Use LLM to find counter-example or construct equivalence proof.
    Iterates based on verifier feedback.
    """
    verifier_error = ""
    
    for iteration in range(max_iterations):
        if verbose >= 1:
            print(f"=== LLM Iteration {iteration + 1}/{max_iterations} ===")
        
        # 1. Query LLM
        llm_response = query_llm_for_equivalence(
            signature=signature,
            ground_truth_spec=ground_truth_spec,
            generated_spec=generated_spec,
            verifier_error=verifier_error
        )
        
        if verbose >= 2:
            print(f"LLM Response: {json.dumps(llm_response, indent=2)}")
        
        # 2. Handle LLM verdict
        if llm_response["verdict"] == "not_equivalent":
            # Validate counter-example
            counter_example = llm_response.get("counter_example", {})
            if validate_counter_example(
                ground_truth_spec, generated_spec, 
                counter_example, param_types
            ):
                return {
                    "equivalent": False,
                    "method": "llm_counter_example",
                    "counter_example": counter_example,
                    "iterations": iteration + 1
                }
            else:
                verifier_error = "Counter-example validation failed. Please reconsider."
                continue
        
        elif llm_response["verdict"] == "equivalent":
            # Try to verify with proof hints
            proof_hints = llm_response.get("proof_hints", [])
            check_code = generate_equiv_check_with_hints(
                param_types, ground_truth_spec, generated_spec, proof_hints
            )
            
            result = run_verus(check_code)
            
            if result["success"]:
                return {
                    "equivalent": True,
                    "method": "llm_proof",
                    "proof_hints": proof_hints,
                    "iterations": iteration + 1
                }
            else:
                verifier_error = result.get("stderr", "Verification failed")
                continue
        
        else:  # unknown
            verifier_error = f"LLM uncertain. Reasoning: {llm_response.get('reasoning', '')}"
            continue
    
    # Max iterations reached
    return {
        "equivalent": None,
        "method": "llm_inconclusive",
        "iterations": max_iterations,
        "last_error": verifier_error
    }

def query_llm_for_equivalence(
    signature: str,
    ground_truth_spec: str,
    generated_spec: str,
    verifier_error: str
) -> Dict:
    """Query LLM with the equivalence checking prompt."""
    prompt = LLM_EQUIV_PROMPT.format(
        signature=signature,
        ground_truth_spec=ground_truth_spec,
        generated_spec=generated_spec,
        verifier_error=verifier_error or "N/A (first attempt)"
    )
    
    # Call your LLM API (e.g., OpenAI, Anthropic, local model)
    response = call_llm_api(prompt)
    
    # Parse JSON response
    return json.loads(response)

def validate_counter_example(
    ground_truth_spec: str,
    generated_spec: str,
    counter_example: Dict,
    param_types: str
) -> bool:
    """
    Validate that the counter-example actually demonstrates non-equivalence.
    Execute both specs on the counter-example inputs.
    """
    inputs = counter_example.get("inputs", {})
    
    # Generate Verus code to evaluate specs on concrete inputs
    eval_code = generate_spec_evaluation_code(
        ground_truth_spec, generated_spec, inputs, param_types
    )
    
    result = run_verus(eval_code)
    
    # Check if specs actually differ on this input
    return result["success"] and specs_differ_on_input(result)

def generate_equiv_check_with_hints(
    param_types: str,
    ground_truth_spec: str,
    generated_spec: str,
    proof_hints: List[str]
) -> str:
    """Generate equivalence check code with LLM-provided proof hints."""
    hints_code = "\n    ".join(proof_hints) if proof_hints else ""
    
    params = ", ".join(p.split(":")[0].strip() for p in param_types.split(","))
    pre_orig = extract_requires(ground_truth_spec)
    pre_gen = extract_requires(generated_spec)
    post_orig = extract_ensures(ground_truth_spec)
    post_gen = extract_ensures(generated_spec)
    
    template = f'''use vstd::prelude::*;

verus! {{

spec fn pre_original({param_types}) -> bool {{
    {pre_orig}
}}

spec fn pre_gen({param_types}) -> bool {{
    {pre_gen}
}}

proof fn pre_eq({param_types})
    ensures pre_original({params}) <==> pre_gen({params})
{{
    {hints_code}
}}

spec fn post_original({param_types}) -> bool
    recommends pre_original({params})
{{
    {post_orig}
}}

spec fn post_gen({param_types}) -> bool
    recommends pre_original({params})
{{
    {post_gen}
}}

proof fn post_eq({param_types})
    requires pre_original({params})
    requires pre_gen({params})
    ensures post_original({params}) <==> post_gen({params})
{{
    {hints_code}
}}

}} // verus!
'''
    return template
```

#### Example: LLM Counter-Example

```
Ground Truth: requires arr@.len() > 0, i < arr@.len() as int
Generated:    requires i >= 0, i < arr@.len() as int

LLM Response:
{
  "verdict": "not_equivalent",
  "counter_example": {
    "inputs": {"arr": "Seq::empty()", "i": -1},
    "ground_truth_result": false,  // rejected by arr@.len() > 0
    "generated_result": false,     // rejected by i >= 0
    "explanation": "Both reject, but for different reasons. Try arr=[1], i=-1"
  },
  "reasoning": "Ground truth requires non-empty array, generated requires non-negative index. 
                These are independent constraints."
}
```

#### Example: LLM Proof Hints

```
Ground Truth: ensures ret >= 0
Generated:    ensures ret == if x < 0 { -x } else { x }

LLM Response:
{
  "verdict": "equivalent",
  "proof_hints": [
    "assert(x < 0 ==> -x >= 0);",
    "assert(x >= 0 ==> x >= 0);"
  ],
  "reasoning": "The generated spec is more precise but implies ret >= 0. 
                Need hints to show absolute value is always non-negative."
}
```

#### Comparison: All Three Methods

| Aspect | Template Equivalence | LLM-Guided | Proxy Testing |
|--------|---------------------|------------|---------------|
| **Priority** | 1st (Primary) | 2nd (Secondary) | 3rd (Fallback) |
| **Coverage** | Universal | Universal | Limited |
| **Precision** | Exact | Exact (if verified) | Approximate |
| **Handles Complex Proofs** | No | Yes (with hints) | N/A |
| **Finds Counter-Examples** | No | Yes | Partial |
| **Cost** | Low (just verifier) | Medium (LLM + verifier) | Low |
| **Use Case** | Simple equivalences | Complex proofs, debugging | Partial credit |

### Fallback Evaluation: Positive/Negative Proxy Testing

**Third Priority**: Test generated specs against positive and negative input/output pairs. Use as fallback when both template equivalence and LLM-guided methods fail.

#### Method: Executable Proxy

1. **Positive Proxies**: Valid (input, output) pairs that the function actually produces
   - Generated specs should **accept** these (i.e., inputs satisfy `requires`, outputs satisfy `ensures`)
   
2. **Negative Proxies**: Invalid (input, output) pairs
   - Generated specs should **reject** these (i.e., precondition fails OR postcondition fails)

```
Soundness:    Specs accept all positive proxies (no false negatives)
Completeness: Specs reject all negative proxies (no false positives)
```

#### Proxy Test Cases

| Proxy Type | Example | Expected |
|------------|---------|----------|
| **Positive** | `arr=[1,2,3], i=1 → ret=2` | Specs accept |
| **Negative (bad input)** | `arr=[], i=0 → ret=?` | `requires` rejects |
| **Negative (bad output)** | `arr=[1,2,3], i=1 → ret=99` | `ensures` rejects |

#### Proxy Testing Code

```python
def evaluate_proxy_testing(input_code, generated_specs, original_code):
    """
    Evaluate specs using positive/negative proxies (fallback method).
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

### Combined Evaluation Strategy

```python
def evaluate_task_a(
    original_code: str,
    generated_specs: str,
    original_specs: str,
    signature: str,
    param_types: str
) -> Dict:
    """
    Evaluate Task A with three-tier strategy:
    1. Template equivalence (primary)
    2. LLM-guided equivalence (secondary)
    3. Proxy testing (fallback)
    """
    
    # ===== Priority 1: Template-based equivalence =====
    template_result = equiv_test_spec(original_specs, generated_specs, param_types)
    
    if template_result["equivalent"]:
        return {
            "score": 1.0,
            "method": "template_equiv",
            "confidence": "high",
            "details": template_result
        }
    
    # ===== Priority 2: LLM-guided equivalence checking =====
    llm_result = llm_guided_equiv_check(
        signature=signature,
        ground_truth_spec=original_specs,
        generated_spec=generated_specs,
        param_types=param_types,
        max_iterations=3
    )
    
    if llm_result["equivalent"] is True:
        return {
            "score": 1.0,
            "method": "llm_proof",
            "confidence": "high",
            "details": llm_result
        }
    elif llm_result["equivalent"] is False:
        return {
            "score": 0.0,
            "method": "llm_counter_example",
            "confidence": "high",
            "counter_example": llm_result.get("counter_example"),
            "details": llm_result
        }
    
    # ===== Priority 3: Proxy testing (fallback for partial credit) =====
    positive_proxies = generate_positive_proxies(original_code, num=50)
    negative_proxies = generate_negative_proxies(original_code, num=50)
    
    soundness = sum(1 for p in positive_proxies if specs_accept(generated_specs, *p)) / len(positive_proxies)
    completeness = sum(1 for p in negative_proxies if specs_reject(generated_specs, *p)) / len(negative_proxies)
    
    return {
        "score": (soundness + completeness) / 2,
        "method": "proxy_test",
        "confidence": "medium",
        "soundness": soundness,
        "completeness": completeness,
        "template_error": template_result.get("verus_output", {}).get("stderr"),
        "llm_result": llm_result
    }
```

#### Evaluation Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Task A: Spec Evaluation                          │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Priority 1: Template Equivalence (Verus verification)              │
│  - Generate equivalence template                                    │
│  - Run Verus to prove pre <==> pre' and post <==> post'            │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
               [Verified]              [Failed to verify]
                    │                         │
                    ▼                         ▼
           Return score=1.0     ┌─────────────────────────────────────┐
           confidence=high      │  Priority 2: LLM-Guided Checking    │
                                │  - Query LLM with specs + error     │
                                │  - LLM provides counter-example OR  │
                                │    proof hints                      │
                                │  - Validate with Verus (iterate)    │
                                └─────────────────────────────────────┘
                                              │
                           ┌──────────────────┼──────────────────┐
                           │                  │                  │
                    [Proof found]    [Counter-example]    [Inconclusive]
                           │                  │                  │
                           ▼                  ▼                  ▼
                    Return score=1.0   Return score=0.0   ┌─────────────┐
                    confidence=high    confidence=high    │  Priority 3 │
                                                          │  Proxy Test │
                                                          └─────────────┘
                                                                 │
                                                                 ▼
                                                    Return score=(sound+compl)/2
                                                    confidence=medium
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

### Advanced Evaluation: Fuzzing + Kani Model Checking

**Enhanced Method**: Combine coverage-guided fuzzing with [Kani Rust Verifier](https://github.com/model-checking/kani) for stronger functional equivalence guarantees.

#### Why Combine Fuzzing + Kani?

| Method | Strengths | Limitations |
|--------|-----------|-------------|
| **Fuzzing** | Fast, finds edge cases, high throughput | Only tests sampled inputs |
| **Kani** | Proves correctness for all inputs (bounded) | Slower, may hit solver limits |
| **Combined** | Best of both: quick failures + formal proof | Comprehensive coverage |

#### Method: Two-Phase Equivalence Checking

```
┌─────────────────────────────────────────────────────────────────────┐
│  Phase 1: Fuzzing (Quick Differential Testing)                      │
│  - Use cargo-fuzz/libFuzzer to generate diverse inputs              │
│  - Run both original and generated code on same inputs              │
│  - If outputs differ → found counter-example → NOT equivalent       │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │                         │
           [Counter-example found]    [No counter-example]
                    │                         │
                    ▼                         ▼
           Return: NOT equivalent    ┌─────────────────────────────────┐
           confidence=high           │  Phase 2: Kani Model Checking   │
                                     │  - Generate equivalence harness │
                                     │  - Use kani::any() for inputs   │
                                     │  - Assert outputs are equal     │
                                     │  - Bounded model checking       │
                                     └─────────────────────────────────┘
                                                    │
                                       ┌────────────┴────────────┐
                                       │                         │
                                  [Verified]              [Counter-example]
                                       │                         │
                                       ▼                         ▼
                               Return: EQUIVALENT         Return: NOT equivalent
                               confidence=high            confidence=high
```

#### Kani Equivalence Harness

```rust
// equivalence_harness.rs
// Generated harness to prove functional equivalence using Kani

use kani::*;

// Original function (from ground truth)
fn original_get_element(arr: &[u64], i: usize) -> u64 {
    arr[i]
}

// Generated function (from model output)
fn generated_get_element(arr: &[u64], i: usize) -> u64 {
    arr[i]  // May have different implementation
}

#[kani::proof]
#[kani::unwind(10)]  // Bound for loops/recursion
fn check_equivalence() {
    // Generate arbitrary inputs satisfying preconditions
    let len: usize = kani::any();
    kani::assume(len > 0 && len <= 10);  // Bounded for tractability
    
    let arr: [u64; 10] = kani::any();
    let arr_slice = &arr[..len];
    
    let i: usize = kani::any();
    kani::assume(i < len);  // Satisfies requires clause
    
    // Call both functions
    let original_result = original_get_element(arr_slice, i);
    let generated_result = generated_get_element(arr_slice, i);
    
    // Assert functional equivalence
    assert_eq!(original_result, generated_result, 
        "Functions produce different outputs!");
}

#[kani::proof]
fn check_no_panic() {
    // Verify generated code doesn't panic on valid inputs
    let len: usize = kani::any();
    kani::assume(len > 0 && len <= 10);
    
    let arr: [u64; 10] = kani::any();
    let i: usize = kani::any();
    kani::assume(i < len);
    
    // Should not panic
    let _ = generated_get_element(&arr[..len], i);
}
```

#### Implementation Code

```python
import subprocess
import tempfile
import os
from typing import Dict, List, Tuple, Optional

def evaluate_task_b_advanced(
    generated_code: str,
    original_code: str,
    input_spec: str,
    fuzz_duration: int = 60,
    kani_unwind: int = 10
) -> Dict:
    """
    Evaluate Task B with Fuzzing + Kani combined approach.
    
    Phase 1: Fuzzing for quick counter-example detection
    Phase 2: Kani for bounded model checking proof
    """
    
    # ===== Phase 1: Fuzzing =====
    fuzz_result = run_differential_fuzzing(
        original_code, generated_code, 
        duration_secs=fuzz_duration
    )
    
    if fuzz_result["counter_example"]:
        return {
            "equivalent": False,
            "method": "fuzzing",
            "confidence": "high",
            "counter_example": fuzz_result["counter_example"],
            "fuzz_iterations": fuzz_result["iterations"]
        }
    
    # ===== Phase 2: Kani Model Checking =====
    kani_result = run_kani_equivalence_check(
        original_code, generated_code,
        input_spec, unwind=kani_unwind
    )
    
    if kani_result["verified"]:
        return {
            "equivalent": True,
            "method": "kani_proof",
            "confidence": "high",
            "bounds": {"unwind": kani_unwind},
            "fuzz_iterations": fuzz_result["iterations"]
        }
    elif kani_result["counter_example"]:
        return {
            "equivalent": False,
            "method": "kani_counter_example",
            "confidence": "high",
            "counter_example": kani_result["counter_example"]
        }
    else:
        # Inconclusive (solver timeout, resource limits)
        return {
            "equivalent": None,
            "method": "inconclusive",
            "confidence": "low",
            "fuzz_iterations": fuzz_result["iterations"],
            "kani_error": kani_result.get("error")
        }

def run_differential_fuzzing(
    original_code: str,
    generated_code: str,
    duration_secs: int = 60
) -> Dict:
    """
    Run differential fuzzing to find inputs where functions differ.
    Uses cargo-fuzz with libFuzzer backend.
    """
    # Create temporary Cargo project with fuzz target
    with tempfile.TemporaryDirectory() as tmpdir:
        # Setup fuzz target
        fuzz_target = generate_fuzz_target(original_code, generated_code)
        setup_fuzz_project(tmpdir, fuzz_target)
        
        # Run cargo fuzz
        result = subprocess.run(
            ["cargo", "fuzz", "run", "diff_fuzz", 
             f"--max-total-time={duration_secs}"],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=duration_secs + 30
        )
        
        # Parse results
        if "SUMMARY: " in result.stderr and "crash" in result.stderr.lower():
            counter_example = extract_crash_input(tmpdir)
            return {
                "counter_example": counter_example,
                "iterations": parse_fuzz_iterations(result.stderr)
            }
        
        return {
            "counter_example": None,
            "iterations": parse_fuzz_iterations(result.stderr)
        }

def generate_fuzz_target(original_code: str, generated_code: str) -> str:
    """Generate cargo-fuzz target for differential testing."""
    return f'''
#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

// Strip Verus annotations from both functions
{strip_verus_annotations(original_code).replace("fn ", "fn original_")}

{strip_verus_annotations(generated_code).replace("fn ", "fn generated_")}

#[derive(Arbitrary, Debug)]
struct FuzzInput {{
    // Input structure matching function parameters
    arr: Vec<u64>,
    i: usize,
}}

fuzz_target!(|input: FuzzInput| {{
    // Skip invalid inputs (violate preconditions)
    if input.arr.is_empty() || input.i >= input.arr.len() {{
        return;
    }}
    
    let orig = original_get_element(&input.arr, input.i);
    let gen = generated_get_element(&input.arr, input.i);
    
    assert_eq!(orig, gen, "Differential: functions differ on input {{:?}}", input);
}});
'''

def run_kani_equivalence_check(
    original_code: str,
    generated_code: str,
    input_spec: str,
    unwind: int = 10
) -> Dict:
    """
    Run Kani model checker to prove functional equivalence.
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        # Generate Kani harness
        harness = generate_kani_harness(
            original_code, generated_code, input_spec, unwind
        )
        
        harness_path = os.path.join(tmpdir, "equivalence.rs")
        with open(harness_path, "w") as f:
            f.write(harness)
        
        # Run Kani
        result = subprocess.run(
            ["cargo", "kani", "--harness", "check_equivalence",
             f"--unwind={unwind}", "--output-format=json"],
            cwd=tmpdir,
            capture_output=True,
            text=True,
            timeout=300  # 5 minute timeout
        )
        
        # Parse Kani output
        if result.returncode == 0:
            return {"verified": True, "counter_example": None}
        
        # Check for counter-example
        if "VERIFICATION FAILED" in result.stdout:
            counter_example = extract_kani_counter_example(result.stdout)
            return {"verified": False, "counter_example": counter_example}
        
        return {"verified": False, "counter_example": None, "error": result.stderr}

def generate_kani_harness(
    original_code: str,
    generated_code: str,
    input_spec: str,
    unwind: int
) -> str:
    """Generate Kani proof harness for equivalence checking."""
    # Parse input spec to extract parameter types and constraints
    params = parse_function_params(input_spec)
    constraints = parse_requires_clause(input_spec)
    
    return f'''
use kani::*;

// Original implementation
{strip_verus_annotations(original_code).replace("fn ", "fn original_")}

// Generated implementation  
{strip_verus_annotations(generated_code).replace("fn ", "fn generated_")}

#[kani::proof]
#[kani::unwind({unwind})]
fn check_equivalence() {{
    // Generate symbolic inputs
    {generate_kani_inputs(params)}
    
    // Apply preconditions
    {generate_kani_assumes(constraints)}
    
    // Call both implementations
    let orig_result = original_{params['fn_name']}({params['call_args']});
    let gen_result = generated_{params['fn_name']}({params['call_args']});
    
    // Assert equivalence
    assert!(orig_result == gen_result, 
        "Functions are not equivalent!");
}}

#[kani::proof]
#[kani::unwind({unwind})]
fn check_generated_safety() {{
    // Verify generated code doesn't exhibit undefined behavior
    {generate_kani_inputs(params)}
    {generate_kani_assumes(constraints)}
    
    // Should complete without UB
    let _ = generated_{params['fn_name']}({params['call_args']});
}}
'''
```

#### Comparison: Evaluation Methods for Task B

| Method | Coverage | Speed | Confidence | Use Case |
|--------|----------|-------|------------|----------|
| **Simple Execution** | Sampled | Fast | Medium | Quick sanity check |
| **Fuzzing Only** | High (random) | Fast | Medium-High | Find edge cases |
| **Kani Only** | Universal (bounded) | Slow | High | Formal proof |
| **Fuzzing + Kani** | Universal (bounded) | Medium | High | **Production evaluation** |

#### Combined Evaluation Strategy for Task B

```python
def evaluate_task_b(
    generated_code: str,
    original_code: str,
    input_spec: str
) -> Dict:
    """
    Three-tier evaluation for Task B:
    1. Quick execution test (sanity check)
    2. Differential fuzzing (find edge cases)
    3. Kani model checking (formal proof)
    """
    
    # ===== Tier 1: Quick Execution Test =====
    quick_result = quick_execution_test(original_code, generated_code, num_tests=100)
    
    if not quick_result["all_match"]:
        return {
            "score": 0.0,
            "method": "quick_test",
            "confidence": "high",
            "counter_example": quick_result["first_mismatch"]
        }
    
    # ===== Tier 2: Differential Fuzzing =====
    fuzz_result = run_differential_fuzzing(original_code, generated_code, duration_secs=30)
    
    if fuzz_result["counter_example"]:
        return {
            "score": 0.0,
            "method": "fuzzing",
            "confidence": "high", 
            "counter_example": fuzz_result["counter_example"]
        }
    
    # ===== Tier 3: Kani Model Checking =====
    kani_result = run_kani_equivalence_check(original_code, generated_code, input_spec)
    
    if kani_result["verified"]:
        return {
            "score": 1.0,
            "method": "kani_proof",
            "confidence": "very_high",
            "proof_bounds": kani_result.get("bounds")
        }
    elif kani_result["counter_example"]:
        return {
            "score": 0.0,
            "method": "kani_counter_example",
            "confidence": "high",
            "counter_example": kani_result["counter_example"]
        }
    
    # Inconclusive - return fuzzing confidence
    return {
        "score": 0.8,  # High confidence from fuzzing, but not proven
        "method": "fuzzing_only",
        "confidence": "medium",
        "note": "Kani inconclusive, relying on fuzzing results"
    }
```

#### Running Kani

```bash
# Install Kani
cargo install --locked kani-verifier
cargo kani setup

# Run equivalence check
cargo kani --harness check_equivalence --unwind 10

# Run with coverage
cargo kani --harness check_equivalence --coverage
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
| **A** | Code without specs | Just the specs | **1. Template equiv → 2. LLM-guided → 3. Proxy test** | Specs provably equivalent to original |
| **B** | Signature + specs | Full implementation | **1. Quick test → 2. Fuzzing → 3. Kani proof** | Functionally equivalent (bounded proof) |
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
