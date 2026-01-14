# Verification Status for /datasets

This document provides the verification status of all .rs files in the `/datasets` directory.

**Last verified**: 2026-01-13
**Verus version**: 0.2026.01.02.6f52890
**Total files**: 9,579

## Summary

| Category | Total Files | Passed | Failed | Pass Rate |
|----------|-------------|--------|--------|-----------|
| **agent_generated** | 26 | 26 | 0 | **100%** ✓ |
| **coq_translation** | 234 | 234 | 0 | **100%** ✓ |
| **llm_extracted** | 9,126 | 2,684 | 6,442 | **29.4%** |
| **llm_generated** | 193 | 193 | 0 | **100%** ✓ |
| **TOTAL** | **9,579** | **3,137** | **6,442** | **32.7%** |

## Detailed Results

### ✓ agent_generated/ (100% pass)

All 26 files are standalone verifiable. These are LLM-generated examples from the CortenMM project covering memory management verification patterns.

**Verification**: All files pass with `verus --crate-type=lib`

### ✓ coq_translation/ (100% pass)

All 234 files are standalone verifiable. These are Verus translations from Coq verification examples covering:
- Verified Functional Algorithms (VFA)
- QuickChick property-based testing
- Software Foundations

**Verification**: All files pass with `verus --crate-type=lib`

### ⚠ llm_extracted/ (29.4% pass)

Only 2,684 out of 9,126 files pass standalone verification. The remaining 6,442 files have various issues:

**Common failure reasons**:
1. **Missing decreases clauses** (most common)
   - Loops without `decreases` clause for termination proof
   - Recursive functions without `decreases` clause

2. **Syntax errors**
   - Incomplete code snippets
   - Missing semicolons or braces

3. **Type errors**
   - Mismatched types
   - Invalid use of `nat`/`int` in exec mode

4. **Verification errors**
   - Assertion failures
   - Incomplete proofs

**Status**: These files appear to be training examples for learning to add specifications rather than complete standalone verified programs. They may require additional work to become fully verifiable.

**Example failure** (llm_extracted/example_00001.rs):
```rust
while index < arr.len()
    invariant
        0 <= index <= arr.len(),
        forall|k: int| 0 <= k < index ==> !(is_even(#[trigger] arr[k])),
{
    // Missing: decreases arr.len() - index,
    ...
}
```

### ✓ llm_generated/ (100% pass)

All 193 files are standalone verifiable. These are curated LLM-generated examples from:
- VeriSMo (70 files) - Verified hypervisor examples
- Verus-Bench (43 files) - Benchmark problems
- VeruSAGE-Bench (80 files) - Real-world verification projects

**Verification**: All files pass with `verus --crate-type=lib`

## Recommended Datasets for Standalone Verification

If you need **fully verified standalone examples**, use these subsets:

### High Quality (100% verified):
```bash
datasets/agent_generated/     # 26 files
datasets/coq_translation/     # 234 files
datasets/llm_generated/       # 193 files
# Total: 453 files, 100% verified
```

### Mixed Quality (includes partial examples):
```bash
datasets/llm_extracted/       # 9,126 files (29.4% verified)
# 2,684 files pass, 6,442 files fail
```

## Verification Methods

### Verify all files:
```bash
python3 /Users/chuyues/VerusFT_RL/verify_datasets.py
```

### Verify a single file:
```bash
verus --crate-type=lib datasets/path/to/file.rs
```

### Verify only high-quality subsets:
```bash
# Verify agent_generated
find datasets/agent_generated -name "*.rs" -exec verus --crate-type=lib {} \;

# Verify coq_translation
find datasets/coq_translation -name "*.rs" -exec verus --crate-type=lib {} \;

# Verify llm_generated
find datasets/llm_generated -name "*.rs" -exec verus --crate-type=lib {} \;
```

## Comparison with Other Datasets

### workspace/min_dataset/verified_sources (100% verified)
- **2,670 files**, all standalone verifiable
- Extracted from min_dataset training data
- High quality, complete specifications
- See: `workspace/min_dataset/verified_sources/README.md`

### datasets_merged (deduplicated, mixed quality)
- **8,833 unique files** after deduplication
- Merges `/datasets` and `/workspace/min_dataset/verified_sources`
- Contains the same quality distribution as `/datasets`
- See: `datasets_merged/README.md`

## Usage Recommendations

### For Training:
- Use **all datasets** including llm_extracted for diversity
- llm_extracted provides examples at various completion stages
- Good for learning to fix incomplete specifications

### For Evaluation/Benchmarking:
- Use **only 100% verified subsets** (453 files from agent_generated, coq_translation, llm_generated)
- Or use **workspace/min_dataset/verified_sources** (2,670 files)
- Ensures ground truth is actually verified

### For Production/Reference:
- Use **100% verified subsets only**
- Avoid llm_extracted unless you verify each file individually

## Fixing llm_extracted Files

Many llm_extracted failures can be fixed by adding:

1. **Decreases clauses for loops**:
   ```rust
   while condition
       invariant ...
       decreases expression,  // Add this
   { ... }
   ```

2. **Decreases clauses for recursion**:
   ```rust
   fn recursive_fn(n: nat)
       decreases n,  // Add this
   { ... }
   ```

3. **Removing incomplete annotations** or **completing proofs**

See `verification_results.json` for the complete list of failures.

## Files

- `verify_datasets.py` - Verification script for all files in /datasets
- `verification_results.json` - Detailed verification results (auto-generated)
- `VERIFICATION_STATUS.md` - This document

## Related Documentation

- `/workspace/min_dataset/verified_sources/README.md` - 100% verified dataset
- `/datasets/README.md` - Dataset overview
- `/datasets_merged/README.md` - Deduplicated dataset
