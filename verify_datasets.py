#!/usr/bin/env python3
"""
Verify that all .rs files in /datasets are standalone verifiable with Verus.
"""

import subprocess
import sys
from pathlib import Path
from concurrent.futures import ProcessPoolExecutor, as_completed
from typing import Tuple, List
import json
from collections import defaultdict

VERUS_PATH = "verus"
DATASETS_DIR = Path("/Users/chuyues/VerusFT_RL/datasets")

def compute_file_hash(file_path: Path) -> str:
    """Compute SHA256 hash of file content."""
    import hashlib
    sha256_hash = hashlib.sha256()
    with open(file_path, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def verify_file(file_path: Path) -> Tuple[str, bool, str]:
    """
    Verify a single Rust file with Verus.
    Returns (relative_path, success, error_message)
    """
    rel_path = file_path.relative_to(DATASETS_DIR)
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

        return (str(rel_path), success, error_msg)

    except subprocess.TimeoutExpired:
        return (str(rel_path), False, "TIMEOUT: Verification took longer than 60 seconds")
    except Exception as e:
        return (str(rel_path), False, f"ERROR: {str(e)}")

def main():
    print("=" * 80)
    print("VERIFYING ALL FILES IN /datasets")
    print("=" * 80)

    # Get all .rs files
    rs_files = sorted(DATASETS_DIR.rglob("*.rs"))
    total_files = len(rs_files)

    print(f"\nTotal .rs files found: {total_files}")

    # Count by subdirectory
    by_subdir = defaultdict(int)
    for f in rs_files:
        rel_path = f.relative_to(DATASETS_DIR)
        subdir = str(rel_path.parts[0]) if rel_path.parts else "root"
        by_subdir[subdir] += 1

    print("\nFiles by subdirectory:")
    for subdir, count in sorted(by_subdir.items()):
        print(f"  {subdir}: {count} files")
    print()

    # Verify all files in parallel
    passed = []
    failed = []
    by_category_passed = defaultdict(int)
    by_category_failed = defaultdict(int)

    print("Verifying files (this may take a while)...")
    print("-" * 80)

    with ProcessPoolExecutor(max_workers=8) as executor:
        futures = {executor.submit(verify_file, f): f for f in rs_files}

        completed = 0
        for future in as_completed(futures):
            rel_path, success, error_msg = future.result()
            completed += 1

            if completed % 100 == 0:
                print(f"Progress: {completed}/{total_files} files verified")

            # Get category from path
            category = rel_path.split('/')[0] if '/' in rel_path else 'root'

            if success:
                passed.append(rel_path)
                by_category_passed[category] += 1
            else:
                failed.append((rel_path, error_msg))
                by_category_failed[category] += 1

    print()
    print("=" * 80)
    print("VERIFICATION RESULTS")
    print("=" * 80)
    print(f"\nTotal files: {total_files}")
    print(f"Passed: {len(passed)} ({len(passed)/total_files*100:.1f}%)")
    print(f"Failed: {len(failed)} ({len(failed)/total_files*100:.1f}%)")

    # Show results by category
    print("\n" + "-" * 80)
    print("RESULTS BY CATEGORY")
    print("-" * 80)
    all_categories = sorted(set(list(by_category_passed.keys()) + list(by_category_failed.keys())))
    for category in all_categories:
        total_cat = by_category_passed[category] + by_category_failed[category]
        passed_cat = by_category_passed[category]
        failed_cat = by_category_failed[category]
        print(f"{category}:")
        print(f"  Total: {total_cat}, Passed: {passed_cat}, Failed: {failed_cat}")

    if failed:
        print(f"\n" + "-" * 80)
        print(f"FAILED FILES (first 20 of {len(failed)}):")
        print("-" * 80)
        for rel_path, error in failed[:20]:
            print(f"\n  {rel_path}")
            # Show first line of error
            error_lines = error.split('\n')
            if error_lines:
                print(f"    {error_lines[0][:150]}")

        if len(failed) > 20:
            print(f"\n  ... and {len(failed) - 20} more")

    # Save results to JSON
    results = {
        "total_files": total_files,
        "passed": len(passed),
        "failed": len(failed),
        "pass_rate": len(passed)/total_files*100,
        "by_category": {
            cat: {
                "passed": by_category_passed[cat],
                "failed": by_category_failed[cat],
                "total": by_category_passed[cat] + by_category_failed[cat]
            }
            for cat in all_categories
        },
        "failed_files": [{"file": f, "error": e} for f, e in failed]
    }

    results_file = DATASETS_DIR / "verification_results.json"
    with open(results_file, 'w') as f:
        json.dump(results, f, indent=2)
    print(f"\n✓ Results saved to {results_file}")

    # Summary
    print("\n" + "=" * 80)
    if len(failed) == 0:
        print("✓ ALL FILES VERIFIED SUCCESSFULLY!")
        print("  All files in /datasets are standalone verifiable")
        return 0
    else:
        print("❌ VERIFICATION FAILED")
        print(f"  {len(failed)} files failed verification")
        return 1

if __name__ == "__main__":
    sys.exit(main())
