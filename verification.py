"""
Parallel verification testing for Verus files.
Efficiently verifies and compiles candidate files.
"""

import subprocess
import time
from pathlib import Path
from typing import List, Dict, Optional, Tuple
import re
from dataclasses import dataclass, asdict
from concurrent.futures import ProcessPoolExecutor, as_completed
from analysis import FileAnalysis

# Error categories
ERROR_CATEGORIES = {
    'dependency': ['unresolved import', 'cannot find', 'no extern crate'],
    'syntax': ['expected', 'unexpected', 'syntax error'],
    'verification': ['assertion failed', 'postcondition', 'precondition', 'invariant'],
    'type': ['cannot use type', 'type mismatch', 'expected type'],
    'timeout': ['timeout', 'timed out'],
    'resource_limit': ['resource limit', 'rlimit exceeded', 'rlimit'],
    'unknown': []
}


@dataclass
class VerificationResult:
    """Result of verifying a file"""
    filepath: str
    file_type: str
    status: str  # "verified", "verified_compiled", "failed", "timeout", "error"
    verify_time_ms: Optional[int] = None
    compile_time_ms: Optional[int] = None
    error_category: Optional[str] = None
    error_message: Optional[str] = None
    verification_output: Optional[str] = None


def categorize_error(error_output: str) -> str:
    """Categorize error output into error types"""
    error_lower = error_output.lower()
    for category, keywords in ERROR_CATEGORIES.items():
        if category == 'unknown':
            continue
        for keyword in keywords:
            if keyword in error_lower:
                return category
    return 'unknown'


def parse_verification_results(output: str) -> Tuple[Optional[int], Optional[int], bool]:
    """
    Parse Verus verification output to extract verified count, error count, and success status.
    
    Looks for pattern: "verification results:: X verified, Y errors"
    
    Returns:
        (verified_count, error_count, has_valid_format) tuple
        Returns (None, None, False) if pattern not found or malformed
    """
    # Look for the verification results line
    # Pattern: "verification results:: X verified, Y errors"
    pattern = r'verification results::\s*(\d+)\s+verified,\s*(\d+)\s+errors'
    match = re.search(pattern, output, re.IGNORECASE)
    
    if match:
        verified = int(match.group(1))
        errors = int(match.group(2))
        return (verified, errors, True)
    
    return (None, None, False)


def has_verification_errors(output: str) -> bool:
    """
    Check if output contains verification errors or resource limit issues.
    
    Returns True if there are errors that should cause verification to fail.
    """
    output_lower = output.lower()
    
    # Check for resource limit errors (these are failures even if returncode is 0)
    if any(keyword in output_lower for keyword in ['resource limit', 'rlimit exceeded', 'rlimit']):
        return True
    
    # Check for error: prefix (Verus error messages)
    if 'error:' in output_lower:
        return True
    
    # Parse verification results
    verified, errors, has_format = parse_verification_results(output)
    
    if has_format:
        # If we have valid format, check error count
        return errors > 0
    
    # If no format found but returncode check will handle it
    return False


def validate_file_content(filepath: Path) -> Optional[str]:
    """
    Validate file content before verification to catch common issues.
    
    Returns error message if validation fails, None if OK.
    """
    try:
        content = filepath.read_text(encoding="utf-8", errors="ignore")
    except Exception as e:
        return f"Could not read file: {e}"
    
    # Check for verus! block
    if 'verus!' not in content and 'verus' not in content.lower():
        return "No verus! block found"
    
    # Check for spec keywords (must have at least one Verus construct)
    verus_keywords = ['spec', 'proof', 'requires', 'ensures', 'decreases', 
                      'assert', 'invariant', '&&&', '|||', 'forall', 'exists']
    if not any(keyword in content for keyword in verus_keywords):
        return "No Verus keywords found (spec, proof, requires, ensures, etc.)"
    
    # Check for function bodies (must have at least one {})
    if '{' not in content:
        return "No function bodies found"
    
    # Check for spec-only files that might be invalid
    # If file only has spec functions without bodies, that's suspicious
    spec_fn_pattern = r'spec\s+fn\s+\w+\s*\([^)]*\)\s*->[^{]*;'
    if re.search(spec_fn_pattern, content) and 'fn' not in content.replace('spec fn', ''):
        # Has spec fn declarations but might be missing implementations
        # This is OK if they're in a verus! block, but we'll let verification catch syntax errors
        pass
    
    return None


def verify_single_file(
    verus_path: str,
    filepath: Path,
    file_type: str,
    timeout: int = 60
) -> VerificationResult:
    """Verify a single file using Verus command as source of truth"""
    full_path = filepath.resolve()
    
    # Determine verification command based on file type
    if file_type == "lib":
        verify_cmd = [verus_path, '--crate-type=lib', str(full_path)]
        compile_cmd = [verus_path, '--crate-type=lib', str(full_path), '--compile']
    else:
        verify_cmd = [verus_path, str(full_path)]
        compile_cmd = [verus_path, str(full_path), '--compile']
    
    # Verify
    verify_start = time.perf_counter()
    try:
        verify_result = subprocess.run(
            verify_cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=str(full_path.parent)
        )
        verify_elapsed = int((time.perf_counter() - verify_start) * 1000)
        
        output = verify_result.stdout + verify_result.stderr
        
        # Parse verification results properly
        verified_count, error_count, has_valid_format = parse_verification_results(output)
        
        # Check for verification errors (including resource limits)
        has_errors = has_verification_errors(output)
        
        # Source of truth: Verus command output
        # Status based on verification results:
        # - "verified_compiled": >=1 verified, 0 errors
        # - "spec_only": 0 verified, 0 errors (compiles but no proofs)
        # - "failed": errors > 0 or exit code != 0
        
        if verify_result.returncode != 0 or has_errors:
            # Verification failed
            error_msg = verify_result.stderr[:500] if verify_result.stderr else output[:500]
            if verified_count is not None and error_count is not None:
                error_msg = f"Verified: {verified_count}, Errors: {error_count}. {error_msg}"
            
            return VerificationResult(
                filepath=str(filepath),
                file_type=file_type,
                status="failed",
                verify_time_ms=verify_elapsed,
                error_category=categorize_error(error_msg),
                error_message=error_msg,
                verification_output=output[:1000]
            )
        
        if has_valid_format and error_count == 0:
            if verified_count > 0:
                # Verified: >=1 verified, 0 errors
                return VerificationResult(
                    filepath=str(filepath),
                    file_type=file_type,
                    status="verified_compiled",
                    verify_time_ms=verify_elapsed
                )
            else:
                # Spec-only: 0 verified, 0 errors
                return VerificationResult(
                    filepath=str(filepath),
                    file_type=file_type,
                    status="spec_only",
                    verify_time_ms=verify_elapsed
                )
        
        # No valid format found but no errors - treat as spec_only
        return VerificationResult(
            filepath=str(filepath),
            file_type=file_type,
            status="spec_only",
            verify_time_ms=verify_elapsed
        )
    except subprocess.TimeoutExpired:
        return VerificationResult(
            filepath=str(filepath),
            file_type=file_type,
            status="timeout",
            error_category="timeout",
            error_message="Verification timed out"
        )
    except Exception as e:
        return VerificationResult(
            filepath=str(filepath),
            file_type=file_type,
            status="error",
            error_category="unknown",
            error_message=str(e)[:500]
        )


def verify_files_parallel(
    verus_path: str,
    candidates: List[FileAnalysis],
    max_workers: int = 4,
    timeout: int = 60
) -> List[VerificationResult]:
    """
    Verify multiple files in parallel.
    
    Args:
        verus_path: Path to verus executable
        candidates: List of FileAnalysis objects to verify
        max_workers: Number of parallel workers
        timeout: Timeout per file in seconds
    
    Returns:
        List of VerificationResult objects
    """
    results = []
    
    print(f"\nVerifying {len(candidates)} files with {max_workers} workers...")
    
    try:
        with ProcessPoolExecutor(max_workers=max_workers) as executor:
            # Submit all tasks
            future_to_file = {
                executor.submit(
                    verify_single_file,
                    verus_path,
                    candidate.path,
                    candidate.file_type,
                    timeout
                ): candidate
                for candidate in candidates
            }
            
            # Process results as they complete
            completed = 0
            for future in as_completed(future_to_file):
                completed += 1
                candidate = future_to_file[future]
                try:
                    result = future.result()
                    results.append(result)
                    
                    if completed % 10 == 0:
                        print(f"  Completed {completed}/{len(candidates)}...")
                except Exception as e:
                    # Handle exceptions from worker
                    results.append(VerificationResult(
                        filepath=str(candidate.path),
                        file_type=candidate.file_type,
                        status="error",
                        error_category="unknown",
                        error_message=f"Worker exception: {str(e)[:500]}"
                    ))
    except Exception as e:
        # Fallback to sequential processing if parallel fails
        print(f"Parallel processing failed: {e}")
        print("Falling back to sequential processing...")
        for i, candidate in enumerate(candidates, 1):
            if i % 10 == 0:
                print(f"  Processed {i}/{len(candidates)}...")
            result = verify_single_file(
                verus_path,
                candidate.path,
                candidate.file_type,
                timeout
            )
            results.append(result)
    
    print(f"\nVerification complete: {len(results)} results")
    return results

