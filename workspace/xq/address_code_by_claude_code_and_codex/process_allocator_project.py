#!/usr/bin/env python3
"""
Script to process verified-memory-allocator project:
1. Use Claude API to split the project into smaller self-contained Rust files
2. Use inline-crate to eliminate package dependencies
3. Use line_count tool to annotate code types (exec/spec/proof)
4. Save results to verus/tests directory
"""

import os
import sys
import json
import subprocess
import shutil
from pathlib import Path
import anthropic
import argparse
import tempfile
import re


# Configuration
SOURCE_PROJECT = "/home/chuyue/verified-memory-allocator"
INLINE_CRATE_DIR = "/home/chuyue/inline-crate"
VERUS_DIR = "/home/chuyue/verus"
LINE_COUNT_TOOL = f"{VERUS_DIR}/source/target/debug/line_count"
OUTPUT_DIR = f"{VERUS_DIR}/tests/split_allocator"
TARGET_LINES_PER_FILE = 400  # Target lines per split file


class AllocatorProcessor:
    def __init__(self, api_key: str, verbose: bool = False):
        self.api_key = api_key
        self.client = anthropic.Anthropic(api_key=api_key)
        self.verbose = verbose
        self.temp_dir = None

    def log(self, message: str):
        """Print log message if verbose mode is enabled."""
        if self.verbose:
            print(f"[INFO] {message}")

    def setup_output_dir(self):
        """Create output directory if it doesn't exist."""
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        self.log(f"Output directory: {OUTPUT_DIR}")

    def build_line_count_tool(self):
        """Build the line_count tool if not already built."""
        self.log("Building line_count tool...")
        line_count_dir = f"{VERUS_DIR}/source/tools/line_count"

        if not os.path.exists(LINE_COUNT_TOOL):
            subprocess.run(
                ["cargo", "build"],
                cwd=line_count_dir,
                check=True,
                capture_output=not self.verbose
            )
            self.log("line_count tool built successfully")
        else:
            self.log("line_count tool already exists")

    def check_inline_crate(self):
        """Check if inline-crate is available."""
        try:
            result = subprocess.run(
                ["cargo", "run", "--", "--version"],
                cwd=INLINE_CRATE_DIR,
                capture_output=True,
                text=True
            )
            self.log(f"inline-crate available: {result.stdout.strip()}")
            return True
        except Exception as e:
            self.log(f"Warning: inline-crate check failed: {e}")
            return False

    def read_project_files(self):
        """Read the structure and content of the allocator project."""
        verus_mimalloc_dir = f"{SOURCE_PROJECT}/verus-mimalloc"
        files_info = []

        for file_path in Path(verus_mimalloc_dir).glob("*.rs"):
            with open(file_path, 'r') as f:
                content = f.read()
                line_count = len(content.splitlines())
                files_info.append({
                    'name': file_path.name,
                    'path': str(file_path),
                    'lines': line_count,
                    'content': content[:2000]  # First 2000 chars for context
                })

        return files_info

    def ask_claude_to_split(self, files_info):
        """Use Claude API to plan how to split the project into smaller files."""
        self.log("Asking Claude to plan the file splitting strategy...")

        # Create a summary of the project
        file_summary = "\n".join([
            f"- {f['name']}: {f['lines']} lines"
            for f in files_info
        ])

        prompt = f"""You are helping to split a verified memory allocator project into smaller, self-contained Rust files for testing purposes.

Project location: {SOURCE_PROJECT}/verus-mimalloc/

Current files:
{file_summary}

Task: Create a plan to split this project into smaller self-contained Rust files, where each file is approximately {TARGET_LINES_PER_FILE} lines or less.

For each output file you propose, specify:
1. The output filename (e.g., "allocator_core_01.rs")
2. Which source files or sections to include
3. What the file contains (brief description)
4. Estimated line count

Consider:
- Group related functionality together
- Ensure each file can be somewhat self-contained
- Keep the total number of files manageable (aim for 15-25 files)
- Larger files (like page_organization.rs with 224K lines or linked_list.rs with 89K lines) should be split into multiple parts

Provide your response as a JSON array with this structure:
[
  {{
    "output_file": "filename.rs",
    "source_files": ["file1.rs", "file2.rs"],
    "description": "What this file contains",
    "estimated_lines": 400
  }},
  ...
]

Only respond with the JSON array, no other text."""

        try:
            message = self.client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=8000,
                messages=[{"role": "user", "content": prompt}]
            )

            response_text = message.content[0].text
            self.log(f"Claude response received: {len(response_text)} characters")

            # Extract JSON from response
            json_match = re.search(r'\[.*\]', response_text, re.DOTALL)
            if json_match:
                split_plan = json.loads(json_match.group())
                self.log(f"Split plan created with {len(split_plan)} files")
                return split_plan
            else:
                self.log("Error: Could not extract JSON from Claude's response")
                return None

        except Exception as e:
            self.log(f"Error calling Claude API: {e}")
            return None

    def create_split_files(self, split_plan):
        """Create the split files based on Claude's plan."""
        self.log("Creating split files based on plan...")
        split_files = []

        for item in split_plan:
            output_file = item['output_file']
            source_files = item['source_files']
            description = item['description']

            self.log(f"Creating {output_file}: {description}")

            # Combine source files
            combined_content = f"// {description}\n\n"

            for source_file in source_files:
                source_path = f"{SOURCE_PROJECT}/verus-mimalloc/{source_file}"
                if os.path.exists(source_path):
                    with open(source_path, 'r') as f:
                        combined_content += f"\n// ====== From {source_file} ======\n\n"
                        combined_content += f.read()
                else:
                    self.log(f"Warning: Source file not found: {source_file}")

            # Write to temporary location
            temp_output = f"{OUTPUT_DIR}/temp_{output_file}"
            with open(temp_output, 'w') as f:
                f.write(combined_content)

            split_files.append({
                'temp_file': temp_output,
                'output_file': output_file,
                'description': description
            })

        return split_files

    def inline_dependencies(self, file_info):
        """Use inline-crate to eliminate package dependencies."""
        temp_file = file_info['temp_file']
        output_file = file_info['output_file']

        self.log(f"Inlining dependencies for {output_file}...")

        # Create a temporary crate structure for inline-crate
        with tempfile.TemporaryDirectory() as temp_crate_dir:
            # Create minimal Cargo.toml
            cargo_toml = f"""{temp_crate_dir}/Cargo.toml"""
            with open(cargo_toml, 'w') as f:
                f.write("""[package]
name = "temp_crate"
version = "0.1.0"
edition = "2021"

[dependencies]
""")

            # Copy file as lib.rs
            shutil.copy(temp_file, f"{temp_crate_dir}/src/lib.rs")
            os.makedirs(f"{temp_crate_dir}/src", exist_ok=True)

            # Try to run inline-crate
            try:
                inlined_output = f"{OUTPUT_DIR}/{output_file}"
                result = subprocess.run(
                    ["cargo", "run", "--", temp_crate_dir, inlined_output, "--force"],
                    cwd=INLINE_CRATE_DIR,
                    capture_output=True,
                    text=True,
                    timeout=60
                )

                if result.returncode == 0 and os.path.exists(inlined_output):
                    self.log(f"Successfully inlined: {output_file}")
                    return inlined_output
                else:
                    # If inline-crate fails, just use the original
                    self.log(f"inline-crate failed for {output_file}, using original")
                    shutil.copy(temp_file, f"{OUTPUT_DIR}/{output_file}")
                    return f"{OUTPUT_DIR}/{output_file}"
            except Exception as e:
                self.log(f"Error running inline-crate: {e}")
                # Fallback: just copy the file
                shutil.copy(temp_file, f"{OUTPUT_DIR}/{output_file}")
                return f"{OUTPUT_DIR}/{output_file}"

    def annotate_with_line_count(self, file_path):
        """Use line_count tool to annotate code types."""
        self.log(f"Annotating {os.path.basename(file_path)} with line counts...")

        try:
            # Run verus on the file to generate .d file
            # For simplicity, we'll use --one-file mode
            result = subprocess.run(
                [LINE_COUNT_TOOL, file_path, "--one-file", "--json"],
                capture_output=True,
                text=True,
                timeout=120
            )

            if result.returncode == 0:
                # Parse JSON output
                try:
                    stats = json.loads(result.stdout)
                    self.log(f"Line count stats: {stats}")
                    return stats
                except json.JSONDecodeError:
                    self.log(f"Could not parse line_count JSON output")
                    return None
            else:
                self.log(f"line_count failed: {result.stderr}")
                return None

        except Exception as e:
            self.log(f"Error running line_count: {e}")
            return None

    def create_summary_report(self, results):
        """Create a summary report of all processed files."""
        report_path = f"{OUTPUT_DIR}/SUMMARY.md"

        with open(report_path, 'w') as f:
            f.write("# Split Allocator Files Summary\n\n")
            f.write(f"Source project: {SOURCE_PROJECT}\n")
            f.write(f"Output directory: {OUTPUT_DIR}\n")
            f.write(f"Total files created: {len(results)}\n\n")

            f.write("## Files\n\n")
            for result in results:
                f.write(f"### {result['filename']}\n\n")
                f.write(f"- Description: {result['description']}\n")

                if result.get('line_stats'):
                    stats = result['line_stats']
                    f.write(f"- Line statistics:\n")
                    for key, value in stats.items():
                        f.write(f"  - {key}: {value}\n")

                f.write("\n")

        self.log(f"Summary report created: {report_path}")

    def process(self):
        """Main processing pipeline."""
        print("=== Allocator Project Processor ===\n")

        # Step 1: Setup
        self.setup_output_dir()
        self.build_line_count_tool()
        self.check_inline_crate()

        # Step 2: Read project structure
        print("\n[Step 1] Reading project files...")
        files_info = self.read_project_files()
        print(f"Found {len(files_info)} Rust files in verus-mimalloc")

        # Step 3: Ask Claude to plan the split
        print("\n[Step 2] Consulting Claude API for splitting strategy...")
        split_plan = self.ask_claude_to_split(files_info)

        if not split_plan:
            print("Error: Could not get split plan from Claude")
            return False

        print(f"Split plan created: {len(split_plan)} output files")

        # Step 4: Create split files
        print("\n[Step 3] Creating split files...")
        split_files = self.create_split_files(split_plan)

        # Step 5: Process each file
        print("\n[Step 4] Processing each file...")
        results = []

        for i, file_info in enumerate(split_files, 1):
            print(f"\n  [{i}/{len(split_files)}] Processing {file_info['output_file']}...")

            # Inline dependencies
            inlined_file = self.inline_dependencies(file_info)

            # Annotate with line_count
            line_stats = self.annotate_with_line_count(inlined_file)

            results.append({
                'filename': file_info['output_file'],
                'description': file_info['description'],
                'path': inlined_file,
                'line_stats': line_stats
            })

            # Clean up temp file
            if os.path.exists(file_info['temp_file']):
                os.remove(file_info['temp_file'])

        # Step 6: Create summary
        print("\n[Step 5] Creating summary report...")
        self.create_summary_report(results)

        print(f"\n=== Processing Complete ===")
        print(f"Output directory: {OUTPUT_DIR}")
        print(f"Files created: {len(results)}")

        return True


def main():
    parser = argparse.ArgumentParser(
        description="Process verified-memory-allocator project into smaller test files"
    )
    parser.add_argument(
        "--api-key",
        required=True,
        help="Anthropic API key (or set ANTHROPIC_API_KEY environment variable)"
    )
    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="Enable verbose output"
    )

    args = parser.parse_args()

    # Get API key from argument or environment
    api_key = args.api_key or os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Error: API key required. Use --api-key or set ANTHROPIC_API_KEY")
        return 1

    # Create processor and run
    processor = AllocatorProcessor(api_key, verbose=args.verbose)

    try:
        success = processor.process()
        return 0 if success else 1
    except KeyboardInterrupt:
        print("\n\nInterrupted by user")
        return 1
    except Exception as e:
        print(f"\nError: {e}")
        if args.verbose:
            import traceback
            traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
