#!/bin/bash
# Script to check if all requirements are met

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Environment Setup Check ===${NC}\n"

# Check 1: Python 3
echo -n "Checking Python 3... "
if command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version)
    echo -e "${GREEN}✓ $PYTHON_VERSION${NC}"
else
    echo -e "${RED}✗ Python 3 not found${NC}"
    exit 1
fi

# Check 2: pip3
echo -n "Checking pip3... "
if command -v pip3 &> /dev/null; then
    echo -e "${GREEN}✓ Found${NC}"
else
    echo -e "${RED}✗ pip3 not found${NC}"
    exit 1
fi

# Check 3: anthropic package
echo -n "Checking anthropic package... "
if python3 -c "import anthropic" 2>/dev/null; then
    echo -e "${GREEN}✓ Installed${NC}"
else
    echo -e "${YELLOW}✗ Not installed (will be installed automatically)${NC}"
fi

# Check 4: Cargo
echo -n "Checking Cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✓ $CARGO_VERSION${NC}"
else
    echo -e "${RED}✗ Cargo not found${NC}"
    exit 1
fi

# Check 5: Source project
echo -n "Checking source project... "
if [ -d "/home/chuyue/verified-memory-allocator/verus-mimalloc" ]; then
    FILE_COUNT=$(find /home/chuyue/verified-memory-allocator/verus-mimalloc -name "*.rs" | wc -l)
    echo -e "${GREEN}✓ Found ($FILE_COUNT Rust files)${NC}"
else
    echo -e "${RED}✗ Not found at /home/chuyue/verified-memory-allocator/verus-mimalloc${NC}"
    exit 1
fi

# Check 6: inline-crate directory
echo -n "Checking inline-crate... "
if [ -d "/home/chuyue/inline-crate" ]; then
    echo -e "${GREEN}✓ Found${NC}"
else
    echo -e "${RED}✗ Not found at /home/chuyue/inline-crate${NC}"
    exit 1
fi

# Check 7: line_count tool
echo -n "Checking line_count tool... "
if [ -f "/home/chuyue/verus/source/target/debug/line_count" ]; then
    echo -e "${GREEN}✓ Built${NC}"
else
    echo -e "${YELLOW}✗ Not built (will be built automatically)${NC}"
fi

# Check 8: Verus directory
echo -n "Checking Verus directory... "
if [ -d "/home/chuyue/verus" ]; then
    echo -e "${GREEN}✓ Found${NC}"
else
    echo -e "${RED}✗ Not found at /home/chuyue/verus${NC}"
    exit 1
fi

# Check 9: Processing scripts
echo -n "Checking processing scripts... "
if [ -f "/home/chuyue/verus/tests/process_allocator_project.py" ] && [ -f "/home/chuyue/verus/tests/run_allocator_processing.sh" ]; then
    echo -e "${GREEN}✓ Found${NC}"
else
    echo -e "${RED}✗ Scripts not found${NC}"
    exit 1
fi

# Check 10: Output directory permissions
echo -n "Checking output directory... "
OUTPUT_DIR="/home/chuyue/verus/tests/split_allocator"
if [ -d "$OUTPUT_DIR" ]; then
    if [ -w "$OUTPUT_DIR" ]; then
        echo -e "${GREEN}✓ Exists and writable${NC}"
    else
        echo -e "${YELLOW}✗ Exists but not writable${NC}"
        echo "  Run: chmod 755 $OUTPUT_DIR"
    fi
else
    if mkdir -p "$OUTPUT_DIR" 2>/dev/null; then
        echo -e "${GREEN}✓ Created successfully${NC}"
    else
        echo -e "${RED}✗ Cannot create directory${NC}"
        exit 1
    fi
fi

echo -e "\n${GREEN}=== Setup Check Complete ===${NC}\n"

# Summary
echo "Summary:"
echo "  - All required tools are available"
echo "  - Source project found"
echo "  - Output directory ready"
echo ""
echo "Next steps:"
echo "  1. Get your Anthropic API key from https://console.anthropic.com/"
echo "  2. Run: ./run_allocator_processing.sh YOUR_API_KEY"
echo ""
echo "For more information, see:"
echo "  - Quick start: cat QUICKSTART.md"
echo "  - Full documentation: cat ALLOCATOR_PROCESSING_README.md"
