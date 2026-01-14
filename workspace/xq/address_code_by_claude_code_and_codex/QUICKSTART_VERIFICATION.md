# Quick Start: Verus Allocator Verification

## TL;DR - Fastest Way to Test

```bash
cd allocator_test
cargo verus verify
```

⚠️ **Note**: This will encounter module visibility issues with `PageOrg`. For full verification, see the complete guide in [VERUS_VERIFICATION_README.md](VERUS_VERIFICATION_README.md).

## Quick Verification Options

### Option 1: Use the Verification Script (Easiest)

```bash
# Interactive menu
./verify_allocator.sh

# Or use command-line options
./verify_allocator.sh --test-project    # Quick test (has known issues)
./verify_allocator.sh --individual      # Test individual modules
./verify_allocator.sh --status          # Show what was fixed
```

### Option 2: Test a Single Module

Verify that the fixes work by testing a simple module:

```bash
# Create a test project
cargo verus new --lib test_simple
cd test_simple

# Update Cargo.toml
cat >> Cargo.toml << 'EOF'
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
libc = "0.2"
EOF

# Copy a simple module (e.g., config.rs)
cp ../allocator_inline_blocks/config.rs src/lib.rs

# Verify
cargo verus verify
```

### Option 3: Check What Was Fixed

View the changes made to modernize the code:

```bash
# Show differences in one file
git diff allocator_inline_blocks/config.rs

# Or use the script
./verify_allocator.sh --status
```

## Expected Results

### Successful Verification
```
verification results:: N verified, 0 errors
   Compiling your_project v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

### Common Warnings (OK to ignore)
```
warning: unexpected `cfg` condition value: `override_system_allocator`
warning: automatically chose triggers for this expression
```

These warnings are informational and don't indicate verification failures.

## What Files Were Modified?

All files in these directories have been fixed:
- ✅ `allocator_inline_blocks/*.rs` (27 files) - Individual modules
- ✅ `allocator_inline_full.rs` - Combined single file
- ✅ `allocator_test/src/lib.rs` - Test project

## Key Changes Made

1. **Import updates**: `verus_state_machines_macros` → `state_machines_macros`
2. **Removed**: Deprecated `#[is_variant]` attributes
3. **Method updates**: `.get_*()` → `.arrow_*()`
4. **Type checking**: `.is_*()` → `matches Pattern(..)`
5. **Syntax fixes**: Added parentheses for `matches` with `==>`

## Verification Times

| Module Type | Expected Time |
|------------|---------------|
| Simple (config, flags) | 5-30 seconds |
| Medium (types, tokens) | 30-90 seconds |
| Complex (segment, page_organization) | 2-10 minutes |
| Full project | 10-30 minutes |

## Need More Details?

See the complete guide: [VERUS_VERIFICATION_README.md](VERUS_VERIFICATION_README.md)

## Troubleshooting Quick Fixes

### "unresolved import `state_machines_macros`"
Add to Cargo.toml:
```toml
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
```

### "use of undeclared type `PageOrg`"
Use individual module files instead of the combined file. See full README for details.

### Verification hangs
Some modules take several minutes. Be patient or verify simpler modules first.

## Quick Command Reference

```bash
# Verify current project
cargo verus verify

# Verify with output to file
cargo verus verify 2>&1 | tee verification.log

# Create new Verus project
cargo verus new --lib project_name

# Check Verus version
cargo verus --version
```

## Status Summary

- ✅ All syntax modernized for latest Verus
- ✅ Individual module files ready for verification
- ⚠️ Combined file has known module visibility issues (use individual files)
- ✅ Test project configured with correct dependencies
