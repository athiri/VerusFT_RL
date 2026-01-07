#!/usr/bin/env python3
"""
Verus Dataset Metrics - Source of Truth: verus command

Tracks Verus-specific features for dataset quality assessment:
- proof fn, spec fn, exec fn
- requires, ensures, recommends
- invariants, decreases
- assert statements
- bit_vector proofs

Uses analysis.py for semantic analysis integration.
"""

import subprocess
import json
import re
import hashlib
import argparse
from pathlib import Path
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional
from collections import Counter
from concurrent.futures import ProcessPoolExecutor, as_completed
import os

# Import existing analysis tools
try:
    from analysis import analyze_file, SPEC_KEYWORDS, VERUS_TOKENS, STUB_PATTERNS
except ImportError:
    SPEC_KEYWORDS = {"requires", "ensures", "invariant", "decreases", "spec fn", "proof fn"}
    VERUS_TOKENS = {"verus!", "requires", "ensures", "decreases", "invariant", "ghost", "proof", "spec"}
    STUB_PATTERNS = {"assume(false)", "unreached()"}
    analyze_file = None

VERUS_PATH = "/Users/athiri/Desktop/verus/source/target-verus/release/verus"
EXAMPLES_DIR = Path("minimized_examples")
CACHE_FILE = Path(".verus_metrics_cache.json")
MAX_WORKERS = os.cpu_count() or 4  # Parallel verification workers


@dataclass
class VerusFeatures:
    """Verus language features in a file"""
    proof_fn: int = 0
    spec_fn: int = 0
    exec_fn: int = 0
    requires: int = 0
    ensures: int = 0
    recommends: int = 0
    invariant: int = 0
    decreases: int = 0
    assert_stmt: int = 0
    bit_vector: int = 0
    forall: int = 0
    exists: int = 0
    ghost: int = 0


@dataclass
class VerusResult:
    """Result from running verus command"""
    file: str
    verified: int
    errors: int
    exit_code: int
    status: str  # verified, spec_only, error
    features: VerusFeatures
    lines: int
    semantic_quality: str  # meaningful, trivial, empty
    is_stub: bool = False  # True if contains assume(false) or unreached()


def check_semantic_quality(content: str, features: VerusFeatures, filepath: Path = None) -> str:
    """
    Classify file's training value per PROJECT_PROPOSAL.md acceptance criteria.
    
    From proposal line 126:
    "Meaningful: Has at least one requires/ensures clause OR a loop invariant OR a proof block"
    
    Classification (mutually exclusive, checked in order):
    
    1. EMPTY:      No functions or only empty fn main() {}
    2. MEANINGFUL: Has requires OR ensures OR invariant OR proof fn
    3. TRIVIAL:    Has functions but doesn't meet meaningful criteria
    """
    # Count total functions
    total_fns = features.proof_fn + features.spec_fn + features.exec_fn
    
    # Rule 1: EMPTY - no functions, or only empty main
    if total_fns == 0:
        return "empty"
    
    has_only_empty_main = (
        total_fns == 1 and 
        features.exec_fn == 1 and
        re.search(r'fn\s+main\s*\(\s*\)\s*\{\s*\}', content)
    )
    if has_only_empty_main:
        return "empty"
    
    # Rule 2: MEANINGFUL - per proposal definition
    # "Has at least one requires/ensures clause OR a loop invariant OR a proof block"
    is_meaningful = (
        features.requires > 0 or
        features.ensures > 0 or
        features.invariant > 0 or
        features.proof_fn > 0
    )
    
    if is_meaningful:
        return "meaningful"
    
    # Rule 3: TRIVIAL - has functions but doesn't meet meaningful criteria
    return "trivial"


def strip_comments_and_strings(content: str) -> str:
    """
    Remove comments and string literals to avoid false positives.
    Only count keywords in actual code, not in comments or strings.
    """
    result = []
    i = 0
    in_string = False
    string_char = None
    
    while i < len(content):
        # Line comment
        if not in_string and i + 1 < len(content) and content[i:i+2] == '//':
            while i < len(content) and content[i] != '\n':
                i += 1
            continue
        
        # Block comment
        if not in_string and i + 1 < len(content) and content[i:i+2] == '/*':
            i += 2
            while i + 1 < len(content) and content[i:i+2] != '*/':
                i += 1
            i += 2
            continue
        
        # String literals
        if content[i] in '"\'':
            if not in_string:
                in_string = True
                string_char = content[i]
                i += 1
                continue
            elif content[i] == string_char and (i == 0 or content[i-1] != '\\'):
                in_string = False
                string_char = None
                i += 1
                continue
        
        if not in_string:
            result.append(content[i])
        i += 1
    
    return ''.join(result)


def extract_features(content: str) -> VerusFeatures:
    """
    Extract Verus language features from actual code.
    Strips comments and strings first to avoid false positives.
    """
    # Strip comments and strings for accurate counting
    clean = strip_comments_and_strings(content)
    
    return VerusFeatures(
        proof_fn=len(re.findall(r'proof\s+fn\s+', clean)),
        spec_fn=len(re.findall(r'spec\s+fn\s+', clean)),
        exec_fn=max(0, len(re.findall(r'\bfn\s+\w+', clean)) - 
                len(re.findall(r'proof\s+fn\s+', clean)) - 
                len(re.findall(r'spec\s+fn\s+', clean))),
        requires=len(re.findall(r'requires\b', clean)),
        ensures=len(re.findall(r'ensures\b', clean)),
        recommends=len(re.findall(r'recommends\b', clean)),
        invariant=len(re.findall(r'invariant\b', clean)),
        decreases=len(re.findall(r'decreases\b', clean)),
        assert_stmt=len(re.findall(r'assert\s*[\(\!]', clean)),
        bit_vector=len(re.findall(r'by\s*\(\s*bit_vector\s*\)', clean)),
        forall=len(re.findall(r'forall\b', clean)),
        exists=len(re.findall(r'exists\b', clean)),
        ghost=len(re.findall(r'\bghost\b', clean)) + len(re.findall(r'\bGhost\b', clean)),
    )


def has_stub_patterns(content: str) -> bool:
    """Check if content contains bogus stub patterns (assume(false), unreached())"""
    return any(pattern in content for pattern in STUB_PATTERNS)


def verify_file(filepath: Path) -> VerusResult:
    """Run verus command and get result"""
    content = filepath.read_text()
    features = extract_features(content)
    lines = len(content.split('\n'))
    semantic_quality = check_semantic_quality(content, features, filepath)
    is_stub = has_stub_patterns(content)
    
    try:
        result = subprocess.run(
            [VERUS_PATH, "--crate-type=lib", str(filepath)],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        output = result.stdout + result.stderr
        
        # Parse verification results
        match = re.search(r'verification results::\s*(\d+)\s+verified,\s*(\d+)\s+errors', output)
        
        if match:
            verified = int(match.group(1))
            errors = int(match.group(2))
        else:
            verified, errors = 0, 0
        
        # Determine status - check exit code first (consistent with verify_minimized.sh)
        if result.returncode != 0:
            status = "error"
        elif errors > 0:
            status = "error"
        elif verified > 0:
            status = "verified"
        else:
            status = "spec_only"
        
        return VerusResult(
            file=filepath.name,
            verified=verified,
            errors=errors,
            exit_code=result.returncode,
            status=status,
            features=features,
            lines=lines,
            semantic_quality=semantic_quality,
            is_stub=is_stub
        )
        
    except subprocess.TimeoutExpired:
        return VerusResult(
            file=filepath.name,
            verified=0,
            errors=0,
            exit_code=-1,
            status="timeout",
            features=features,
            lines=lines,
            semantic_quality=semantic_quality,
            is_stub=is_stub
        )


def get_file_hash(filepath: Path) -> str:
    """Get hash of file content for cache invalidation"""
    content = filepath.read_text()
    return hashlib.md5(content.encode()).hexdigest()


def load_cache() -> Dict:
    """Load cached verification results"""
    if CACHE_FILE.exists():
        try:
            with open(CACHE_FILE) as f:
                return json.load(f)
        except:
            pass
    return {}


def save_cache(cache: Dict):
    """Save cache to disk"""
    with open(CACHE_FILE, 'w') as f:
        json.dump(cache, f)


def verify_file_cached(filepath: Path, cache: Dict) -> Optional[VerusResult]:
    """Check if we have a valid cached result for this file"""
    key = str(filepath)
    if key in cache:
        cached = cache[key]
        current_hash = get_file_hash(filepath)
        if cached.get("hash") == current_hash:
            # Reconstruct VerusResult from cache
            return VerusResult(
                file=cached["file"],
                verified=cached["verified"],
                errors=cached["errors"],
                exit_code=cached["exit_code"],
                status=cached["status"],
                features=VerusFeatures(**cached["features"]),
                lines=cached["lines"],
                semantic_quality=cached["semantic_quality"],
                is_stub=cached.get("is_stub", False)
            )
    return None


def verify_and_cache(filepath: Path) -> tuple:
    """Verify file and return (filepath, result, hash) for caching"""
    result = verify_file(filepath)
    file_hash = get_file_hash(filepath)
    return (str(filepath), result, file_hash)


def main():
    """Run metrics on all examples"""
    parser = argparse.ArgumentParser(description="Verus Dataset Metrics")
    parser.add_argument("--no-cache", action="store_true", help="Ignore cache and re-verify all files")
    parser.add_argument("--workers", type=int, default=MAX_WORKERS, help=f"Number of parallel workers (default: {MAX_WORKERS})")
    args = parser.parse_args()
    
    print("Verus Dataset Metrics")
    print("=" * 60)
    
    files = sorted(EXAMPLES_DIR.glob("*.rs"))
    total_files = len(files)
    
    # Load cache
    cache = {} if args.no_cache else load_cache()
    
    results = []
    to_verify = []
    
    # Check cache first
    for filepath in files:
        cached_result = verify_file_cached(filepath, cache)
        if cached_result:
            results.append(cached_result)
        else:
            to_verify.append(filepath)
    
    cached_count = len(results)
    verify_count = len(to_verify)
    
    if cached_count > 0:
        print(f"\n  Using {cached_count} cached results, verifying {verify_count} files...")
    else:
        print(f"\n  Verifying {verify_count} files with {args.workers} parallel workers...")
    
    # Parallel verification of non-cached files
    if to_verify:
        completed = 0
        with ProcessPoolExecutor(max_workers=args.workers) as executor:
            futures = {executor.submit(verify_and_cache, fp): fp for fp in to_verify}
            
            for future in as_completed(futures):
                filepath_str, result, file_hash = future.result()
                results.append(result)
                
                # Update cache
                cache[filepath_str] = {
                    "hash": file_hash,
                    "file": result.file,
                    "verified": result.verified,
                    "errors": result.errors,
                    "exit_code": result.exit_code,
                    "status": result.status,
                    "features": asdict(result.features),
                    "lines": result.lines,
                    "semantic_quality": result.semantic_quality,
                    "is_stub": result.is_stub
                }
                
                completed += 1
                if completed % 50 == 0 or completed == verify_count:
                    print(f"  Progress: {completed}/{verify_count} verified")
        
        # Save updated cache
        save_cache(cache)
    
    # Sort results by filename for consistent output
    results.sort(key=lambda r: r.file)
    
    # Aggregate stats
    verified = [r for r in results if r.status == "verified"]
    spec_only = [r for r in results if r.status == "spec_only"]
    errors = [r for r in results if r.status == "error"]
    timeouts = [r for r in results if r.status == "timeout"]
    
    # Stub detection - critical for SFT quality
    stubs = [r for r in results if r.is_stub]
    legitimate = [r for r in results if not r.is_stub]
    
    print(f"\n⚠️  STUB DETECTION (critical for SFT)")
    print("-" * 60)
    print(f"  Stubs (assume(false)/unreached): {len(stubs):5d}  ({100*len(stubs)/len(results):.1f}%) ❌ NOT for training")
    print(f"  Legitimate implementations:      {len(legitimate):5d}  ({100*len(legitimate)/len(results):.1f}%) ✅ OK for training")
    
    print(f"\nVerification Status (source: verus command)")
    print("-" * 60)
    print(f"  Verified (>=1):    {len(verified):5d}  ({100*len(verified)/len(results):.1f}%)")
    print(f"  Spec-only (0):     {len(spec_only):5d}  ({100*len(spec_only)/len(results):.1f}%)")
    print(f"  Errors:            {len(errors):5d}  ({100*len(errors)/len(results):.1f}%)")
    print(f"  Timeouts:          {len(timeouts):5d}")
    print(f"  Total:             {len(results):5d}")
    
    # Verification count distribution
    print(f"\nVerification Count Distribution")
    print("-" * 60)
    verify_counts = Counter(r.verified for r in results if r.status != "error" and r.status != "timeout")
    for count in sorted(verify_counts.keys()):
        num_files = verify_counts[count]
        bar = "█" * min(num_files, 40)
        print(f"  {count:2d} verified: {num_files:3d}  {bar}")
    
    # Verus construct distribution
    print(f"\nVerus Construct Distribution")
    print("-" * 60)
    
    total_features = VerusFeatures()
    for r in results:
        for field in total_features.__dataclass_fields__:
            setattr(total_features, field, 
                    getattr(total_features, field) + getattr(r.features, field))
    
    print(f"  proof fn:     {total_features.proof_fn:3d}")
    print(f"  spec fn:      {total_features.spec_fn:3d}")
    print(f"  exec fn:      {total_features.exec_fn:3d}")
    print(f"  requires:     {total_features.requires:3d}")
    print(f"  ensures:      {total_features.ensures:3d}")
    print(f"  recommends:   {total_features.recommends:3d}")
    print(f"  invariant:    {total_features.invariant:3d}")
    print(f"  decreases:    {total_features.decreases:3d}")
    print(f"  assert:       {total_features.assert_stmt:3d}")
    print(f"  bit_vector:   {total_features.bit_vector:3d}")
    print(f"  forall:       {total_features.forall:3d}")
    print(f"  exists:       {total_features.exists:3d}")
    print(f"  ghost:        {total_features.ghost:3d}")
    
    # Semantic quality
    meaningful = [r for r in results if r.semantic_quality == "meaningful"]
    trivial = [r for r in results if r.semantic_quality == "trivial"]
    empty = [r for r in results if r.semantic_quality == "empty"]
    
    print(f"\nSemantic Quality (training value)")
    print("-" * 60)
    print(f"  Meaningful:    {len(meaningful):3d}  ({100*len(meaningful)/len(results):.1f}%) - good for training")
    print(f"  Trivial:       {len(trivial):3d}  ({100*len(trivial)/len(results):.1f}%) - low value")
    print(f"  Empty:         {len(empty):3d}  ({100*len(empty)/len(results):.1f}%) - no training value")
    
    if empty:
        print(f"\n  Empty files (should remove):")
        for r in empty[:10]:
            print(f"    {r.file}")
        if len(empty) > 10:
            print(f"    ... and {len(empty) - 10} more")
    
    # Files with specific features (for training value)
    print(f"\nVerus Constructs (files containing)")
    print("-" * 60)
    has_proof = sum(1 for r in results if r.features.proof_fn > 0)
    has_ensures = sum(1 for r in results if r.features.ensures > 0)
    has_requires = sum(1 for r in results if r.features.requires > 0)
    has_invariant = sum(1 for r in results if r.features.invariant > 0)
    has_assert = sum(1 for r in results if r.features.assert_stmt > 0)
    
    print(f"  proof fn:    {has_proof:3d}")
    print(f"  ensures:     {has_ensures:3d}")
    print(f"  requires:    {has_requires:3d}")
    print(f"  invariant:   {has_invariant:3d}")
    print(f"  assert:      {has_assert:3d}")
    
    # Save detailed results
    output_file = Path("verus_metrics.json")
    with open(output_file, 'w') as f:
        json.dump({
            "summary": {
                "total": len(results),
                "verified": len(verified),
                "spec_only": len(spec_only),
                "errors": len(errors),
                "timeouts": len(timeouts),
                "stubs": len(stubs),
                "legitimate": len(legitimate)
            },
            "features": asdict(total_features),
            "quality": {
                "files_with_proof_fn": has_proof,
                "files_with_ensures": has_ensures,
                "files_with_requires": has_requires,
                "files_with_invariant": has_invariant,
                "files_with_assert": has_assert
            },
            "results": [asdict(r) for r in results]
        }, f, indent=2)
    
    # Generate metadata spreadsheet (CSV) per deliverables
    ORIGINAL_DIR = Path("extracted_examples")
    csv_file = Path("dataset_metadata.csv")
    
    with open(csv_file, 'w') as f:
        f.write("file_path,original_LOC,minimized_LOC,reduction_ratio,status,readability_score,self_contained,verifiable,is_stub,sft_suitable\n")
        
        for r in results:
            # Get original LOC if available
            original_path = ORIGINAL_DIR / r.file
            if original_path.exists():
                original_loc = len(original_path.read_text().split('\n'))
            else:
                original_loc = r.lines  # Use current if no original
            
            minimized_loc = r.lines
            reduction = 1.0 - (minimized_loc / original_loc) if original_loc > 0 else 0.0
            
            # Readability score: meaningful=5, trivial=3, empty=1
            readability = {"meaningful": 5, "trivial": 3, "empty": 1}.get(r.semantic_quality, 1)
            
            # Self-contained: check for only vstd/std dependencies
            content = (EXAMPLES_DIR / r.file).read_text()
            deps = re.findall(r'^use\s+(\w+)', content, re.MULTILINE)
            self_contained = all(d in {'vstd', 'std', 'core'} for d in deps) if deps else True
            
            # Verifiable: verified or spec_only (compiles without errors)
            verifiable = r.status in {"verified", "spec_only"}
            
            # SFT suitable: verifiable AND not a stub
            sft_suitable = verifiable and not r.is_stub
            
            f.write(f"{r.file},{original_loc},{minimized_loc},{reduction:.2f},{r.status},{readability},{self_contained},{verifiable},{r.is_stub},{sft_suitable}\n")
    
    print(f"\nDetailed results saved to: {output_file}")
    print(f"Metadata spreadsheet saved to: {csv_file}")


if __name__ == "__main__":
    main()

