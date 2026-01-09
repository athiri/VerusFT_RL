"""
Verify Dataset Samples and Create Training Tasks

This script:
1. Verifies that samples in the dataset are standalone verifiable with Verus
2. Creates the 3 training task datasets (A, B, C) in instruction-tuning format

Tasks:
- Task A: Code → Specifications (generate requires/ensures from code)
- Task B: Specifications → Verified Code (generate code from specs)
- Task C: Error-Guided Repair (fix broken code)
"""

import json
import os
import subprocess
import tempfile
import random
import argparse
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from concurrent.futures import ThreadPoolExecutor, as_completed


# Paths
DATASET_PATH = Path("workspace/min_dataset/dataset.jsonl")
SPLITS_PATH = Path("workspace/min_dataset/splits")
OUTPUT_PATH = Path("workspace/training_tasks")
VERUS_PATH = os.environ.get("VERUS_PATH", "/Users/chuyues/verus/verus")


@dataclass
class VerificationResult:
    """Result of Verus verification."""
    sample_id: str
    success: bool
    error: Optional[str] = None
    time_seconds: float = 0.0


def verify_code(code: str, sample_id: str, timeout: int = 60) -> VerificationResult:
    """
    Verify a code sample with Verus.
    
    Args:
        code: Full Verus code to verify
        sample_id: Sample identifier for logging
        timeout: Timeout in seconds
        
    Returns:
        VerificationResult with success status and any errors
    """
    import time
    start = time.time()
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.rs', delete=False) as f:
        f.write(code)
        temp_path = f.name
    
    try:
        result = subprocess.run(
            [VERUS_PATH, "--crate-type=lib", temp_path],
            capture_output=True,
            text=True,
            timeout=timeout
        )
        elapsed = time.time() - start
        
        if result.returncode == 0:
            return VerificationResult(sample_id, True, None, elapsed)
        else:
            # Extract error message
            error_msg = result.stderr[:500] if result.stderr else result.stdout[:500]
            return VerificationResult(sample_id, False, error_msg, elapsed)
            
    except subprocess.TimeoutExpired:
        return VerificationResult(sample_id, False, f"Timeout after {timeout}s", timeout)
    except Exception as e:
        return VerificationResult(sample_id, False, str(e), time.time() - start)
    finally:
        os.unlink(temp_path)


def load_dataset(path: Path, limit: Optional[int] = None) -> List[Dict]:
    """Load JSONL dataset."""
    samples = []
    with open(path, 'r') as f:
        for i, line in enumerate(f):
            if limit and i >= limit:
                break
            samples.append(json.loads(line))
    return samples


def verify_dataset_samples(
    samples: List[Dict],
    num_samples: int = 100,
    num_workers: int = 4,
    random_seed: int = 42
) -> Tuple[List[VerificationResult], float]:
    """
    Verify a random sample of dataset entries.
    
    Args:
        samples: List of dataset samples
        num_samples: Number of samples to verify
        num_workers: Number of parallel workers
        random_seed: Random seed for reproducibility
        
    Returns:
        Tuple of (results list, pass rate)
    """
    random.seed(random_seed)
    
    # Sample randomly
    if num_samples < len(samples):
        test_samples = random.sample(samples, num_samples)
    else:
        test_samples = samples
    
    print(f"\nVerifying {len(test_samples)} samples with Verus...")
    print(f"Using {num_workers} parallel workers")
    print("-" * 60)
    
    results = []
    
    with ThreadPoolExecutor(max_workers=num_workers) as executor:
        futures = {
            executor.submit(
                verify_code, 
                sample['full_verified_code'], 
                sample['id']
            ): sample 
            for sample in test_samples
        }
        
        for i, future in enumerate(as_completed(futures)):
            result = future.result()
            results.append(result)
            
            status = "✓" if result.success else f"✗ ({result.error[:50]}...)" if result.error else "✗"
            print(f"  [{i+1}/{len(test_samples)}] {result.sample_id}: {status} ({result.time_seconds:.1f}s)")
    
    # Compute pass rate
    passed = sum(1 for r in results if r.success)
    pass_rate = passed / len(results) if results else 0.0
    
    print("-" * 60)
    print(f"Pass rate: {passed}/{len(results)} ({pass_rate:.1%})")
    
    return results, pass_rate


# ============================================================================
# Task Formatting Functions
# ============================================================================

def format_task_a_prompt(sample: Dict) -> str:
    """
    Format Task A prompt: Code → Specifications
    
    Input: Code without specs (input_text)
    Output: The specifications (target_text)
    """
    instruction = """You are a Verus formal verification expert. Given Verus code, add the appropriate specifications (requires, ensures, decreases, invariants).

### Code (without specifications):
```verus
{input}
```

### Specifications to add:"""
    
    return instruction.format(input=sample['input_text'].strip())


def format_task_a_completion(sample: Dict) -> str:
    """Format Task A completion."""
    return sample['target_text'].strip()


def format_task_b_prompt(sample: Dict) -> str:
    """
    Format Task B prompt: Specifications → Verified Code
    
    Input: Function signature with specs (input_text)
    Output: Full verified implementation (target_text)
    """
    instruction = """You are a Verus formal verification expert. Given a function signature with specifications, write the complete implementation that satisfies all requirements.

### Function signature with specifications:
```verus
{input}
```

### Complete implementation:"""
    
    return instruction.format(input=sample['input_text'].strip())


def format_task_b_completion(sample: Dict) -> str:
    """Format Task B completion."""
    return f"```verus\n{sample['target_text'].strip()}\n```"


def format_task_c_prompt(sample: Dict) -> str:
    """
    Format Task C prompt: Error-Guided Repair
    
    Input: Broken code (input_text) - missing ensures, requires, etc.
    Output: Fixed code (target_text)
    """
    bug_type = sample.get('metadata', {}).get('bug_type', 'unknown')
    
    bug_descriptions = {
        'missing_ensures': 'missing postcondition (ensures clause)',
        'missing_requires': 'missing precondition (requires clause)',
        'missing_decreases': 'missing termination measure (decreases clause)',
        'missing_invariant': 'missing loop invariant',
        'missing_assert': 'missing assertion for proof',
    }
    
    bug_desc = bug_descriptions.get(bug_type, 'verification error')
    
    instruction = """You are a Verus formal verification expert. The following code has a verification error: {bug_desc}. Fix the code so it verifies.

### Broken code:
```verus
{input}
```

### Fixed code:"""
    
    return instruction.format(bug_desc=bug_desc, input=sample['input_text'].strip())


def format_task_c_completion(sample: Dict) -> str:
    """Format Task C completion."""
    return f"```verus\n{sample['target_text'].strip()}\n```"


def create_sft_example(prompt: str, completion: str) -> Dict:
    """Create an SFT training example in text format."""
    return {
        "text": f"{prompt}\n{completion}"
    }


def create_training_tasks(
    samples: List[Dict],
    output_dir: Path,
    include_metadata: bool = False
) -> Dict[str, int]:
    """
    Create training task files from verified samples.
    
    Args:
        samples: List of verified dataset samples
        output_dir: Directory to save training files
        include_metadata: Whether to include metadata in output
        
    Returns:
        Dictionary with counts per task
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Group samples by task
    task_samples = {'task_a': [], 'task_b': [], 'task_c': []}
    
    for sample in samples:
        task = sample.get('task', '')
        if task in task_samples:
            task_samples[task].append(sample)
    
    counts = {}
    
    # Process Task A: Code → Specs
    print("\nCreating Task A (Code → Specifications)...")
    task_a_examples = []
    for sample in task_samples['task_a']:
        prompt = format_task_a_prompt(sample)
        completion = format_task_a_completion(sample)
        example = create_sft_example(prompt, completion)
        if include_metadata:
            example['metadata'] = {
                'id': sample['id'],
                'source': sample.get('source'),
                'function_name': sample.get('metadata', {}).get('function_name')
            }
        task_a_examples.append(example)
    
    with open(output_dir / "task_a_sft.jsonl", 'w') as f:
        for ex in task_a_examples:
            f.write(json.dumps(ex) + '\n')
    counts['task_a'] = len(task_a_examples)
    print(f"  Created {len(task_a_examples)} examples → task_a_sft.jsonl")
    
    # Process Task B: Specs → Code
    print("\nCreating Task B (Specifications → Code)...")
    task_b_examples = []
    for sample in task_samples['task_b']:
        prompt = format_task_b_prompt(sample)
        completion = format_task_b_completion(sample)
        example = create_sft_example(prompt, completion)
        if include_metadata:
            example['metadata'] = {
                'id': sample['id'],
                'source': sample.get('source'),
                'function_name': sample.get('metadata', {}).get('function_name')
            }
        task_b_examples.append(example)
    
    with open(output_dir / "task_b_sft.jsonl", 'w') as f:
        for ex in task_b_examples:
            f.write(json.dumps(ex) + '\n')
    counts['task_b'] = len(task_b_examples)
    print(f"  Created {len(task_b_examples)} examples → task_b_sft.jsonl")
    
    # Process Task C: Repair
    print("\nCreating Task C (Error-Guided Repair)...")
    task_c_examples = []
    for sample in task_samples['task_c']:
        prompt = format_task_c_prompt(sample)
        completion = format_task_c_completion(sample)
        example = create_sft_example(prompt, completion)
        if include_metadata:
            example['metadata'] = {
                'id': sample['id'],
                'source': sample.get('source'),
                'function_name': sample.get('metadata', {}).get('function_name'),
                'bug_type': sample.get('metadata', {}).get('bug_type')
            }
        task_c_examples.append(example)
    
    with open(output_dir / "task_c_sft.jsonl", 'w') as f:
        for ex in task_c_examples:
            f.write(json.dumps(ex) + '\n')
    counts['task_c'] = len(task_c_examples)
    print(f"  Created {len(task_c_examples)} examples → task_c_sft.jsonl")
    
    # Create combined multi-task dataset
    print("\nCreating combined multi-task dataset...")
    all_examples = task_a_examples + task_b_examples + task_c_examples
    random.shuffle(all_examples)
    
    with open(output_dir / "multi_task_sft.jsonl", 'w') as f:
        for ex in all_examples:
            f.write(json.dumps(ex) + '\n')
    counts['multi_task'] = len(all_examples)
    print(f"  Created {len(all_examples)} examples → multi_task_sft.jsonl")
    
    return counts


def main():
    parser = argparse.ArgumentParser(
        description="Verify dataset and create training tasks"
    )
    parser.add_argument(
        "--verify-samples",
        type=int,
        default=20,
        help="Number of samples to verify with Verus (0 to skip verification)"
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=4,
        help="Number of parallel verification workers"
    )
    parser.add_argument(
        "--create-tasks",
        action="store_true",
        default=True,
        help="Create training task files"
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="workspace/training_tasks",
        help="Output directory for training tasks"
    )
    parser.add_argument(
        "--include-metadata",
        action="store_true",
        help="Include metadata in output files"
    )
    
    args = parser.parse_args()
    
    print("=" * 60)
    print("Verus Dataset Verification and Task Creation")
    print("=" * 60)
    
    # Load dataset
    print(f"\nLoading dataset from {DATASET_PATH}...")
    samples = load_dataset(DATASET_PATH)
    print(f"Loaded {len(samples)} samples")
    
    # Show task distribution
    task_counts = {}
    for s in samples:
        task = s.get('task', 'unknown')
        task_counts[task] = task_counts.get(task, 0) + 1
    
    print("\nTask distribution:")
    for task, count in sorted(task_counts.items()):
        print(f"  {task}: {count}")
    
    # Verify samples
    if args.verify_samples > 0:
        results, pass_rate = verify_dataset_samples(
            samples,
            num_samples=args.verify_samples,
            num_workers=args.workers
        )
        
        if pass_rate < 1.0:
            print(f"\n⚠️  Warning: {(1-pass_rate)*100:.1f}% of samples failed verification")
            failed = [r for r in results if not r.success]
            print("Failed samples:")
            for r in failed[:5]:  # Show first 5
                print(f"  - {r.sample_id}: {r.error[:100]}...")
    
    # Create training tasks
    if args.create_tasks:
        print("\n" + "=" * 60)
        print("Creating Training Tasks")
        print("=" * 60)
        
        counts = create_training_tasks(
            samples,
            Path(args.output_dir),
            include_metadata=args.include_metadata
        )
        
        print("\n" + "=" * 60)
        print("Summary")
        print("=" * 60)
        print(f"\nTraining tasks created in: {args.output_dir}/")
        print("\nFiles created:")
        for task, count in counts.items():
            print(f"  {task}_sft.jsonl: {count} examples")
        
        print("\nTo train on a specific task:")
        print(f"  python train_task.py --task task_a --data {args.output_dir}/task_a_sft.jsonl")
        print("\nTo train on all tasks (multi-task):")
        print(f"  python train_task.py --task multi --data {args.output_dir}/multi_task_sft.jsonl")


if __name__ == "__main__":
    main()
