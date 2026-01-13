#!/usr/bin/env python3
"""
Verify that all .rs files in verified_sources are standalone verifiable with Verus.
"""

import subprocess
import sys
from pathlib import Path
from concurrent.futures import ProcessPoolExecutor, as_completed
from typing import Tuple, List
import json

VERUS_PATH = "verus"
SOURCES_DIR = Path("/Users/chuyues/VerusFT_RL/workspace/min_dataset/verified_sources")
FAILED_FILES_PATH = SOURCES_DIR / "FAILED_FILES.txt"

def parse_known_failures() -> set:
    """Parse FAILED_FILES.txt to get known failing files."""
    known_failures = set()
    if FAILED_FILES_PATH.exists():
        with open(FAILED_FILES_PATH, 'r') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#') and line.endswith('.rs'):
                    known_failures.add(line)
    return known_failures

def verify_file(file_path: Path) -> Tuple[str, bool, str]:
    """
    Verify a single Rust file with Verus.
    Returns (filename, success, error_message)
    """
    filename = file_path.name
    try:
        result = subprocess.run(
            [VERUS_PATH, "--crate-type=lib", str(file_path)],
            capture_output=True,
            text=True,
            timeout=60  # 60 second timeout per file
        )

        success = result.returncode == 0
        error_msg = ""

        if not success:
            # Extract relevant error information
            stderr = result.stderr.strip()
            stdout = result.stdout.strip()
            error_msg = stderr if stderr else stdout
            # Limit error message length
            if len(error_msg) > 500:
                error_msg = error_msg[:500] + "..."

        return (filename, success, error_msg)

    except subprocess.TimeoutExpired:
        return (filename, False, "TIMEOUT: Verification took longer than 60 seconds")
    except Exception as e:
        return (filename, False, f"ERROR: {str(e)}")

def main():
    print("=" * 80)
    print("VERIFYING ALL FILES IN verified_sources")
    print("=" * 80)

    # Get all .rs files
    rs_files = sorted(SOURCES_DIR.glob("*.rs"))
    total_files = len(rs_files)

    print(f"\nTotal .rs files found: {total_files}")

    # Parse known failures
    known_failures = parse_known_failures()
    print(f"Known failures documented: {len(known_failures)}")
    print()

    # Verify all files in parallel
    passed = []
    failed = []
    new_failures = []  # Failures not in FAILED_FILES.txt
    unexpected_passes = []  # Known failures that now pass

    print("Verifying files (this may take a while)...")
    print("-" * 80)

    with ProcessPoolExecutor(max_workers=8) as executor:
        futures = {executor.submit(verify_file, f): f for f in rs_files}

        completed = 0
        for future in as_completed(futures):
            filename, success, error_msg = future.result()
            completed += 1

            if completed % 100 == 0:
                print(f"Progress: {completed}/{total_files} files verified")

            if success:
                passed.append(filename)
                if filename in known_failures:
                    unexpected_passes.append(filename)
            else:
                failed.append((filename, error_msg))
                if filename not in known_failures:
                    new_failures.append((filename, error_msg))

    print()
    print("=" * 80)
    print("VERIFICATION RESULTS")
    print("=" * 80)
    print(f"\nTotal files: {total_files}")
    print(f"Passed: {len(passed)} ({len(passed)/total_files*100:.1f}%)")
    print(f"Failed: {len(failed)} ({len(failed)/total_files*100:.1f}%)")

    if new_failures:
        print(f"\n⚠️  NEW FAILURES (not in FAILED_FILES.txt): {len(new_failures)}")
        print("-" * 80)
        for filename, error in new_failures[:10]:  # Show first 10
            print(f"\n  {filename}")
            print(f"    Error: {error[:200]}")
        if len(new_failures) > 10:
            print(f"\n  ... and {len(new_failures) - 10} more")

    if unexpected_passes:
        print(f"\n✓ UNEXPECTED PASSES (were in FAILED_FILES.txt): {len(unexpected_passes)}")
        print("-" * 80)
        for filename in unexpected_passes:
            print(f"  {filename}")

    # Summary
    print("\n" + "=" * 80)
    if new_failures:
        print("❌ VERIFICATION FAILED: Found new failing files!")
        print(f"   {len(new_failures)} files failed that are not documented in FAILED_FILES.txt")
        return 1
    elif len(failed) == len(known_failures) and not unexpected_passes:
        print("✓ ALL FILES VERIFIED SUCCESSFULLY!")
        print("  All failures match FAILED_FILES.txt exactly")
        return 0
    elif unexpected_passes:
        print("✓ IMPROVED: Some previously failing files now pass!")
        print(f"  {len(unexpected_passes)} files that were failing now pass")
        print("  Consider updating FAILED_FILES.txt")
        return 0
    else:
        print("✓ ALL FILES VERIFIED SUCCESSFULLY!")
        return 0

if __name__ == "__main__":
    sys.exit(main())
