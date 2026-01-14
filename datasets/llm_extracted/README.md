# llm_extracted Dataset

This directory contains 9,126 Rust files with Verus specifications that are in various stages of completion.

## Verification Status

**Only 2,684 files (29.4%) pass current Verus verification**

## Why So Many Fail?

This dataset was originally extracted from a larger corpus and represents **intermediate/incomplete examples** rather than fully verified code. From the project history (commit 0e57703):

> "This is a subset of the 9,126 files in extracted_verified/. The remaining ~6,500 files fail with current Verus (mostly missing decreases clauses). These will be added after LLM-assisted repair (Method 3 from proposal)."

### Common Failure Patterns

1. **Missing `decreases` clauses** (1,497 files, ~23%)
   - Loops without termination proofs
   - Recursive functions without decreases annotations
   - Required by current Verus version but not in original extraction

2. **Syntax errors and incomplete code**
   - Malformed specifications
   - Missing semicolons or braces
   - Incomplete function bodies

3. **Type errors**
   - Using `nat`/`int` in exec mode
   - Mismatched types in specifications

4. **Verification failures**
   - Assertion failures
   - Incomplete proofs
   - Missing invariants

## Dataset Purpose

This dataset was designed for **training LLMs to complete/repair Verus specifications**, not as a collection of verified examples. The files represent:

- **Partial specifications** that need completion
- **Training examples** at various stages of verification
- **Repair targets** for LLM-assisted fixing

## Example: Missing Decreases Clause

```rust
// example_00001.rs - FAILS verification
while index < arr.len()
    invariant
        0 <= index <= arr.len(),
        forall|k: int| 0 <= k < index ==> !(is_even(#[trigger] arr[k])),
    // MISSING: decreases arr.len() - index,
{
    if (arr[index] % 2 == 0) {
        return true;
    }
    index += 1;
}
```

To fix, add:
```rust
while index < arr.len()
    invariant
        0 <= index <= arr.len(),
        forall|k: int| 0 <= k < index ==> !(is_even(#[trigger] arr[k])),
    decreases arr.len() - index,  // ADD THIS
{
    ...
}
```

## Recommended Usage

### For Training:
✓ **Use this dataset** - provides diverse examples of incomplete specifications
- Good for learning to add missing clauses
- Shows realistic partial verification attempts
- Useful for specification completion tasks

### For Evaluation/Benchmarking:
✗ **Do NOT use this dataset** - most files don't verify
- Use `/datasets/agent_generated/` (26 files, 100% pass)
- Use `/datasets/coq_translation/` (234 files, 100% pass)
- Use `/datasets/llm_generated/` (193 files, 100% pass)
- Use `/workspace/min_dataset/verified_sources/` (2,670 files, 100% pass)

### For Reference/Production:
✗ **Do NOT use this dataset** - not fully verified
- Use fully verified subsets mentioned above
- Or manually verify each file before use

## Repair Strategy

To make these files verifiable:

1. **Add decreases clauses** to loops and recursive functions
2. **Fix syntax errors** in specifications
3. **Complete partial implementations**
4. **Add missing invariants** and proof assertions
5. **Verify with current Verus toolchain**

## Verification Results

Run verification yourself:
```bash
# Verify all files (takes ~30 minutes)
python3 /Users/chuyues/VerusFT_RL/verify_datasets.py

# Verify single file
verus --crate-type=lib example_00001.rs
```

See `../VERIFICATION_STATUS.md` for complete results.

## Historical Context

This dataset originated from:
1. Large-scale extraction of Verus code snippets
2. LLM-based iterative refinement attempts
3. Incomplete repair/completion process
4. Archived as training data for specification completion

**Status**: Intentionally incomplete - designed for learning, not production use.
