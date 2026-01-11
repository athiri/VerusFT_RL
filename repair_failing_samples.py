#!/usr/bin/env python3
"""
LLM-Assisted Repair for Failing Verus Samples

This script attempts to repair failing Verus samples using LLM assistance.
It reads the dataset_metadata.csv to identify failing samples, runs Verus
to get error messages, and uses an LLM to suggest fixes.

Usage:
    python3 repair_failing_samples.py                    # Repair all failing samples
    python3 repair_failing_samples.py --batch 50         # Process 50 samples
    python3 repair_failing_samples.py --dry-run          # Show what would be repaired
    python3 repair_failing_samples.py --model claude     # Use Claude (default)
    python3 repair_failing_samples.py --model openai     # Use OpenAI

Environment Variables:
    ANTHROPIC_API_KEY - Required for Claude
    OPENAI_API_KEY    - Required for OpenAI
    VERUS_PATH        - Path to Verus executable (optional)
"""

import argparse
import csv
import json
import os
import re
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR = Path(__file__).parent
MINIMIZED_DIR = SCRIPT_DIR / "minimized_examples"
REPAIRED_DIR = SCRIPT_DIR / "repaired_examples"
METADATA_FILE = SCRIPT_DIR / "dataset_metadata.csv"
REPAIR_LOG_FILE = SCRIPT_DIR / "repair_log.json"

# Verus path discovery
VERUS_PATH = os.environ.get("VERUS_PATH")
if not VERUS_PATH:
    verus_candidate = SCRIPT_DIR.parent / "verus/source/target-verus/release/verus"
    if verus_candidate.exists():
        VERUS_PATH = str(verus_candidate)
    else:
        VERUS_PATH = "/Users/athiri/Desktop/verus/source/target-verus/release/verus"

MAX_REPAIR_ATTEMPTS = 3
VERUS_TIMEOUT = 60

# ============================================================================
# Data Classes
# ============================================================================

@dataclass
class FailingSample:
    """Represents a failing Verus sample."""
    file_path: str
    original_loc: int
    status: str
    content: str = ""
    error_message: str = ""


@dataclass
class RepairResult:
    """Result of a repair attempt."""
    file_path: str
    success: bool
    attempts: int
    original_error: str
    fixed_code: Optional[str]
    final_error: Optional[str]
    duration_seconds: float


# ============================================================================
# Verus Verification
# ============================================================================

def run_verus(filepath: Path, timeout: int = VERUS_TIMEOUT) -> Tuple[bool, str]:
    """
    Run Verus verification on a file.
    
    Returns: (success, output_or_error)
    """
    try:
        result = subprocess.run(
            [VERUS_PATH, "--crate-type=lib", str(filepath)],
            capture_output=True,
            text=True,
            timeout=timeout
        )
        
        output = result.stdout + result.stderr
        
        # Check for successful verification
        if result.returncode == 0:
            # Also check for verification results pattern
            pattern = r'verification results::\s*(\d+)\s+verified,\s*(\d+)\s+errors'
            match = re.search(pattern, output, re.IGNORECASE)
            if match:
                verified = int(match.group(1))
                errors = int(match.group(2))
                if verified > 0 and errors == 0:
                    return True, output
        
        # Check for specific error patterns
        if 'error:' in output.lower():
            return False, output
        
        return False, output
        
    except subprocess.TimeoutExpired:
        return False, "Verification timed out"
    except Exception as e:
        return False, f"Exception: {str(e)}"


def extract_error_context(error_output: str) -> str:
    """Extract the most relevant error context for the LLM."""
    lines = error_output.split('\n')
    
    # Find error lines
    error_lines = []
    capture = False
    
    for line in lines:
        if 'error' in line.lower() or 'warning' in line.lower():
            capture = True
        if capture:
            error_lines.append(line)
            if len(error_lines) > 30:  # Limit context size
                break
    
    return '\n'.join(error_lines) if error_lines else error_output[:2000]


# ============================================================================
# LLM Integration
# ============================================================================

def repair_with_claude(code: str, error: str, attempt: int) -> Optional[str]:
    """Use Claude to repair Verus code."""
    try:
        import anthropic
    except ImportError:
        print("Error: anthropic package not installed. Run: pip install anthropic")
        return None
    
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("Error: ANTHROPIC_API_KEY environment variable not set")
        return None
    
    client = anthropic.Anthropic(api_key=api_key)
    
    prompt = f"""You are an expert in Verus, a verification language for Rust. 
Your task is to fix the following Verus code that fails verification.

IMPORTANT RULES:
1. Make MINIMAL changes - only fix what's necessary
2. Preserve the original function names and signatures if possible
3. Do NOT add unnecessary imports or helper functions
4. The code must be self-contained (no external dependencies except vstd)
5. Ensure proper verus! macro wrapping
6. Include necessary `use vstd::prelude::*;` if missing

ORIGINAL CODE:
```rust
{code}
```

VERUS ERROR (attempt {attempt}/{MAX_REPAIR_ATTEMPTS}):
```
{error}
```

Provide ONLY the fixed code, no explanations. The code should compile and verify with Verus.
Start with the imports and wrap everything in verus! if needed.
"""

    try:
        response = client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=4096,
            messages=[{"role": "user", "content": prompt}]
        )
        
        # Extract code from response
        response_text = response.content[0].text
        code = extract_code_from_response(response_text)
        if code is None:
            print(f"  Warning: LLM response did not contain valid Verus/Rust code")
        return code
        
    except Exception as e:
        print(f"  Claude API error: {e}")
        return None


def repair_with_openai(code: str, error: str, attempt: int) -> Optional[str]:
    """Use OpenAI to repair Verus code."""
    try:
        import openai
    except ImportError:
        print("Error: openai package not installed. Run: pip install openai")
        return None
    
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        print("Error: OPENAI_API_KEY environment variable not set")
        return None
    
    client = openai.OpenAI(api_key=api_key)
    
    prompt = f"""You are an expert in Verus, a verification language for Rust. 
Fix the following Verus code that fails verification.

RULES:
1. Make MINIMAL changes
2. Preserve original function names/signatures
3. Code must be self-contained (only vstd dependencies)
4. Ensure proper verus! macro wrapping
5. Include `use vstd::prelude::*;` if missing

ORIGINAL CODE:
```rust
{code}
```

VERUS ERROR (attempt {attempt}/{MAX_REPAIR_ATTEMPTS}):
```
{error}
```

Provide ONLY the fixed code, no explanations."""

    try:
        response = client.chat.completions.create(
            model="gpt-4-turbo-preview",
            max_tokens=4096,
            messages=[{"role": "user", "content": prompt}]
        )
        
        response_text = response.choices[0].message.content
        code = extract_code_from_response(response_text)
        if code is None:
            print(f"  Warning: LLM response did not contain valid Verus/Rust code")
        return code
        
    except Exception as e:
        print(f"  OpenAI API error: {e}")
        return None


def is_valid_verus_code(code: str) -> bool:
    """
    Check if extracted code looks like valid Verus/Rust code.
    Rejects Dafny syntax and prose-only content.
    """
    if not code or len(code.strip()) < 10:
        return False
    
    # Reject Dafny syntax patterns
    dafny_patterns = [
        r'\bvar\s+\w+\s*:=',       # var x :=
        r'\bset<\w+>',              # set<int>
        r'\bseq<\w+>',              # seq<int>  
        r'\blemma\s+\w+\s*\(',      # lemma name(
        r'\bfunction\s+\w+\s*\([^)]*\)\s*:\s*\w+',  # function name(...): type (Dafny style)
        r':=\s*\w+\s*\+\s*1\s*;',   # := x + 1;
        r'\ba\s*:\|\s*a\s+in\b',    # a :| a in (Dafny choice)
    ]
    for pattern in dafny_patterns:
        if re.search(pattern, code):
            return False
    
    # Must have Rust/Verus indicators
    rust_indicators = ['fn ', 'let ', 'use ', 'pub ', 'struct ', 'impl ', 'mod ', 
                       'verus!', '#[', 'requires', 'ensures', 'invariant', '->']
    return any(indicator in code for indicator in rust_indicators)


def clean_llm_explanations(code: str) -> str:
    """Remove common LLM explanation patterns from before/after code."""
    lines = code.split('\n')
    
    explanation_patterns = [
        r'^The (key|main|problem|issue|fix)',
        r'^I (made|added|fixed|changed|updated|would)',
        r'^However',
        r'^If you',
        r'^This (fix|change|modification|code)',
        r'^Note:',
        r'^Here',
        r'^\d+\.\s+\w',
        r'^-\s+\w',
        r'^Key (changes|fixes)',
        r'^Main (changes|fixes)',
    ]
    
    # Find where actual code starts
    start_idx = 0
    for i, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue
        is_explanation = any(re.match(p, stripped, re.IGNORECASE) for p in explanation_patterns)
        if not is_explanation:
            code_starters = ['use ', 'fn ', 'pub ', '//', '/*', '#[', 'verus!', 'struct ', 'impl ']
            if any(stripped.startswith(s) for s in code_starters):
                start_idx = i
                break
    
    # Find where actual code ends
    brace_depth = 0
    end_idx = len(lines)
    in_code = False
    
    for i, line in enumerate(lines[start_idx:], start=start_idx):
        stripped = line.strip()
        if 'verus!' in stripped or stripped.startswith('fn ') or stripped.startswith('pub fn'):
            in_code = True
        if in_code:
            brace_depth += stripped.count('{') - stripped.count('}')
            if brace_depth == 0 and i > start_idx:
                remaining = '\n'.join(lines[i+1:]).strip()
                if remaining and any(re.match(p, remaining, re.IGNORECASE) for p in explanation_patterns):
                    end_idx = i + 1
                    break
    
    return '\n'.join(lines[start_idx:end_idx]).strip()


def extract_code_from_response(response: str) -> Optional[str]:
    """
    Extract code from LLM response (handles markdown blocks).
    
    Returns None if no valid Verus/Rust code can be extracted.
    This prevents saving invalid content (Dafny, prose, etc.).
    """
    # Try ```rust or ```verus blocks first
    patterns = [
        r"```(?:rust|verus)\s*\n(.*?)```",
        r"```\s*\n(.*?)```",
    ]
    
    for pattern in patterns:
        matches = re.findall(pattern, response, re.DOTALL)
        if matches:
            code = matches[0].strip()
            if is_valid_verus_code(code):
                return code
    
    # Try to clean and extract from raw response
    cleaned = clean_llm_explanations(response)
    if is_valid_verus_code(cleaned):
        return cleaned
    
    # Return None to indicate extraction failure - caller should handle this
    return None


# ============================================================================
# Main Repair Logic
# ============================================================================

def load_failing_samples() -> List[FailingSample]:
    """Load failing samples from metadata CSV."""
    failing = []
    
    if not METADATA_FILE.exists():
        print(f"Error: Metadata file not found: {METADATA_FILE}")
        return failing
    
    with open(METADATA_FILE, 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            status = row.get('status', '')
            verifiable = row.get('verifiable', 'False')
            
            # Include samples that failed verification
            if status == 'error' or verifiable == 'False':
                file_path = row.get('file_path', '')
                if file_path:
                    sample = FailingSample(
                        file_path=file_path,
                        original_loc=int(row.get('original_LOC', 0)),
                        status=status,
                    )
                    failing.append(sample)
    
    return failing


def repair_sample(sample: FailingSample, model: str = "claude") -> RepairResult:
    """
    Attempt to repair a single failing sample.
    
    Returns RepairResult with success status and details.
    """
    start_time = time.time()
    
    # Load file content
    filepath = MINIMIZED_DIR / sample.file_path
    if not filepath.exists():
        return RepairResult(
            file_path=sample.file_path,
            success=False,
            attempts=0,
            original_error="File not found",
            fixed_code=None,
            final_error="File not found",
            duration_seconds=0
        )
    
    original_code = filepath.read_text()
    
    # Get initial error
    success, error_output = run_verus(filepath)
    if success:
        # Already verifies - shouldn't be in failing list
        return RepairResult(
            file_path=sample.file_path,
            success=True,
            attempts=0,
            original_error="",
            fixed_code=original_code,
            final_error=None,
            duration_seconds=time.time() - start_time
        )
    
    original_error = extract_error_context(error_output)
    current_code = original_code
    current_error = original_error
    
    # Select repair function
    repair_fn = repair_with_claude if model == "claude" else repair_with_openai
    
    # Iterative repair attempts
    for attempt in range(1, MAX_REPAIR_ATTEMPTS + 1):
        print(f"    Attempt {attempt}/{MAX_REPAIR_ATTEMPTS}...", end=" ")
        
        # Get LLM fix suggestion
        fixed_code = repair_fn(current_code, current_error, attempt)
        
        if not fixed_code:
            print("LLM failed")
            continue
        
        # Write to temp file and verify
        temp_file = SCRIPT_DIR / f"temp_repair_{os.getpid()}.rs"
        try:
            temp_file.write_text(fixed_code)
            success, verify_output = run_verus(temp_file)
            
            if success:
                print("✓ VERIFIED!")
                return RepairResult(
                    file_path=sample.file_path,
                    success=True,
                    attempts=attempt,
                    original_error=original_error,
                    fixed_code=fixed_code,
                    final_error=None,
                    duration_seconds=time.time() - start_time
                )
            else:
                current_code = fixed_code
                current_error = extract_error_context(verify_output)
                print(f"still failing")
                
        finally:
            if temp_file.exists():
                temp_file.unlink()
    
    # All attempts failed
    return RepairResult(
        file_path=sample.file_path,
        success=False,
        attempts=MAX_REPAIR_ATTEMPTS,
        original_error=original_error,
        fixed_code=None,
        final_error=current_error,
        duration_seconds=time.time() - start_time
    )


def save_repair_results(results: List[RepairResult]):
    """Save successful repairs and log all results."""
    # Create repaired directory
    REPAIRED_DIR.mkdir(exist_ok=True)
    
    # Save successful repairs
    successful = [r for r in results if r.success and r.fixed_code]
    for result in successful:
        output_path = REPAIRED_DIR / result.file_path
        output_path.write_text(result.fixed_code)
    
    # Save log
    log_data = {
        "timestamp": datetime.now().isoformat(),
        "total_processed": len(results),
        "successful": len(successful),
        "failed": len(results) - len(successful),
        "results": [
            {
                "file": r.file_path,
                "success": r.success,
                "attempts": r.attempts,
                "duration_seconds": r.duration_seconds,
                "original_error": r.original_error[:500] if r.original_error else None,
                "final_error": r.final_error[:500] if r.final_error else None,
            }
            for r in results
        ]
    }
    
    with open(REPAIR_LOG_FILE, 'w') as f:
        json.dump(log_data, f, indent=2)
    
    print(f"\nResults saved to: {REPAIRED_DIR}/")
    print(f"Log saved to: {REPAIR_LOG_FILE}")


def update_metadata_csv(results: List[RepairResult]):
    """Update the metadata CSV with repaired samples."""
    successful = {r.file_path for r in results if r.success}
    
    if not successful:
        return
    
    # Read existing metadata
    rows = []
    with open(METADATA_FILE, 'r') as f:
        reader = csv.DictReader(f)
        fieldnames = reader.fieldnames
        for row in reader:
            if row['file_path'] in successful:
                row['status'] = 'verified'
                row['verifiable'] = 'True'
            rows.append(row)
    
    # Write updated metadata
    with open(METADATA_FILE, 'w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)
    
    print(f"Updated {len(successful)} entries in {METADATA_FILE}")


# ============================================================================
# Main Entry Point
# ============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="LLM-assisted repair for failing Verus samples"
    )
    parser.add_argument(
        "--batch", type=int, default=None,
        help="Number of samples to process (default: all)"
    )
    parser.add_argument(
        "--model", choices=["claude", "openai"], default="claude",
        help="LLM model to use (default: claude)"
    )
    parser.add_argument(
        "--dry-run", action="store_true",
        help="Show what would be repaired without actually repairing"
    )
    parser.add_argument(
        "--continue-from", type=str, default=None,
        help="Continue from a specific file (skip earlier files)"
    )
    
    args = parser.parse_args()
    
    # Check Verus availability
    if not Path(VERUS_PATH).exists():
        print(f"Error: Verus not found at {VERUS_PATH}")
        print("Set VERUS_PATH environment variable or build Verus")
        sys.exit(1)
    
    print("=" * 70)
    print("LLM-Assisted Verus Repair")
    print("=" * 70)
    print(f"Model: {args.model}")
    print(f"Verus: {VERUS_PATH}")
    print(f"Source: {MINIMIZED_DIR}/")
    print(f"Output: {REPAIRED_DIR}/")
    print("=" * 70)
    
    # Load failing samples
    failing_samples = load_failing_samples()
    print(f"\nFound {len(failing_samples)} failing samples")
    
    if not failing_samples:
        print("No failing samples to repair!")
        return
    
    # Filter samples
    if args.continue_from:
        idx = next((i for i, s in enumerate(failing_samples) 
                   if s.file_path == args.continue_from), 0)
        failing_samples = failing_samples[idx:]
        print(f"Continuing from {args.continue_from} ({len(failing_samples)} remaining)")
    
    if args.batch:
        failing_samples = failing_samples[:args.batch]
        print(f"Processing batch of {len(failing_samples)} samples")
    
    # Dry run - just show what would be processed
    if args.dry_run:
        print("\nDry run - would process:")
        for i, sample in enumerate(failing_samples[:20], 1):
            print(f"  {i}. {sample.file_path} (status: {sample.status})")
        if len(failing_samples) > 20:
            print(f"  ... and {len(failing_samples) - 20} more")
        return
    
    # Process samples
    print(f"\nProcessing {len(failing_samples)} samples...\n")
    
    results: List[RepairResult] = []
    start_time = time.time()
    
    for i, sample in enumerate(failing_samples, 1):
        print(f"[{i}/{len(failing_samples)}] {sample.file_path}")
        result = repair_sample(sample, model=args.model)
        results.append(result)
        
        status = "✓ REPAIRED" if result.success else "✗ FAILED"
        print(f"  {status} ({result.attempts} attempts, {result.duration_seconds:.1f}s)")
        print()
    
    # Summary
    elapsed = time.time() - start_time
    successful = sum(1 for r in results if r.success)
    
    print("=" * 70)
    print("Repair Complete!")
    print("=" * 70)
    print(f"Total processed: {len(results)}")
    print(f"Successfully repaired: {successful} ({100*successful/len(results):.1f}%)")
    print(f"Failed: {len(results) - successful}")
    print(f"Total time: {elapsed/60:.1f} minutes")
    print("=" * 70)
    
    # Save results
    if successful > 0:
        save_repair_results(results)
        
        # Ask to update metadata
        response = input("\nUpdate metadata CSV with repaired samples? [y/N] ")
        if response.lower() == 'y':
            update_metadata_csv(results)


if __name__ == "__main__":
    main()

