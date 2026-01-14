#!/bin/bash
# Complete verification script for all allocator blocks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================"
echo "Verify All Allocator Blocks"
echo "========================================"
echo ""

# Check if cargo verus is available
if ! command -v cargo-verus &> /dev/null; then
    echo "❌ Error: cargo verus not found!"
    echo "Please install Verus from: https://github.com/verus-lang/verus"
    exit 1
fi

echo "✓ Verus installation found"
echo ""

# Create a temporary directory for the project
TEMP_DIR=$(mktemp -d)
PROJECT_DIR="$TEMP_DIR/allocator_full_verify"

echo "Creating multi-file Verus project..."
echo "Location: $PROJECT_DIR"
echo ""

# Create the project
cd "$TEMP_DIR"
cargo verus new --lib allocator_full_verify > /dev/null 2>&1
cd allocator_full_verify

# Update Cargo.toml
echo "Configuring Cargo.toml..."
cat > Cargo.toml << 'EOF'
[package]
name = "allocator_full_verify"
version = "0.1.0"
edition = "2021"

[dependencies]
vstd = "=0.0.0-2025-12-28-0056"
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
libc = "0.2"

[package.metadata.verus]
verify = true
EOF

# Create lib.rs with all module declarations
echo "Creating lib.rs with module declarations..."
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

echo "Copying and processing module files..."
echo ""

# Copy each module file, removing the outer mod wrapper
MODULE_COUNT=0
for file in "$SCRIPT_DIR/allocator_inline_blocks"/*.rs; do
    if [ -f "$file" ]; then
        basename=$(basename "$file")
        MODULE_COUNT=$((MODULE_COUNT + 1))

        echo "  Processing: $basename"

        # Copy the file directly (already has correct structure)
        cp "$file" "src/${basename}"
    fi
done

echo ""
echo "✓ Processed $MODULE_COUNT module files"
echo ""
echo "========================================"
echo "Running Verification"
echo "========================================"
echo "This may take 10-30 minutes for full verification..."
echo ""

# Run verification and capture output
if cargo verus verify 2>&1 | tee "$SCRIPT_DIR/verification_full_output.txt"; then
    RESULT=$?
else
    RESULT=$?
fi

echo ""
echo "========================================"
echo "Verification Complete"
echo "========================================"
echo ""

# Check results
if grep -q "0 errors" "$SCRIPT_DIR/verification_full_output.txt"; then
    echo "✅ SUCCESS: Verification completed with no errors!"
    grep "verification results::" "$SCRIPT_DIR/verification_full_output.txt"
else
    echo "⚠️  Verification completed with some errors"
    echo ""
    echo "Summary of errors:"
    grep -E "error\[E[0-9]+\]|error:" "$SCRIPT_DIR/verification_full_output.txt" | head -20
    echo ""
    echo "See full output in: verification_full_output.txt"
fi

echo ""
echo "Output saved to: verification_full_output.txt"
echo "Temporary project at: $PROJECT_DIR"
echo ""

# Ask if user wants to keep the project
read -p "Keep the temporary project for inspection? [y/N] " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cleaning up temporary project..."
    rm -rf "$TEMP_DIR"
    echo "✓ Cleanup complete"
else
    echo "Project preserved at: $PROJECT_DIR"
    echo "To verify again: cd $PROJECT_DIR && cargo verus verify"
fi

echo ""
echo "Done!"
