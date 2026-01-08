# Verified Pre-Minimized Verus Dataset

This directory contains **6,675 minimal, verified Verus files** (≤100 lines each) that pass verification with at least one verified proof.

## Contents

- **`extracted_verified/`** - 6,675 minimal verified Verus source files (≤100 LOC each)
- **`fetch_sources.py`** - Script to extract files from source repositories
- **`verification.py`** - Core Verus verification logic
- **`analysis.py`** - File analysis and filtering utilities
- **`verification_results.json`** - Complete verification results for all 14,199 extracted files
- **`source_manifest.json`** - Source repository metadata

## Dataset Statistics

**Extraction Results:**
- 14,199 files extracted from 10 source repositories
- 739,239 lines of code total

**Verification Results:**
- ✅ **7,029 verified** (49.5%) - Files with ≥1 verified proof
- 📄 23 spec-only (0 proofs)
- ❌ 7,147 failed verification

**Minimal Filter Applied:**
- 📏 **6,675 minimal files** (≤100 lines) - included in dataset
- 354 files over 100 lines removed

## Source Repositories

Files extracted from:
1. **Vericoding** - 13,325 files - Competitive programming benchmarks
2. **HumanEval-Verus** - 174 files - Hand-verified benchmark problems
3. **Verus Core Tests** - 110 files - Verus test suite
4. **vstd** - 77 files - Verus standard library
5. **Microsoft Verismo** - 164 files - Verified confidential firmware
6. **IronKV** - 62 files - Verified key-value store
7. **Node Replication** - 30 files - Verified replication library
8. **Mimalloc** - 27 files - Verified memory allocator
9. **Asterinas VOSTD** - 116 files - Auto-generated specs
10. **VMware BetrFS** - 114 files - Verified file system components

## Verification Criteria

A file passes if:
1. Exit code = 0
2. No errors in output
3. Pattern: `verification results:: N verified, 0 errors` where N ≥ 1

## Reproduction

```bash
# 1. Extract files from source repositories
python3 fetch_sources.py

# 2. Verify extracted files (requires Verus installed)
python3 verify_extracted_parallel.py
```

## File Characteristics

Pre-minimized files contain:
- Original source code structure (not minimized)
- Complete Verus verification constructs
- Self-contained or standard dependencies only (vstd, std, core)
- Minimum 5 lines of code
- At least one Verus keyword

## Next Steps

These verified pre-minimized files can be:
- Used directly for training/fine-tuning
- Fed into minimization pipeline (see `run_minimizer.py`)
- Used as benchmarks for verification tools
- Studied as Verus examples
