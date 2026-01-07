#!/usr/bin/env python3
"""
Fetch Verus source files from known repositories for dataset generation.

Downloads example/test files from verus-lang/verus and extracts them
to extracted_examples/ for minimization.

Usage:
    python3 fetch_sources.py              # Fetch all sources
    python3 fetch_sources.py --list       # List available sources
    python3 fetch_sources.py --clean      # Remove extracted_examples/
"""

import argparse
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import List, Tuple

# Source repositories and paths to extract
SOURCES = [
    {
        "repo": "https://github.com/verus-lang/verus.git",
        "branch": "main",
        "paths": [
            "source/rust_verify_test/tests",
            "source/rust_verify/example",
            "source/vstd/source/vstd",
        ],
        "description": "Verus core tests, examples, and vstd library",
    },
    {
        "repo": "https://github.com/microsoft/verismo.git",
        "branch": "main",
        "paths": [
            "source/verismo/src",
            "source/verismo_main/src",
        ],
        "description": "Microsoft Verismo - verified firmware",
    },
    {
        "repo": "https://github.com/verus-lang/verified-memory-allocator.git",
        "branch": "main",
        "paths": [
            "verus-mimalloc/src",
        ],
        "description": "Verified memory allocator (mimalloc)",
    },
    {
        "repo": "https://github.com/verus-lang/verified-node-replication.git",
        "branch": "main",
        "paths": [
            "node-replication/src",
            "verified-node-replication/src",
        ],
        "description": "Verified node replication",
    },
    {
        "repo": "https://github.com/Beneficial-AI-Foundation/vericoding.git",
        "branch": "main",
        "paths": [
            "benchmarks",
        ],
        "description": "Vericoding benchmarks",
    },
]

# File patterns to include
INCLUDE_PATTERNS = [
    r"\.rs$",  # Rust files only
]

# Patterns to skip (non-verifying or infrastructure files)
SKIP_PATTERNS = [
    r"mod\.rs$",           # Module files
    r"lib\.rs$",           # Library roots
    r"main\.rs$",          # Binary entry points
    r"_test\.rs$",         # Test harness files
    r"build\.rs$",         # Build scripts
]

# Minimum file size (bytes) - skip tiny stubs
MIN_FILE_SIZE = 100

# Keywords that indicate Verus verification content
VERUS_KEYWORDS = [
    "verus!",
    "requires",
    "ensures", 
    "proof fn",
    "spec fn",
    "invariant",
]


def has_verus_content(content: str) -> bool:
    """Check if file contains Verus verification constructs."""
    return any(kw in content for kw in VERUS_KEYWORDS)


def should_include(filepath: Path) -> bool:
    """Check if file should be included based on patterns."""
    name = filepath.name
    
    # Must match include pattern
    if not any(re.search(p, name) for p in INCLUDE_PATTERNS):
        return False
    
    # Must not match skip pattern
    if any(re.search(p, name) for p in SKIP_PATTERNS):
        return False
    
    return True


def extract_files(repo_dir: Path, extract_paths: List[str], output_dir: Path) -> List[Tuple[str, str]]:
    """
    Extract Verus files from cloned repo to output directory.
    
    Returns list of (source_path, dest_name) tuples.
    """
    extracted = []
    file_counter = 1
    
    for extract_path in extract_paths:
        source_dir = repo_dir / extract_path
        if not source_dir.exists():
            print(f"  Warning: {extract_path} not found, skipping")
            continue
        
        for rs_file in source_dir.rglob("*.rs"):
            if not should_include(rs_file):
                continue
            
            # Check file size
            if rs_file.stat().st_size < MIN_FILE_SIZE:
                continue
            
            # Check for Verus content
            try:
                content = rs_file.read_text()
                if not has_verus_content(content):
                    continue
            except Exception:
                continue
            
            # Generate unique output name
            dest_name = f"example_{file_counter:03d}.rs"
            dest_path = output_dir / dest_name
            
            # Copy file
            shutil.copy2(rs_file, dest_path)
            extracted.append((str(rs_file.relative_to(repo_dir)), dest_name))
            file_counter += 1
    
    return extracted


def fetch_sources(output_dir: Path, verbose: bool = True) -> int:
    """
    Fetch all source files from configured repositories.
    
    Returns total number of files extracted.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    total_extracted = 0
    
    for source in SOURCES:
        repo_url = source["repo"]
        branch = source.get("branch", "main")
        paths = source["paths"]
        desc = source.get("description", repo_url)
        
        if verbose:
            print(f"\nFetching: {desc}")
            print(f"  Repo: {repo_url}")
            print(f"  Branch: {branch}")
        
        # Clone to temp directory (shallow clone for speed)
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            repo_dir = tmp_path / "repo"
            
            if verbose:
                print(f"  Cloning (shallow)...")
            
            result = subprocess.run(
                ["git", "clone", "--depth", "1", "--branch", branch, repo_url, str(repo_dir)],
                capture_output=True,
                text=True,
            )
            
            if result.returncode != 0:
                print(f"  Error cloning: {result.stderr}")
                continue
            
            if verbose:
                print(f"  Extracting Verus files...")
            
            extracted = extract_files(repo_dir, paths, output_dir)
            total_extracted += len(extracted)
            
            if verbose:
                print(f"  Extracted {len(extracted)} files")
                for src, dest in extracted[:5]:
                    print(f"    {dest} <- {src}")
                if len(extracted) > 5:
                    print(f"    ... and {len(extracted) - 5} more")
    
    return total_extracted


def list_sources():
    """List available source repositories."""
    print("Available sources:\n")
    for i, source in enumerate(SOURCES, 1):
        print(f"{i}. {source.get('description', source['repo'])}")
        print(f"   Repo: {source['repo']}")
        print(f"   Branch: {source.get('branch', 'main')}")
        print(f"   Paths: {', '.join(source['paths'])}")
        print()


def clean(output_dir: Path):
    """Remove extracted_examples directory."""
    if output_dir.exists():
        shutil.rmtree(output_dir)
        print(f"Removed {output_dir}")
    else:
        print(f"{output_dir} does not exist")


def main():
    parser = argparse.ArgumentParser(
        description="Fetch Verus source files for dataset generation"
    )
    parser.add_argument(
        "--list", action="store_true",
        help="List available source repositories"
    )
    parser.add_argument(
        "--clean", action="store_true", 
        help="Remove extracted_examples directory"
    )
    parser.add_argument(
        "--output", type=str, default="extracted_examples",
        help="Output directory (default: extracted_examples)"
    )
    parser.add_argument(
        "-q", "--quiet", action="store_true",
        help="Quiet mode (less output)"
    )
    
    args = parser.parse_args()
    output_dir = Path(args.output)
    
    if args.list:
        list_sources()
        return
    
    if args.clean:
        clean(output_dir)
        return
    
    # Check for git
    if shutil.which("git") is None:
        print("Error: git not found. Please install git.")
        return
    
    print("Fetching Verus source files...")
    print(f"Output: {output_dir.absolute()}")
    
    total = fetch_sources(output_dir, verbose=not args.quiet)
    
    print(f"\n{'='*50}")
    print(f"Total: {total} files extracted to {output_dir}/")
    print(f"\nNext steps:")
    print(f"  1. Run minimizer:  python3 run_minimizer.py")
    print(f"  2. Check metrics:  python3 verus_metrics.py")


if __name__ == "__main__":
    main()

