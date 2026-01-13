#!/usr/bin/env python3
"""Verify all extracted .rs files with Verus"""

import subprocess
import os
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
import json

VERUS_PATH = "/Users/chuyues/verus/verus"
SOURCE_DIR = Path(__file__).parent / "verified_sources"

def verify_file(rs_file: Path) -> dict:
    """Run Verus on a single file and return result"""
    try:
        result = subprocess.run(
            [VERUS_PATH, "--crate-type=lib", str(rs_file)],
            capture_output=True,
            text=True,
            timeout=60  # 60 second timeout per file
        )
        verified = result.returncode == 0
        return {
            "file": rs_file.name,
            "verified": verified,
            "returncode": result.returncode,
            "error": result.stderr[:500] if not verified else None
        }
    except subprocess.TimeoutExpired:
        return {
            "file": rs_file.name,
            "verified": False,
            "returncode": -1,
            "error": "Timeout (60s)"
        }
    except Exception as e:
        return {
            "file": rs_file.name,
            "verified": False,
            "returncode": -1,
            "error": str(e)
        }

def main():
    rs_files = list(SOURCE_DIR.glob("*.rs"))
    total = len(rs_files)
    print(f"Verifying {total} files with Verus...")
    
    verified_count = 0
    failed_count = 0
    failed_files = []
    
    # Use parallel execution for speed (4 workers)
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = {executor.submit(verify_file, f): f for f in rs_files}
        
        for i, future in enumerate(as_completed(futures), 1):
            result = future.result()
            if result["verified"]:
                verified_count += 1
            else:
                failed_count += 1
                failed_files.append(result)
            
            # Progress update every 100 files
            if i % 100 == 0 or i == total:
                print(f"Progress: {i}/{total} | Verified: {verified_count} | Failed: {failed_count}")
    
    print(f"\n{'='*50}")
    print(f"FINAL RESULTS:")
    print(f"Total files: {total}")
    print(f"Verified: {verified_count} ({100*verified_count/total:.1f}%)")
    print(f"Failed: {failed_count} ({100*failed_count/total:.1f}%)")
    
    if failed_files:
        print(f"\nFailed files ({len(failed_files)}):")
        for f in failed_files[:20]:  # Show first 20 failures
            print(f"  - {f['file']}: {f['error'][:100] if f['error'] else 'Unknown error'}")
        if len(failed_files) > 20:
            print(f"  ... and {len(failed_files) - 20} more")
        
        # Print all failures to stdout instead of file
        print("\nAll failed files:")
        for f in failed_files:
            print(f"  - {f['file']}")

if __name__ == "__main__":
    main()
