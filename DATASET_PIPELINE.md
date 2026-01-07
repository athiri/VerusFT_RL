# Verus Dataset Pipeline

Generate self-contained, verifiable Verus examples from source repos.

## Pipeline

```
fetch_sources.py  →  extracted_examples/  →  run_minimizer.py  →  minimized_examples/
   (from repos)         (216 files)            + creduce            (81 files)
```

## Quick Start (Full Reproduction)

```bash
# 1. Fetch source files from verus-lang/verus
python3 fetch_sources.py

# 2. Run minimizer (requires creduce + verus)
python3 run_minimizer.py

# 3. Verify and report metrics
python3 verus_metrics.py
```

## Quick Start (Existing Dataset)

```bash
# Verify existing dataset
python3 verus_metrics.py

# Validate minimized files
python3 run_minimizer.py --validate-only
```

## Files

| File | Purpose |
|------|---------|
| `fetch_sources.py` | Download Verus files from source repos |
| `run_minimizer.py` | Run creduce + validate results |
| `verus_metrics.py` | Verify dataset, report metrics |
| `verification.py` | Core Verus verification logic |
| `analysis.py` | Semantic analysis utilities |

### External Dependencies

| Tool | Purpose | Install |
|------|---------|---------|
| `verus` | Verification | [verus-lang/verus](https://github.com/verus-lang/verus) |
| `creduce` | Minimization | `brew install creduce` or [creduce.org](https://creduce.org) |
| `git` | Source fetching | System package manager |

## Directories

| Directory | Contents |
|-----------|----------|
| `extracted_examples/` | Pre-minimization (source files from repos) |
| `minimized_examples/` | Post-minimization (81 quality-filtered files) |

## Dataset Stats

- **11,579 total examples** in `minimized_examples/`
- **2,935 legitimate** (25.3%) - real implementations that verify
  - 2,844 fully clean (no `assume` statements)
  - 91 with arithmetic safety `assume` statements (valid)
- **8,644 stubs** (74.6%) - trivially "verified" using `assume(false); unreached()`
  - ⚠️ **NOT suitable for SFT** - would teach model to output stubs

### Quality Filter

To get only legitimate training examples:
```bash
# Files without assume(false) or unreached() patterns
grep -L -E "assume\(false\)|unreached\(\)" minimized_examples/*.rs > clean_files.txt
# Result: 2,935 files suitable for training
```

### Sources
- verus-lang/verus (tests)
- microsoft/verismo (verified firmware)
- verus-lang/verified-node-replication
- Beneficial-AI-Foundation/vericoding (benchmarks)

## Verification Criteria

A file passes if:
1. Exit code = 0
2. No errors in output
3. Pattern `verification results:: N verified, 0 errors`

## Meaningful File Criteria

Per `PROJECT_PROPOSAL.md` acceptance criteria (line 126):

> "Meaningful: Has at least one `requires`/`ensures` clause OR a loop invariant OR a proof block"

A file is **meaningful** if it has at least one of:
- `requires` - precondition
- `ensures` - postcondition
- `invariant` - loop invariant
- `proof fn` - proof block

Files without these are **trivial** (e.g., only `assert` or `spec fn`).

## Features

- Strips comments/strings before keyword counting 
- Batch creduce minimization with validation
- Exit code + output parsing for status
- Semantic quality per proposal definition
- Verification count distribution (1, 2, 3+ verified)
- Metadata CSV export (`dataset_metadata.csv`)

