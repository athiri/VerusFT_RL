#!/usr/bin/env python3
"""
Improved minimizer script that uses Verus-provided tools and validates results.

This script:
1. Uses the interestingness_test.sh for creduce validation
2. Validates minimized files after minimization
3. Provides better error reporting
4. Can reproduce dataset without additional tools

Usage: python3 run_minimizer_improved.py [batch_size] [--validate-only]
"""
import subprocess
import shutil
import os
from pathlib import Path
import sys
import time
from typing import Optional, Tuple
from concurrent.futures import ProcessPoolExecutor, as_completed

# Configuration - adjust paths as needed
# Use local minimizer directory in this repo
SCRIPT_DIR = Path(__file__).parent
MINIMIZER_DIR = SCRIPT_DIR / "tools/minimizers"
EXAMPLES_DIR = SCRIPT_DIR / "extracted_examples"
OUTPUT_DIR = SCRIPT_DIR / "minimized_examples"

# Try to find Verus executable
# 1. Check environment variable
VERUS_PATH = os.environ.get("VERUS_PATH")
if VERUS_PATH:
    VERUS_PATH = Path(VERUS_PATH)
else:
    # 2. Try relative path (if Verus is checked out next to VerusFT_RL)
    verus_root = SCRIPT_DIR.parent / "verus"
    verus_path_candidate = verus_root / "source/target-verus/release/verus"
    if verus_path_candidate.exists():
        VERUS_PATH = verus_path_candidate
    else:
        # 3. Try absolute path (fallback)
        VERUS_PATH = Path("/Users/athiri/Desktop/verus/source/target-verus/release/verus")

# Get batch size from command line (default 10)
BATCH_SIZE = int(sys.argv[1]) if len(sys.argv) > 1 and sys.argv[1].isdigit() else 10
VALIDATE_ONLY = "--validate-only" in sys.argv


def validate_minimized_file(filepath: Path) -> Tuple[bool, Optional[str]]:
    """
    Validate a minimized file using the same logic as interestingness_test.sh
    
    Returns: (is_valid, error_message)
    """
    if not filepath.exists():
        return False, "File does not exist"
    
    try:
        content = filepath.read_text()
    except Exception as e:
        return False, f"Could not read file: {e}"
    
    # Check for verus! block
    if 'verus!' not in content:
        return False, "No verus! block found"
    
    # Check for Verus keywords
    verus_keywords = ['spec', 'proof', 'requires', 'ensures', 'decreases', 
                      'assert', 'invariant', '&&&', '|||']
    if not any(keyword in content for keyword in verus_keywords):
        return False, "No Verus keywords found"
    
    # Check for function bodies
    if '{' not in content:
        return False, "No function bodies found"
    
    # Run actual verification
    try:
        result = subprocess.run(
            [str(VERUS_PATH), '--crate-type=lib', str(filepath)],
            capture_output=True,
            text=True,
            timeout=30  # Reduced from 60s
        )
        
        output = result.stdout + result.stderr
        
        # Check for resource limit errors
        if 'resource limit' in output.lower() or 'rlimit' in output.lower():
            return False, "Resource limit exceeded"
        
        # Check for error messages
        if 'error:' in output.lower():
            return False, "Verification errors found"
        
        # Check for verification results format
        import re
        pattern = r'verification results::\s*(\d+)\s+verified,\s*(\d+)\s+errors'
        match = re.search(pattern, output, re.IGNORECASE)
        
        if not match:
            return False, "No verification results found"
        
        verified = int(match.group(1))
        errors = int(match.group(2))
        
        if verified == 0:
            return False, f"No verified items (errors: {errors})"
        
        if errors > 0:
            return False, f"Verification errors: {errors}"
        
        if result.returncode != 0:
            return False, f"Non-zero exit code: {result.returncode}"
        
        return True, None
        
    except subprocess.TimeoutExpired:
        return False, "Verification timed out"
    except Exception as e:
        return False, f"Verification exception: {e}"


def minimize_file(example_file: Path, minimizer_dir: Path, output_dir: Path) -> Tuple[bool, str, dict]:
    """
    Minimize a single file using creduce.
    
    Returns: (success, message, stats_dict)
    """
    foo_path = minimizer_dir / "foo.rs"
    
    # Copy file to minimizer directory
    shutil.copy(example_file, foo_path)
    
    original_size = foo_path.stat().st_size
    original_lines = len(foo_path.read_text().split('\n'))
    
    stats = {
        'original_size': original_size,
        'original_lines': original_lines,
        'minimized_size': None,
        'minimized_lines': None,
        'reduction_percent': None
    }
    
    try:
        # Set VERUS_PATH environment variable for interestingness_test.sh
        env = os.environ.copy()
        env["VERUS_PATH"] = str(VERUS_PATH)
        
        # Run creduce
        result = subprocess.run(
            ["creduce", "--n", "4", "./interestingness_test.sh", "./foo.rs"],
            cwd=minimizer_dir,
            capture_output=True,
            text=True,
            timeout=300,
            env=env
        )
        
        minimized_size = foo_path.stat().st_size
        minimized_lines = len(foo_path.read_text().split('\n'))
        reduction = 100 * (1 - minimized_size / original_size) if original_size > 0 else 0
        
        stats['minimized_size'] = minimized_size
        stats['minimized_lines'] = minimized_lines
        stats['reduction_percent'] = reduction
        
        # Validate minimized file
        is_valid, error_msg = validate_minimized_file(foo_path)
        
        if not is_valid:
            return False, f"Minimized file failed validation: {error_msg}", stats
        
        # Copy to output directory
        output_file = output_dir / example_file.name
        shutil.copy(foo_path, output_file)
        
        # Validate output file too
        is_valid_output, error_msg_output = validate_minimized_file(output_file)
        if not is_valid_output:
            return False, f"Output file failed validation: {error_msg_output}", stats
        
        return True, f"Minimized: {minimized_size} bytes, {minimized_lines} lines ({reduction:.1f}% reduction)", stats
        
    except subprocess.TimeoutExpired:
        return False, "Minimization timed out", stats
    except Exception as e:
        return False, f"Minimization error: {e}", stats
    finally:
        # Clean up foo.rs if it exists
        if foo_path.exists():
            pass  # Keep it for inspection if needed


def _validate_single(filepath: Path) -> Tuple[Path, bool, Optional[str]]:
    """Helper for parallel validation."""
    is_valid, error_msg = validate_minimized_file(filepath)
    return filepath, is_valid, error_msg


def validate_existing_files(output_dir: Path, parallel: bool = True) -> dict:
    """
    Validate all existing minimized files.
    
    Returns: dict with validation results
    """
    files = list(output_dir.glob("*.rs"))
    results = {
        'total': len(files),
        'valid': 0,
        'invalid': 0,
        'errors': []
    }
    
    print(f"\nValidating {len(files)} existing minimized files...")
    
    if parallel and len(files) > 10:
        # Use parallel validation for speed
        with ProcessPoolExecutor(max_workers=8) as executor:
            futures = {executor.submit(_validate_single, f): f for f in files}
            done = 0
            for future in as_completed(futures):
                filepath, is_valid, error_msg = future.result()
                done += 1
                if done % 100 == 0:
                    print(f"  Progress: {done}/{len(files)}")
                if is_valid:
                    results['valid'] += 1
                else:
                    results['invalid'] += 1
                    results['errors'].append({
                        'file': filepath.name,
                        'error': error_msg
                    })
    else:
        for filepath in files:
            is_valid, error_msg = validate_minimized_file(filepath)
            if is_valid:
                results['valid'] += 1
            else:
                results['invalid'] += 1
                results['errors'].append({
                    'file': filepath.name,
                    'error': error_msg
                })
                print(f"  ❌ {filepath.name}: {error_msg}")
    
    print(f"\nValidation Results:")
    print(f"  Valid: {results['valid']}/{results['total']}")
    print(f"  Invalid: {results['invalid']}/{results['total']}")
    
    return results


def main():
    """Main entry point"""
    # Create minimizer directory if it doesn't exist
    MINIMIZER_DIR.mkdir(parents=True, exist_ok=True)
    
    # Check if interestingness_test.sh exists
    interestingness_test = MINIMIZER_DIR / "interestingness_test.sh"
    if not interestingness_test.exists():
        print(f"Error: interestingness_test.sh not found: {interestingness_test}")
        print("Please ensure tools/minimizers/interestingness_test.sh exists in this repo")
        sys.exit(1)
    
    # Check if Verus executable exists
    if not VERUS_PATH.exists():
        print(f"Warning: Verus executable not found: {VERUS_PATH}")
        print("Please set VERUS_PATH environment variable or ensure Verus is built")
        print("Example: export VERUS_PATH=/path/to/verus/source/target-verus/release/verus")
        if not VALIDATE_ONLY:
            print("Minimization will fail without Verus executable")
            sys.exit(1)
    
    OUTPUT_DIR.mkdir(exist_ok=True)
    
    if VALIDATE_ONLY:
        validate_existing_files(OUTPUT_DIR)
        return
    
    if not EXAMPLES_DIR.exists():
        print(f"Error: Examples directory not found: {EXAMPLES_DIR}")
        sys.exit(1)
    
    all_examples = sorted(list(EXAMPLES_DIR.glob("example_*.rs")))
    examples_to_minimize = [ex for ex in all_examples if not (OUTPUT_DIR / ex.name).exists()]
    examples = examples_to_minimize[:BATCH_SIZE]
    
    total_done = len(all_examples) - len(examples_to_minimize)
    
    print("="*70)
    print("Improved Minimizer")
    print("="*70)
    print(f"Total examples: {len(all_examples)}")
    print(f"Already done: {total_done}")
    print(f"Remaining: {len(examples_to_minimize)}")
    print(f"Processing now: {len(examples)}")
    print("="*70)
    print()
    
    if not examples:
        print("✅ All examples minimized!")
        # Validate all files
        validate_existing_files(OUTPUT_DIR)
        return
    
    start_time = time.time()
    successful = 0
    failed = 0
    
    for i, example_file in enumerate(examples, 1):
        print(f"[{total_done + i}/{len(all_examples)}] {example_file.name}")
        
        success, message, stats = minimize_file(example_file, MINIMIZER_DIR, OUTPUT_DIR)
        
        if success:
            print(f"  ✅ {message}")
            successful += 1
        else:
            print(f"  ❌ {message}")
            failed += 1
            # DON'T copy invalid files - only keep validated ones
        
        print()
    
    elapsed = time.time() - start_time
    remaining = len(examples_to_minimize) - len(examples)
    
    print("="*70)
    print("Batch Complete!")
    print("="*70)
    print(f"Processed: {len(examples)} examples in {elapsed/60:.1f} minutes")
    print(f"  Successful: {successful}")
    print(f"  Failed: {failed}")
    print(f"Remaining: {remaining}")
    print(f"Total done: {total_done + len(examples)}/{len(all_examples)}")
    print(f"Output: {OUTPUT_DIR}/")
    print("="*70)
    
    if remaining > 0:
        avg_time = elapsed / len(examples) if examples else 0
        est_time = (avg_time * remaining) / 60
        print(f"\nEstimated time to finish: {est_time:.1f} minutes")
        print(f"Run again to process next batch")
    
    # Final validation
    if successful > 0:
        print("\nRunning final validation...")
        validate_existing_files(OUTPUT_DIR)


if __name__ == "__main__":
    main()

