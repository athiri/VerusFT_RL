# Quick Start Guide

## Quick Usage

### Method 1: Using Shell Wrapper Script (Recommended)

```bash
cd /home/chuyue/verus/tests

# Pass API key via command line
./run_allocator_processing.sh YOUR_API_KEY

# Or use environment variable
export ANTHROPIC_API_KEY="your-api-key-here"
./run_allocator_processing.sh
```

### Method 2: Using Python Script Directly

```bash
cd /home/chuyue/verus/tests

# Ensure dependencies are installed
pip3 install anthropic

# Run the script
./process_allocator_project.py --api-key YOUR_API_KEY --verbose
```

## Output Location

Processed files will be saved to:
```
/home/chuyue/verus/tests/split_allocator/
```

View the summary report:
```bash
cat /home/chuyue/verus/tests/split_allocator/SUMMARY.md
```

## Workflow

The script will automatically complete the following tasks:

1. ✅ Analyze the `/home/chuyue/verified-memory-allocator` project
2. ✅ Call Claude API to generate splitting strategy
3. ✅ Create smaller self-contained Rust files
4. ✅ Use `inline-crate` to eliminate dependencies
5. ✅ Use `line_count` to annotate code types
6. ✅ Generate summary report

## Estimated Processing Time

- Project analysis: ~30 seconds
- Claude API call: ~10-20 seconds
- File processing: ~5-10 minutes (depends on number of files)

## Detailed Documentation

View complete documentation:
```bash
cat /home/chuyue/verus/tests/ALLOCATOR_PROCESSING_README.md
```

## Get API Key

Visit [Anthropic Console](https://console.anthropic.com/) to get an API key.
