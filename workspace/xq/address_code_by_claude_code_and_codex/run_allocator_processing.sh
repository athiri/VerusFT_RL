#!/bin/bash
# Wrapper script for process_allocator_project.py
# Usage: ./run_allocator_processing.sh [API_KEY]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_SCRIPT="$SCRIPT_DIR/process_allocator_project.py"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Allocator Project Processor ===${NC}\n"

# Check if API key is provided
if [ -z "$1" ]; then
    if [ -z "$ANTHROPIC_API_KEY" ]; then
        echo -e "${RED}Error: API key required${NC}"
        echo "Usage: $0 <API_KEY>"
        echo "   Or: export ANTHROPIC_API_KEY=your-key-here && $0"
        exit 1
    else
        API_KEY="$ANTHROPIC_API_KEY"
        echo -e "${YELLOW}Using API key from ANTHROPIC_API_KEY environment variable${NC}\n"
    fi
else
    API_KEY="$1"
    echo -e "${YELLOW}Using API key from command line argument${NC}\n"
fi

# Check if Python script exists
if [ ! -f "$PYTHON_SCRIPT" ]; then
    echo -e "${RED}Error: Python script not found at $PYTHON_SCRIPT${NC}"
    exit 1
fi

# Check if anthropic package is installed
if ! python3 -c "import anthropic" 2>/dev/null; then
    echo -e "${YELLOW}Installing anthropic package...${NC}"
    pip3 install anthropic
fi

# Run the Python script with verbose output
echo -e "${GREEN}Starting processing...${NC}\n"
python3 "$PYTHON_SCRIPT" --api-key "$API_KEY" --verbose

# Check exit code
if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}=== Processing completed successfully! ===${NC}"
    echo -e "Output directory: ${GREEN}/home/chuyue/verus/tests/split_allocator${NC}"
    echo -e "Summary report: ${GREEN}/home/chuyue/verus/tests/split_allocator/SUMMARY.md${NC}"
else
    echo -e "\n${RED}=== Processing failed ===${NC}"
    exit 1
fi
