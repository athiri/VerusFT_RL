# Verified Sources Dataset

This directory contains 2,670 standalone verifiable Verus source files extracted from the min_dataset. All files have been verified to pass Verus verification when treated as library crates.

## Verification Status

- **Total files**: 2,670 Rust files
- **Verification rate**: 100% (2,670/2,670 pass)
- **Last verified**: 2026-01-13
- **Verus version**: 0.2026.01.02.6f52890

## Prerequisites

### Install Verus

You need Verus installed and available in your PATH. To check if Verus is installed:

```bash
verus --version
```

If not installed, follow the instructions at: https://github.com/verus-lang/verus

## How to Verify Files

### Verify a Single File

To verify any individual file in this directory:

```bash
verus --crate-type=lib <filename.rs>
```

**Example:**
```bash
verus --crate-type=lib complex_repos__verified-ironkv_args_t_f2429d08994c.rs
```

**Important**: The `--crate-type=lib` flag is required because these files are library code without `main()` functions.

### Verify All Files (Automated)

To verify all 2,670 files in parallel:

```bash
python3 ../verify_all_sources.py
```

This script will:
- Run Verus verification on all .rs files in parallel (8 workers)
- Display progress updates every 100 files
- Report success/failure statistics
- Identify any new failures or unexpected passes
- Complete in approximately 10-15 minutes

**Script location**: `/Users/chuyues/VerusFT_RL/workspace/min_dataset/verify_all_sources.py`

### Verify a Subset of Files

To verify specific files matching a pattern:

```bash
for file in complex_repos__verified-ironkv_*.rs; do
    echo "Verifying $file..."
    verus --crate-type=lib "$file"
done
```

To verify files from a specific source:

```bash
# Verify all coq_translation files
for file in coq_translation_*.rs; do
    verus --crate-type=lib "$file"
done

# Verify all complex_repos files
for file in complex_repos__*.rs; do
    verus --crate-type=lib "$file"
done
```

## Expected Output

### Successful Verification

When a file verifies successfully, you'll see output like:

```
verification results:: N verified, 0 errors
```

Where N is the number of verified functions/proofs in the file.

### Failed Verification

If a file fails (should not happen with current dataset), you'll see error messages indicating:
- Syntax errors
- Type errors
- Verification failures
- Timeout errors

## File Naming Convention

Files follow the pattern: `<source>__<identifier>_<hash>.rs`

- **source**: Origin of the file (e.g., `complex_repos`, `coq_translation`)
- **identifier**: Original filename or module path
- **hash**: Unique identifier for the file

Examples:
- `complex_repos__verified-ironkv_args_t_f2429d08994c.rs`
- `coq_translation_all_pass_effb319cf823.rs`

## Dataset Characteristics

This dataset contains:
- **Standalone verification**: Each file verifies independently without external dependencies
- **Diverse verification patterns**: Includes various Verus features (specs, proofs, loops, invariants)
- **Real-world code**: Extracted from actual verified projects
- **Complete specifications**: All necessary specifications included inline

## Troubleshooting

### Issue: "main function not found"

**Solution**: Make sure to use `--crate-type=lib` flag:
```bash
verus --crate-type=lib file.rs  # Correct
verus file.rs                    # Wrong - will fail
```

### Issue: Verification timeout

Some complex files may take longer. Increase timeout in the verification script or run individually:
```bash
# The script uses 60 second timeout per file by default
# Files should complete within this time
```

### Issue: Verus not found

**Solution**: Ensure Verus is in your PATH:
```bash
export PATH="/path/to/verus:$PATH"
```

Or use absolute path:
```bash
/path/to/verus --crate-type=lib file.rs
```

## Verification Script Details

The `verify_all_sources.py` script provides:
- **Parallel processing**: 8 concurrent verification processes
- **Progress tracking**: Updates every 100 files
- **Error reporting**: Captures and reports verification failures
- **Comparison with known failures**: Checks against FAILED_FILES.txt
- **Detailed statistics**: Pass/fail rates and timing information

## Related Files

- `FAILED_FILES.txt` - Verification status report (currently all files pass)
- `../verify_all_sources.py` - Automated verification script for all files
- `../verify_dataset_entries.py` - Original dataset verification script

## Usage Examples

### Quick Verification Test
```bash
# Verify a small sample
for file in $(ls *.rs | head -10); do
    echo "Testing $file"
    verus --crate-type=lib "$file" && echo "✓ Pass" || echo "✗ Fail"
done
```

### Full Dataset Verification
```bash
# Run complete verification suite
cd /Users/chuyues/VerusFT_RL/workspace/min_dataset
python3 verify_all_sources.py
```

### Check Specific File Details
```bash
# See detailed verification output
verus --crate-type=lib complex_repos__verified-ironkv_seq_lib_v_d34419b2766e.rs
```

## Performance Notes

- Single file verification: ~0.1-5 seconds per file (varies by complexity)
- Full dataset verification: ~10-15 minutes with parallel processing
- Memory usage: ~100-500MB per Verus process

## Dataset Quality Metrics

✓ All files are syntactically valid Rust
✓ All files pass Verus type checking
✓ All files have complete specifications
✓ All files verify their safety properties
✓ All files are standalone (no external dependencies)
✓ All files use `--crate-type=lib` (library crates)

## Questions or Issues?

If you encounter verification failures or have questions about specific files, check:
1. Verus version compatibility (tested with 0.2026.01.02.6f52890)
2. Using `--crate-type=lib` flag
3. File permissions and path correctness
4. FAILED_FILES.txt for known issues (currently none)
