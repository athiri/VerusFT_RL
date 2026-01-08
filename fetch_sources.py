#!/usr/bin/env python3
"""
Unified Verus Source Fetcher

Fetches Verus source files from curated repositories for dataset generation.
Combines high-yield benchmark datasets with verified systems code.

Usage:
    python3 fetch_sources.py                    # Fetch from all sources
    python3 fetch_sources.py --list             # List available sources
    python3 fetch_sources.py --repos vericoding,human-eval  # Fetch specific repos
    python3 fetch_sources.py --estimate         # Estimate yield without fetching
    python3 fetch_sources.py --github-search    # Search GitHub for more repos
    python3 fetch_sources.py --clean            # Remove extracted_examples/
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR = Path(__file__).parent
DEFAULT_OUTPUT_DIR = SCRIPT_DIR / "extracted_examples"
MANIFEST_FILE = SCRIPT_DIR / "source_manifest.json"

# Minimum file requirements
MIN_FILE_SIZE = 100
MIN_LOC = 5

# Keywords that indicate Verus verification content
VERUS_KEYWORDS = [
    "verus!",
    "requires",
    "ensures",
    "proof fn",
    "spec fn",
    "invariant",
    "decreases",
]

# File patterns to skip
SKIP_PATTERNS = [
    r"mod\.rs$",
    r"lib\.rs$",
    r"main\.rs$",
    r"build\.rs$",
    r"_test\.rs$",
]

# ============================================================================
# Repository Definitions
# ============================================================================

SOURCES: Dict[str, dict] = {
    # === HIGH-YIELD BENCHMARK DATASETS ===
    "vericoding": {
        "repo": "https://github.com/Beneficial-AI-Foundation/vericoding.git",
        "branch": "main",
        "paths": ["benchmarks", "src", "."],
        "description": "Vericoding - competitive programming benchmarks",
        "priority": 1,
        "est_files": 13000,
    },
    "human-eval": {
        "repo": "https://github.com/secure-foundations/human-eval-verus.git",
        "branch": "main",
        "paths": ["tasks"],
        "description": "HumanEval-Verus - 167 hand-verified benchmark problems",
        "priority": 1,
        "est_files": 170,
    },

    # === VERUS CORE ===
    "verus-tests": {
        "repo": "https://github.com/verus-lang/verus.git",
        "branch": "main",
        "paths": ["source/rust_verify_test/tests"],
        "description": "Verus core test suite",
        "priority": 1,
        "est_files": 140,
    },
    "vstd": {
        "repo": "https://github.com/verus-lang/verus.git",
        "branch": "main",
        "paths": ["source/vstd"],
        "description": "Verus standard library (vstd)",
        "priority": 1,
        "est_files": 90,
    },

    # === MICROSOFT PROJECTS ===
    "verismo": {
        "repo": "https://github.com/microsoft/verismo.git",
        "branch": "main",
        "paths": ["source/verismo/src", "source/verismo_main/src"],
        "description": "Microsoft Verismo - verified confidential firmware",
        "priority": 1,
        "est_files": 165,
    },

    # === VERUS-LANG VERIFIED SYSTEMS ===
    "ironkv": {
        "repo": "https://github.com/verus-lang/verified-ironkv.git",
        "branch": "main",
        "paths": ["ironsht/src", "."],
        "description": "Verified IronKV - key-value store",
        "priority": 2,
        "est_files": 65,
    },
    "node-replication": {
        "repo": "https://github.com/verus-lang/verified-node-replication.git",
        "branch": "main",
        "paths": ["verified-node-replication/src", "."],
        "description": "Verified node replication library",
        "priority": 2,
        "est_files": 30,
    },
    "mimalloc": {
        "repo": "https://github.com/verus-lang/verified-memory-allocator.git",
        "branch": "main",
        "paths": ["."],
        "description": "Verified mimalloc implementation",
        "priority": 2,
        "est_files": 30,
    },

    # === COMMUNITY/RESEARCH ===
    "vostd": {
        "repo": "https://github.com/asterinas/vostd.git",
        "branch": "main",
        "paths": ["."],
        "description": "Asterinas VOSTD - auto-generated specs",
        "priority": 3,
        "est_files": 120,
    },
    "betrfs": {
        "repo": "https://github.com/vmware-labs/verified-betrfs.git",
        "branch": "main",
        "paths": ["."],
        "description": "VMware verified BetrFS components",
        "priority": 3,
        "est_files": 20,
    },
}

# ============================================================================
# Data Classes
# ============================================================================

@dataclass
class ExtractedFile:
    source_repo: str
    source_path: str
    dest_path: str
    loc: int
    keywords_found: List[str] = field(default_factory=list)


@dataclass
class RepoStats:
    name: str
    success: bool
    files_extracted: int
    total_loc: int
    error_message: Optional[str] = None


# ============================================================================
# Core Functions
# ============================================================================

def has_verus_content(content: str) -> Tuple[bool, List[str]]:
    """Check if file contains Verus verification constructs."""
    found = [kw for kw in VERUS_KEYWORDS if kw in content]
    return len(found) > 0, found


def should_include(filepath: Path) -> bool:
    """Check if file should be included based on patterns."""
    name = filepath.name
    if not name.endswith('.rs'):
        return False
    if any(re.search(p, name) for p in SKIP_PATTERNS):
        return False
    return True


def count_loc(content: str) -> int:
    """Count non-empty, non-comment lines."""
    lines = content.split('\n')
    count = 0
    in_block = False
    for line in lines:
        stripped = line.strip()
        if '/*' in stripped:
            in_block = True
        if '*/' in stripped:
            in_block = False
            continue
        if in_block or not stripped or stripped.startswith('//'):
            continue
        count += 1
    return count


def get_next_file_index(output_dir: Path) -> int:
    """Get the next available file index."""
    existing = list(output_dir.glob("example_*.rs"))
    if not existing:
        return 1
    max_idx = 0
    for f in existing:
        match = re.search(r'example_(\d+)', f.name)
        if match:
            max_idx = max(max_idx, int(match.group(1)))
    return max_idx + 1


def clone_repo(repo_url: str, branch: str, dest_dir: Path) -> Tuple[bool, str]:
    """Clone a repository (shallow clone)."""
    try:
        result = subprocess.run(
            ["git", "clone", "--depth", "1", "--branch", branch, repo_url, str(dest_dir)],
            capture_output=True,
            text=True,
            timeout=300,
        )
        if result.returncode != 0:
            error = result.stderr.strip().split('\n')[-1] if result.stderr else "Unknown error"
            return False, error
        return True, ""
    except subprocess.TimeoutExpired:
        return False, "Clone timed out"
    except Exception as e:
        return False, str(e)


def extract_from_repo(
    repo_dir: Path,
    repo_name: str,
    extract_paths: List[str],
    output_dir: Path,
    start_index: int,
) -> Tuple[List[ExtractedFile], int]:
    """Extract Verus files from a cloned repository.
    
    Note: Tracks seen source paths to avoid duplicates when extract_paths overlap
    (e.g., if both a parent directory and subdirectory are specified).
    """
    extracted = []
    current_index = start_index
    seen_paths: set = set()  # Track resolved source paths to avoid duplicates

    for extract_path in extract_paths:
        source_dir = repo_dir if extract_path == "." else repo_dir / extract_path
        if not source_dir.exists():
            continue

        for rs_file in source_dir.rglob("*.rs"):
            # Dedupe: skip if we've already processed this file
            resolved_path = rs_file.resolve()
            if resolved_path in seen_paths:
                continue
            seen_paths.add(resolved_path)
            
            if not should_include(rs_file):
                continue

            try:
                if rs_file.stat().st_size < MIN_FILE_SIZE:
                    continue
                content = rs_file.read_text()
            except (OSError, FileNotFoundError, UnicodeDecodeError):
                continue

            is_verus, keywords = has_verus_content(content)
            if not is_verus:
                continue

            loc = count_loc(content)
            if loc < MIN_LOC:
                continue

            dest_name = f"example_{current_index:05d}.rs"
            dest_path = output_dir / dest_name

            while dest_path.exists():
                current_index += 1
                dest_name = f"example_{current_index:05d}.rs"
                dest_path = output_dir / dest_name

            shutil.copy2(rs_file, dest_path)
            extracted.append(ExtractedFile(
                source_repo=repo_name,
                source_path=str(rs_file.relative_to(repo_dir)),
                dest_path=dest_name,
                loc=loc,
                keywords_found=keywords,
            ))
            current_index += 1

    return extracted, current_index


def fetch_repo(
    repo_name: str,
    repo_config: dict,
    output_dir: Path,
    start_index: int,
    verbose: bool = True,
) -> Tuple[List[ExtractedFile], RepoStats, int]:
    """Fetch and extract files from a single repository."""
    repo_url = repo_config["repo"]
    branch = repo_config.get("branch", "main")
    paths = repo_config["paths"]
    desc = repo_config.get("description", repo_name)

    if verbose:
        print(f"\n📦 {desc}")
        print(f"   Repo: {repo_url}")
        print(f"   Cloning...")

    with tempfile.TemporaryDirectory() as tmp:
        repo_dir = Path(tmp) / "repo"
        success, error = clone_repo(repo_url, branch, repo_dir)

        if not success:
            if verbose:
                print(f"   ❌ Clone failed: {error}")
            return [], RepoStats(repo_name, False, 0, 0, error), start_index

        if verbose:
            print(f"   Extracting Verus files...")

        extracted, next_index = extract_from_repo(
            repo_dir, repo_name, paths, output_dir, start_index
        )
        total_loc = sum(f.loc for f in extracted)

        if verbose:
            if extracted:
                print(f"   ✅ Extracted {len(extracted)} files ({total_loc:,} LOC)")
            else:
                print(f"   ⚠️  No Verus files found")

        return extracted, RepoStats(repo_name, True, len(extracted), total_loc), next_index


def fetch_all(
    repo_names: Optional[List[str]] = None,
    output_dir: Optional[Path] = None,
    verbose: bool = True,
    save_manifest: bool = True,
) -> Tuple[List[ExtractedFile], List[RepoStats]]:
    """Fetch files from all (or selected) repositories."""
    if output_dir is None:
        output_dir = DEFAULT_OUTPUT_DIR
    output_dir.mkdir(parents=True, exist_ok=True)

    # Determine repos to fetch
    if repo_names:
        repos = {k: v for k, v in SOURCES.items() if k in repo_names}
        unknown = set(repo_names) - set(repos.keys())
        if unknown:
            print(f"Warning: Unknown repos ignored: {unknown}")
            print(f"Available: {', '.join(SOURCES.keys())}")
    else:
        repos = SOURCES

    # Sort by priority
    sorted_repos = sorted(repos.items(), key=lambda x: x[1].get("priority", 99))

    if verbose:
        print("=" * 65)
        print("Verus Source Fetcher")
        print("=" * 65)
        print(f"Output: {output_dir.absolute()}")
        print(f"Repositories: {len(sorted_repos)}")
        print("=" * 65)

    start_index = get_next_file_index(output_dir)
    if verbose and start_index > 1:
        print(f"Continuing from index {start_index} ({start_index - 1} existing files)")

    all_extracted: List[ExtractedFile] = []
    all_stats: List[RepoStats] = []
    current_index = start_index

    for i, (name, config) in enumerate(sorted_repos, 1):
        if verbose:
            print(f"\n[{i}/{len(sorted_repos)}]", end="")
        extracted, stats, current_index = fetch_repo(
            name, config, output_dir, current_index, verbose
        )
        all_extracted.extend(extracted)
        all_stats.append(stats)

    # Summary
    total_files = len(all_extracted)
    total_loc = sum(f.loc for f in all_extracted)
    successful = sum(1 for s in all_stats if s.success)

    if verbose:
        final_count = len(list(output_dir.glob("example_*.rs")))
        print("\n" + "=" * 65)
        print("Extraction Complete!")
        print("=" * 65)
        print(f"Repositories: {successful}/{len(sorted_repos)} successful")
        print(f"New files: {total_files:,}")
        print(f"New LOC: {total_loc:,}")
        print(f"Total in {output_dir.name}/: {final_count:,}")
        print("=" * 65)

    # Save manifest
    if save_manifest and all_extracted:
        manifest = {
            "timestamp": datetime.now().isoformat(),
            "total_files": total_files,
            "total_loc": total_loc,
            "repositories": [
                {"name": s.name, "success": s.success, "files": s.files_extracted, "loc": s.total_loc}
                for s in all_stats
            ],
            "files": [
                {"dest": f.dest_path, "repo": f.source_repo, "src": f.source_path, "loc": f.loc}
                for f in all_extracted[:1000]  # Limit manifest size
            ],
        }
        with open(MANIFEST_FILE, 'w') as f:
            json.dump(manifest, f, indent=2)
        if verbose:
            print(f"\nManifest: {MANIFEST_FILE}")

    if verbose:
        print("\nNext steps:")
        print("  1. Run minimizer:  python3 run_minimizer.py 100")
        print("  2. Check metrics:  python3 verus_metrics.py")

    return all_extracted, all_stats


# ============================================================================
# Utility Functions
# ============================================================================

def list_sources():
    """List all available repositories."""
    print("\n" + "=" * 65)
    print("Available Verus Repositories")
    print("=" * 65)

    sorted_repos = sorted(SOURCES.items(), key=lambda x: x[1].get("priority", 99))
    total_est = 0

    for name, config in sorted_repos:
        priority = config.get("priority", 3)
        est = config.get("est_files", 0)
        total_est += est
        stars = "★" * (4 - priority)

        print(f"\n{stars} {name}")
        print(f"   {config.get('description', 'No description')}")
        print(f"   Repo: {config['repo']}")
        print(f"   Est. files: ~{est:,}")

    print("\n" + "=" * 65)
    print(f"Total: {len(SOURCES)} repositories, ~{total_est:,} estimated files")
    print("=" * 65)


def estimate_yield():
    """Estimate total yield without fetching."""
    print("\n" + "=" * 65)
    print("Estimated Dataset Yield")
    print("=" * 65)

    total = 0
    for name, config in sorted(SOURCES.items(), key=lambda x: -x[1].get("est_files", 0)):
        est = config.get("est_files", 0)
        total += est
        print(f"  {name}: ~{est:,} files")

    print("-" * 40)
    print(f"  TOTAL: ~{total:,} files")
    print("\nNote: Actual yield depends on filtering and deduplication")
    print("=" * 65)


def search_github():
    """Search GitHub for additional Verus repositories."""
    print("\n🔍 Searching GitHub for Verus repositories...")

    if shutil.which("gh") is None:
        print("   ⚠️  GitHub CLI (gh) not found. Install: brew install gh")
        return

    queries = ["verus! language:rust", "vstd verus", "requires ensures language:rust"]
    found = {}

    for query in queries:
        try:
            result = subprocess.run(
                ["gh", "search", "repos", query, "--limit", "20", "--json", "fullName,description,url"],
                capture_output=True,
                text=True,
                timeout=30,
            )
            if result.returncode == 0:
                for repo in json.loads(result.stdout):
                    name = repo.get("fullName", "")
                    if name and name not in found:
                        known = [s["repo"] for s in SOURCES.values()]
                        url = f"https://github.com/{name}.git"
                        if url not in known:
                            found[name] = repo.get("description", "")[:60]
        except Exception:
            pass

    if found:
        print(f"\n   Found {len(found)} additional repositories:\n")
        for name, desc in list(found.items())[:15]:
            print(f"   • {name}")
            if desc:
                print(f"     {desc}")
    else:
        print("   No additional repositories found")


def clean(output_dir: Path):
    """Remove extracted_examples directory."""
    if output_dir.exists():
        count = len(list(output_dir.glob("*.rs")))
        shutil.rmtree(output_dir)
        print(f"Removed {output_dir} ({count} files)")
    else:
        print(f"{output_dir} does not exist")


# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Fetch Verus source files for dataset generation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 fetch_sources.py                     # Fetch all
  python3 fetch_sources.py --repos vericoding  # Fetch specific repo
  python3 fetch_sources.py --list              # List repos
  python3 fetch_sources.py --estimate          # Estimate yield
        """
    )
    parser.add_argument("--list", action="store_true", help="List available repositories")
    parser.add_argument("--estimate", action="store_true", help="Estimate yield without fetching")
    parser.add_argument("--repos", type=str, help="Comma-separated repos to fetch")
    parser.add_argument("--output", type=str, default="extracted_examples", help="Output directory")
    parser.add_argument("--github-search", action="store_true", help="Search GitHub for more repos")
    parser.add_argument("--clean", action="store_true", help="Remove output directory")
    parser.add_argument("-q", "--quiet", action="store_true", help="Quiet mode")

    args = parser.parse_args()
    output_dir = Path(args.output)

    if args.list:
        list_sources()
        return

    if args.estimate:
        estimate_yield()
        return

    if args.github_search:
        search_github()
        return

    if args.clean:
        clean(output_dir)
        return

    if shutil.which("git") is None:
        print("Error: git not found. Please install git.")
        return

    repo_names = [r.strip() for r in args.repos.split(',')] if args.repos else None
    fetch_all(repo_names=repo_names, output_dir=output_dir, verbose=not args.quiet)


if __name__ == "__main__":
    main()
