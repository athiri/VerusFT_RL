# Verus Dataset Pipeline

Generate self-contained, verifiable Verus examples from source repos.

## Pipeline Overview

```
fetch_sources.py  →  extracted_verified/  →  run_minimizer.py  →  minimized_examples/
   (from repos)       (6,675 minimal        (+ creduce)           (further reduced)
                       verified files)
```

## Phase 1: Pre-Minimized Dataset (Current PR)

### Quick Start

```bash
# 1. Fetch and verify source files from Verus repositories
python3 fetch_sources.py

# 2. Results in extracted_verified/ (6,675 minimal verified files)
```

### Dataset: `extracted_verified/`

- **6,675 minimal verified files** (≤100 lines each)
- All pass Verus verification with ≥1 verified proof
- 47.0% pass rate from 14,199 extracted files

**Extraction pipeline:**
1. 14,199 files extracted from 10 source repositories
2. 7,029 files pass verification (49.5%)
3. 6,675 files are minimal (≤100 LOC) - **included in dataset**

### Source Repositories

| Repository | Files | Description |
|------------|-------|-------------|
| Vericoding | 13,325 | Competitive programming benchmarks |
| HumanEval-Verus | 174 | Human evaluation suite |
| Verus tests | 110 | Core Verus test suite |
| vstd | 77 | Verified standard library |
| Verismo | 164 | Microsoft verified firmware |
| IronKV | 62 | Verified key-value store |
| Node-replication | 30 | Verified NUMA replication |
| Mimalloc | 57 | Verified memory allocator |

---

## Phase 2: Post-Minimized Dataset (Future)

### Quick Start

```bash
# Run minimizer on extracted files (requires creduce + verus)
python3 run_minimizer.py

# Verify and report metrics
python3 verus_metrics.py
```

### Dataset: `minimized_examples/`

- **11,579 total files** (post-creduce minimization)
- **2,935 legitimate** (25.3%) - real implementations that verify
  - 2,844 fully clean (no `assume` statements)
  - 91 with arithmetic safety `assume` statements
- **8,644 stubs** (74.6%) - spec-only with `assume(false); unreached()`

### Quality Filter

```bash
# Get only legitimate training examples (no stubs)
grep -L -E "assume\(false\)|unreached\(\)" minimized_examples/*.rs > clean_files.txt
```

---

## Scripts

| File | Purpose |
|------|---------|
| `fetch_sources.py` | Download and verify Verus files from source repos |
| `verification.py` | Core Verus verification logic |
| `analysis.py` | Semantic analysis + stub detection |
| `verus_metrics.py` | Dataset metrics and quality reporting |
| `run_minimizer.py` | Run creduce minimization |

### External Dependencies

| Tool | Purpose | Install |
|------|---------|---------|
| `verus` | Verification | [verus-lang/verus](https://github.com/verus-lang/verus) |
| `creduce` | Minimization (Phase 2) | `brew install creduce` |

---

## Acceptance Criteria

### Verification Criteria

A file passes if:
1. Exit code = 0
2. No errors in output
3. Pattern `verification results:: N verified, 0 errors` with N ≥ 1

### Minimal Criteria

A file is **minimal** if:
- ≤100 lines of code

### Meaningful Criteria

Per `PROJECT_PROPOSAL.md`:

> "Meaningful: Has at least one `requires`/`ensures` clause OR a loop invariant OR a proof block"

### Stub Detection

Files containing these patterns are **stubs** (not suitable for SFT):
- `assume(false)`
- `unreached()`

---

## Output Files

| File | Contents |
|------|----------|
| `source_manifest.json` | Source repository metadata |
| `verification_results.json` | Complete verification results |
| `dataset_metadata.csv` | Per-file metrics (LOC, status, quality) |
| `verus_metrics.json` | Aggregated dataset statistics |
