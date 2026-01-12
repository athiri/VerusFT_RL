# Allocator Project Processing Script

This script automates the processing of the `verified-memory-allocator` project, splitting it into smaller self-contained Rust files for testing.

## Features

The script performs the following steps:

1. **Call Claude API** - Use Claude to analyze project structure and generate splitting strategy
2. **Split Files** - Split large files into smaller files based on Claude's recommendations (target ~400 lines/file)
3. **Inline Dependencies** - Use the `inline-crate` tool to eliminate package dependencies
4. **Code Annotation** - Use the `line_count` tool to label code types (exec/spec/proof, etc.)
5. **Generate Report** - Create a summary report of processing results

## Prerequisites

### 1. Python Dependencies

Install the Anthropic Python SDK:

```bash
pip install anthropic
```

### 2. Build Required Tools

Ensure the following tools are built:

```bash
# Build the line_count tool
cd /home/chuyue/verus/source/tools/line_count
cargo build

# Ensure inline-crate is available
cd /home/chuyue/inline-crate
cargo build
```

### 3. Anthropic API Key

You need an Anthropic API key. You can obtain one from [Anthropic Console](https://console.anthropic.com/).

## Usage

### Basic Usage

```bash
cd /home/chuyue/verus
./process_allocator_project.py --api-key YOUR_API_KEY
```

### Using Environment Variables

```bash
export ANTHROPIC_API_KEY="your-api-key-here"
./process_allocator_project.py --api-key $ANTHROPIC_API_KEY
```

### Verbose Output Mode

```bash
./process_allocator_project.py --api-key YOUR_API_KEY --verbose
```

### View Help

```bash
./process_allocator_project.py --help
```

## Output

### Output Directory

All processed files will be saved to:
```
/home/chuyue/verus/tests/split_allocator/
```

### Output Files

- **`*.rs`** - Split and processed Rust files
- **`SUMMARY.md`** - Summary report containing statistics for all files

### Summary Report Contents

The report includes:
- Source project information
- List of created files
- Description of each file
- Code statistics (exec/spec/proof line counts, etc.)

## Processing Flow Details

### Step 1: Project Analysis

The script first reads all Rust files in `/home/chuyue/verified-memory-allocator/verus-mimalloc/`:

- Count lines for each file
- Collect file structure information

### Step 2: Claude Planning

Send a request to the Claude API asking it to:
- Analyze the project structure
- Propose a strategy for splitting large files into smaller ones
- Consider functional relevance and self-containment
- Target: approximately 400 lines per file

### Step 3: File Splitting

Based on Claude's plan:
- Merge related source files
- Create new split files
- Preserve original comments and documentation

### Step 4: Dependency Inlining

For each split file:
- Use the `inline-crate` tool
- Eliminate external crate dependencies
- Create self-contained single files

### Step 5: Code Annotation

Using Verus's `line_count` tool:
- Analyze each line of code
- Label as different types:
  - **exec** - Executable code
  - **spec** - Specification code
  - **proof** - Proof code
  - **trusted** - Trusted code
  - **comment** - Comments
  - **layout** - Layout/formatting

### Step 6: Generate Report

Create a Markdown format summary report with statistics.

## Configuration Options

The following configurations can be modified at the beginning of the script:

```python
SOURCE_PROJECT = "/home/chuyue/verified-memory-allocator"
OUTPUT_DIR = f"{VERUS_DIR}/tests/split_allocator"
TARGET_LINES_PER_FILE = 400  # Target lines per file
```

## Troubleshooting

### Issue: Invalid API key

Ensure your API key is a valid Anthropic API key.

### Issue: line_count tool not found

Run:
```bash
cd /home/chuyue/verus/source/tools/line_count
cargo build
```

### Issue: inline-crate fails

If inline-crate fails for certain files, the script will automatically fall back to using the original file.

### Issue: Output directory permissions

Ensure you have write permissions to the output directory:
```bash
mkdir -p /home/chuyue/verus/tests/split_allocator
chmod 755 /home/chuyue/verus/tests/split_allocator
```

## Example Output

```
=== Allocator Project Processor ===

[Step 1] Reading project files...
Found 33 Rust files in verus-mimalloc

[Step 2] Consulting Claude API for splitting strategy...
Split plan created: 18 output files

[Step 3] Creating split files...
Creating allocator_core_01.rs: Core allocation structures and types
Creating allocator_core_02.rs: Page organization and management
...

[Step 4] Processing each file...
  [1/18] Processing allocator_core_01.rs...
  [2/18] Processing allocator_core_02.rs...
  ...

[Step 5] Creating summary report...

=== Processing Complete ===
Output directory: /home/chuyue/verus/tests/split_allocator
Files created: 18
```

## Advanced Usage

### Process Only Specific Files

Modify the `read_project_files()` method in the script to filter files:

```python
def read_project_files(self):
    verus_mimalloc_dir = f"{SOURCE_PROJECT}/verus-mimalloc"
    files_info = []

    # Process only specific files
    target_files = ["page.rs", "segment.rs", "types.rs"]

    for file_name in target_files:
        file_path = Path(verus_mimalloc_dir) / file_name
        if file_path.exists():
            # ... process file
```

### Adjust Split Granularity

Modify `TARGET_LINES_PER_FILE` to change target file size:

```python
TARGET_LINES_PER_FILE = 600  # Larger files
# or
TARGET_LINES_PER_FILE = 200  # Smaller files
```

## Technical Details

### Claude API Usage

- Model: `claude-sonnet-4-20250514`
- Max tokens: 8000
- Output format: JSON

### inline-crate Tool

- Creates temporary crate structure
- Timeout: 60 seconds
- Falls back to original file on failure

### line_count Tool

- Uses `--one-file` mode to process single files
- Outputs JSON format statistics
- Timeout: 120 seconds

## License

This script uses the same license as the Verus project (MIT).

## Support

For questions or suggestions, please refer to:
- Verus documentation: `/home/chuyue/verus/README.md`
- inline-crate documentation: `/home/chuyue/inline-crate/README.md`
