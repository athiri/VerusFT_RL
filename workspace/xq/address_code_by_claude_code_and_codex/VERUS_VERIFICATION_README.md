# Verus Allocator Verification Guide

This document explains how to verify the fixed Verus memory allocator implementation.

## Overview

The allocator code has been modernized to work with the latest version of Verus. All deprecated syntax has been updated and the code is ready for verification.

## Project Structure

```
allocator_inline_blocks/     # 27 individual module files (RECOMMENDED for verification)
├── types.rs
├── tokens.rs
├── page_organization.rs
├── config.rs
├── layout.rs
├── linked_list.rs
├── page.rs
├── segment.rs
├── queues.rs
├── init.rs
├── free.rs
├── alloc_fast.rs
├── alloc_generic.rs
├── realloc.rs
├── os_mem.rs
├── os_mem_util.rs
├── os_alloc.rs
├── os_commit.rs
├── commit_mask.rs
├── commit_segment.rs
├── arena.rs
├── bitmap.rs
├── bin_sizes.rs
├── dealloc_token.rs
├── flags.rs
├── pigeonhole.rs
└── thread.rs

allocator_inline_full.rs      # Single combined file (has module visibility issues)
allocator_test/               # Test Verus project (pre-configured)
```

## Prerequisites

1. **Install Verus**: Follow the installation instructions at https://github.com/verus-lang/verus
2. **Rust toolchain**: Ensure you have a recent Rust nightly toolchain installed
3. **Cargo verus**: The `cargo verus` command should be available after installing Verus

## Method 1: Using the Test Project (Quickest)

The easiest way to verify is using the pre-configured test project:

```bash
cd allocator_test
cargo verus verify
```

**Note**: The test project uses `allocator_inline_full.rs` which has known module visibility issues with the `PageOrg` state machine. Use Method 2 for full verification.

## Method 2: Multi-File Project (Recommended)

For proper verification of all modules, create a new Verus project with separate files:

### Step 1: Create a new Verus library project

```bash
cargo verus new --lib allocator_verify
cd allocator_verify
```

### Step 2: Update Cargo.toml

Edit `Cargo.toml` to add required dependencies:

```toml
[package]
name = "allocator_verify"
version = "0.1.0"
edition = "2021"

[dependencies]
vstd = "=0.0.0-2025-12-28-0056"
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
libc = "0.2"

[package.metadata.verus]
verify = true
```

### Step 3: Set up the source structure

Create a proper Rust module structure:

```bash
# Create a lib.rs that declares all modules
cat > src/lib.rs << 'EOF'
#![feature(core_intrinsics)]
#![feature(lazy_cell)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![feature(thread_id_value)]
#![feature(strict_provenance)]
#![verifier::exec_allows_no_decreases_clause]

mod os_mem;
mod config;
mod tokens;
mod layout;
mod bin_sizes;
mod commit_mask;
mod page_organization;
mod linked_list;
mod types;
mod arena;
mod os_mem_util;
mod bitmap;
mod pigeonhole;
mod flags;
mod dealloc_token;
mod thread;
mod segment;
mod page;
mod queues;
mod os_alloc;
mod os_commit;
mod commit_segment;
mod free;
mod alloc_fast;
mod alloc_generic;
mod realloc;
mod init;
EOF

# Copy the individual module files (without the mod declaration wrappers)
# You'll need to extract each module's content from the allocator_inline_blocks files
```

### Step 4: Prepare module files

Each file in `allocator_inline_blocks/` is wrapped with a `mod modulename { ... }` declaration. You need to:

1. Remove the outer `mod modulename {` and closing `}` from each file
2. Keep the inner `verus! { }` blocks
3. Copy the cleaned files to `src/`

Example for `config.rs`:
```bash
# Remove lines 1-14 (module declaration) and last 2 lines (closing braces)
tail -n +15 ../allocator_inline_blocks/config.rs | head -n -2 > src/config.rs
```

### Step 5: Run verification

```bash
cargo verus verify
```

## Method 3: Single File Verification (Limited)

If you want to quickly test a single module:

```bash
# Create a test project
cargo verus new --lib test_module
cd test_module

# Update Cargo.toml with dependencies (see Method 2, Step 2)

# Copy a single module file
cp ../allocator_inline_blocks/config.rs src/lib.rs

# Remove the outer mod declaration
sed -i '1,14d' src/lib.rs  # Remove first 14 lines
sed -i '$d' src/lib.rs     # Remove last line
sed -i '$d' src/lib.rs     # Remove second-to-last line

# Run verification
cargo verus verify
```

## What Was Fixed

The following modernizations were applied to all files:

### 1. State Machine Macro Import
```rust
// OLD (deprecated)
use verus_state_machines_macros::*;

// NEW (current)
use state_machines_macros::*;
```

### 2. Enum Variant Attribute
```rust
// OLD (deprecated)
#[is_variant]
pub ghost enum Popped { ... }

// NEW (current)
pub ghost enum Popped { ... }
```

### 3. Enum Accessor Methods
```rust
// OLD (deprecated)
popped.get_VeryUnready_0()
popped.get_Normal_1()

// NEW (current)
popped.arrow_VeryUnready_0()
popped.arrow_Normal_1()
```

### 4. Enum Type Checking
```rust
// OLD (deprecated)
if popped.is_VeryUnready() { ... }
if popped.is_SegmentCreating() { ... }

// NEW (current)
if popped matches Popped::VeryUnready(..) { ... }
if popped matches Popped::SegmentCreating(..) { ... }
```

### 5. Matches Expression Syntax
```rust
// OLD (syntax error)
success ==> popped matches Popped::VeryUnready(..) ==> result

// NEW (correct)
success ==> (popped matches Popped::VeryUnready(..)) ==> result
```

## Verification Output

Successful verification will show:
```
verification results:: N verified, 0 errors
   Compiling allocator_verify v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs
```

You may see warnings about `cfg` conditions or triggers - these are informational and don't affect verification success.

## Known Issues

### PageOrg Module Visibility (Single File Only)

The combined `allocator_inline_full.rs` file has a known issue where the `PageOrg` state machine is not properly visible across module boundaries. This is because:

1. `state_machine!` macro generates a module named `PageOrg`
2. In a single-file context, nested modules don't export properly
3. Cross-module references to `PageOrg::State` fail

**Solution**: Use Method 2 (multi-file project) where each module is in its own file.

### Long Verification Times

Some modules (especially `segment.rs`, `page_organization.rs`, and `linked_list.rs`) may take several minutes to verify due to complex proofs and state machines.

## Troubleshooting

### Error: "unresolved import `state_machines_macros`"

Make sure your `Cargo.toml` includes:
```toml
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
```

### Error: "use of undeclared type `PageOrg`"

You're likely using the single-file version. Use Method 2 (multi-file project) instead.

### Error: "matches with ==> is not allowed"

Make sure you've applied all the fixes. Search for patterns like:
```rust
condition ==> x matches Pattern(..) ==> result
```
And wrap the matches expression in parentheses:
```rust
condition ==> (x matches Pattern(..)) ==> result
```

## Performance Tips

1. **Verify modules individually**: Start with smaller modules like `config.rs` or `flags.rs`
2. **Use multiple cores**: Verus can verify functions in parallel
3. **Skip some modules**: If you only need to verify specific functionality, comment out unused modules in `lib.rs`

## Additional Resources

- Verus Documentation: https://verus-lang.github.io/verus/
- Verus GitHub: https://github.com/verus-lang/verus
- State Machines Guide: https://verus-lang.github.io/verus/state_machines/
- Original allocator (mimalloc): https://github.com/microsoft/mimalloc

## Contact

For issues with the Verus language or verification, please open an issue on the Verus GitHub repository.

For issues specific to this allocator implementation, refer to the original verified-memory-allocator project.
