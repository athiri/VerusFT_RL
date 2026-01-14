#!/bin/bash
# Quick verification script for Verus allocator

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "================================================"
echo "Verus Allocator Verification Script"
echo "================================================"
echo ""

# Check if cargo verus is available
if ! command -v cargo-verus &> /dev/null; then
    echo "Error: cargo verus not found!"
    echo "Please install Verus from: https://github.com/verus-lang/verus"
    exit 1
fi

echo "✓ Verus installation found"
echo ""

# Function to verify the test project
verify_test_project() {
    echo "Option 1: Verifying using test project (allocator_test/)"
    echo "Note: This has known module visibility issues with PageOrg"
    echo ""

    if [ ! -d "allocator_test" ]; then
        echo "Error: allocator_test directory not found"
        return 1
    fi

    cd allocator_test
    echo "Running: cargo verus verify"
    echo "This may take several minutes..."
    echo ""

    cargo verus verify 2>&1 | tee verification_output.txt

    # Check if verification succeeded
    if grep -q "0 errors" verification_output.txt; then
        echo ""
        echo "✓ Verification completed with no errors!"
        return 0
    else
        echo ""
        echo "✗ Verification completed with errors"
        echo "See verification_output.txt for details"
        return 1
    fi
}

# Function to verify individual modules
verify_individual_modules() {
    echo "Option 2: Verifying individual modules"
    echo "This will test each module file separately"
    echo ""

    if [ ! -d "allocator_inline_blocks" ]; then
        echo "Error: allocator_inline_blocks directory not found"
        return 1
    fi

    # Create a temporary project for testing
    TEMP_DIR=$(mktemp -d)
    echo "Creating temporary project in: $TEMP_DIR"

    cd "$TEMP_DIR"
    cargo verus new --lib test_module
    cd test_module

    # Update Cargo.toml
    cat > Cargo.toml << 'EOF'
[package]
name = "test_module"
version = "0.1.0"
edition = "2021"

[dependencies]
vstd = "=0.0.0-2025-12-28-0056"
state_machines_macros = { package = "verus_state_machines_macros", version = "=0.0.0-2025-11-23-0053" }
libc = "0.2"

[package.metadata.verus]
verify = true
EOF

    # List of simpler modules to test
    SIMPLE_MODULES=("config" "flags" "thread" "dealloc_token" "arena")

    echo ""
    echo "Testing simple modules: ${SIMPLE_MODULES[*]}"
    echo ""

    SUCCESS_COUNT=0
    FAIL_COUNT=0

    for module in "${SIMPLE_MODULES[@]}"; do
        echo "----------------------------------------"
        echo "Testing: $module.rs"
        echo "----------------------------------------"

        # Copy the module file
        cp "$SCRIPT_DIR/allocator_inline_blocks/${module}.rs" src/lib.rs

        # Try to verify
        if cargo verus verify 2>&1 | grep -q "0 errors"; then
            echo "✓ $module.rs verified successfully"
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
            echo "✗ $module.rs verification failed"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
        echo ""
    done

    # Cleanup
    cd "$SCRIPT_DIR"
    rm -rf "$TEMP_DIR"

    echo "========================================"
    echo "Summary:"
    echo "  Successful: $SUCCESS_COUNT"
    echo "  Failed: $FAIL_COUNT"
    echo "========================================"

    return $FAIL_COUNT
}

# Main menu
show_menu() {
    echo "Choose verification method:"
    echo "  1) Test project (quick but has known issues)"
    echo "  2) Individual modules (slower but more reliable)"
    echo "  3) Show verification status of fixes"
    echo "  4) Exit"
    echo ""
    read -p "Enter choice [1-4]: " choice

    case $choice in
        1)
            verify_test_project
            ;;
        2)
            verify_individual_modules
            ;;
        3)
            show_fix_status
            ;;
        4)
            echo "Exiting..."
            exit 0
            ;;
        *)
            echo "Invalid choice"
            return 1
            ;;
    esac
}

# Show what was fixed
show_fix_status() {
    echo ""
    echo "========================================"
    echo "Modernization Fixes Applied"
    echo "========================================"
    echo ""
    echo "✓ Updated state machine macro imports"
    echo "  Changed: use verus_state_machines_macros::*"
    echo "  To:      use state_machines_macros::*"
    echo ""
    echo "✓ Removed deprecated #[is_variant] attributes"
    echo ""
    echo "✓ Updated enum accessor methods"
    echo "  Changed: .get_VeryUnready_0()"
    echo "  To:      .arrow_VeryUnready_0()"
    echo ""
    echo "✓ Updated enum type checking"
    echo "  Changed: .is_VeryUnready()"
    echo "  To:      matches Popped::VeryUnready(..)"
    echo ""
    echo "✓ Fixed matches expression syntax"
    echo "  Added parentheses for precedence with ==>"
    echo ""
    echo "Files modified:"
    echo "  - 27 individual modules in allocator_inline_blocks/"
    echo "  - allocator_inline_full.rs (combined file)"
    echo ""
    echo "========================================"
    echo ""
}

# Check for command line arguments
if [ $# -eq 0 ]; then
    show_menu
else
    case "$1" in
        --test-project)
            verify_test_project
            ;;
        --individual)
            verify_individual_modules
            ;;
        --status)
            show_fix_status
            ;;
        --help)
            echo "Usage: $0 [OPTION]"
            echo ""
            echo "Options:"
            echo "  --test-project   Verify using the test project"
            echo "  --individual     Verify individual modules"
            echo "  --status         Show what fixes were applied"
            echo "  --help           Show this help message"
            echo ""
            echo "If no option is provided, interactive menu is shown."
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
fi
