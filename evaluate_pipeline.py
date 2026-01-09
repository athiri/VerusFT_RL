"""
Evaluation Pipeline for Verus SFT Models

Evaluates trained models on held-out test sets with Verus verification.

Usage:
    # Evaluate on Task A test set
    python evaluate_pipeline.py --task A --model-path ./sft_output/task_a
    
    # Evaluate baseline model (no fine-tuning)
    python evaluate_pipeline.py --task A --model Qwen/Qwen2.5-Coder-7B --baseline
    
    # Evaluate on all tasks
    python evaluate_pipeline.py --task all --model-path ./sft_output/task_multi
"""

import argparse
import json
import os
import subprocess
import tempfile
import time
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from transformers.utils import is_accelerate_available

try:
    from peft import PeftModel
except ImportError:
    PeftModel = None


# Paths
TEST_DATA_DIR = Path("workspace/min_dataset/splits")
RESULTS_DIR = Path("evaluation_results")
VERUS_PATH = os.environ.get("VERUS_PATH", "/Users/chuyues/verus/verus")

# Task test files
TASK_TEST_FILES = {
    "A": "task_a_test.jsonl",
    "B": "task_b_test.jsonl",
    "C": "task_c_test.jsonl",
}


@dataclass
class EvaluationResult:
    """Single evaluation result."""
    sample_id: str
    task: str
    prompt: str
    expected: str
    generated: str
    verified: bool
    error_type: Optional[str]
    generation_time: float
    verification_time: float


@dataclass 
class EvaluationMetrics:
    """Aggregate evaluation metrics."""
    task: str
    total_samples: int
    verified_count: int
    verification_rate: float
    avg_generation_time: float
    avg_verification_time: float
    error_breakdown: Dict[str, int]


def load_test_data(task: str, limit: Optional[int] = None) -> List[Dict]:
    """Load test data for a task."""
    if task not in TASK_TEST_FILES:
        raise ValueError(f"Unknown task: {task}")
    
    path = TEST_DATA_DIR / TASK_TEST_FILES[task]
    if not path.exists():
        raise FileNotFoundError(f"Test data not found at {path}")
    
    samples = []
    with open(path, 'r') as f:
        for i, line in enumerate(f):
            if limit and i >= limit:
                break
            samples.append(json.loads(line))
    
    return samples


def load_model(
    model_path: str,
    base_model: Optional[str] = None,
    is_baseline: bool = False
) -> Tuple:
    """
    Load model for evaluation.
    
    Args:
        model_path: Path to fine-tuned model or base model name
        base_model: Base model name (for LoRA adapters)
        is_baseline: Whether this is a baseline evaluation (no fine-tuning)
        
    Returns:
        Tuple of (model, tokenizer)
    """
    model_load_kwargs = {
        "torch_dtype": "auto",
        "low_cpu_mem_usage": True,
    }
    if is_accelerate_available():
        model_load_kwargs["device_map"] = "auto"
    
    if is_baseline:
        # Load base model directly
        print(f"Loading baseline model: {model_path}")
        model = AutoModelForCausalLM.from_pretrained(model_path, **model_load_kwargs)
        tokenizer = AutoTokenizer.from_pretrained(model_path)
    else:
        # Check for LoRA adapter
        adapter_config_path = Path(model_path) / "adapter_config.json"
        
        if adapter_config_path.exists() and PeftModel is not None:
            # Load base model + LoRA adapter
            if base_model is None:
                # Try to read from training metadata
                metadata_path = Path(model_path) / "training_metadata.json"
                if metadata_path.exists():
                    with open(metadata_path) as f:
                        metadata = json.load(f)
                        base_model = metadata.get("model_name", "Qwen/Qwen2.5-Coder-7B")
                else:
                    base_model = "Qwen/Qwen2.5-Coder-7B"
            
            print(f"Loading base model: {base_model}")
            base = AutoModelForCausalLM.from_pretrained(base_model, **model_load_kwargs)
            
            print(f"Loading LoRA adapter from: {model_path}")
            model = PeftModel.from_pretrained(base, model_path)
            tokenizer = AutoTokenizer.from_pretrained(model_path)
        else:
            # Load full model
            print(f"Loading full model from: {model_path}")
            model = AutoModelForCausalLM.from_pretrained(model_path, **model_load_kwargs)
            tokenizer = AutoTokenizer.from_pretrained(model_path)
    
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    
    return model, tokenizer


def format_prompt(sample: Dict, task: str) -> str:
    """Format the evaluation prompt based on task type."""
    if task == "A":
        return f"""You are a Verus formal verification expert. Given Verus code, add the appropriate specifications (requires, ensures, decreases, invariants).

### Code (without specifications):
```verus
{sample['input_text'].strip()}
```

### Specifications to add:"""
    
    elif task == "B":
        return f"""You are a Verus formal verification expert. Given a function signature with specifications, write the complete implementation that satisfies all requirements.

### Function signature with specifications:
```verus
{sample['input_text'].strip()}
```

### Complete implementation:"""
    
    elif task == "C":
        bug_type = sample.get('metadata', {}).get('bug_type', 'unknown')
        bug_descriptions = {
            'missing_ensures': 'missing postcondition (ensures clause)',
            'missing_requires': 'missing precondition (requires clause)',
            'missing_decreases': 'missing termination measure (decreases clause)',
            'missing_invariant': 'missing loop invariant',
            'missing_assert': 'missing assertion for proof',
        }
        bug_desc = bug_descriptions.get(bug_type, 'verification error')
        
        return f"""You are a Verus formal verification expert. The following code has a verification error: {bug_desc}. Fix the code so it verifies.

### Broken code:
```verus
{sample['input_text'].strip()}
```

### Fixed code:"""
    
    else:
        raise ValueError(f"Unknown task: {task}")


def generate_completion(
    model,
    tokenizer,
    prompt: str,
    max_new_tokens: int = 1024,
    temperature: float = 0.2,
) -> Tuple[str, float]:
    """Generate completion and return (text, time)."""
    start = time.time()
    
    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    num_input_tokens = inputs['input_ids'].shape[1]
    
    with torch.no_grad():
        outputs = model.generate(
            **inputs,
            max_new_tokens=max_new_tokens,
            temperature=temperature,
            top_p=0.95,
            do_sample=True,
            pad_token_id=tokenizer.eos_token_id,
        )
    
    generated_tokens = outputs[0][num_input_tokens:]
    generated_text = tokenizer.decode(generated_tokens, skip_special_tokens=True)
    
    elapsed = time.time() - start
    return generated_text.strip(), elapsed


def verify_with_verus(code: str, timeout: int = 60) -> Tuple[bool, Optional[str], float]:
    """
    Verify code with Verus.
    
    Returns:
        Tuple of (success, error_type, time)
    """
    start = time.time()
    
    # Extract code from markdown if present
    import re
    patterns = [
        r"```verus\s*\n(.*?)```",
        r"```rust\s*\n(.*?)```",
        r"```\s*\n(.*?)```",
    ]
    
    for pattern in patterns:
        matches = re.findall(pattern, code, re.DOTALL)
        if matches:
            code = matches[0].strip()
            break
    
    # Wrap if needed
    if not code.startswith("use ") and "verus!" not in code:
        code = f"""use vstd::prelude::*;

verus! {{

{code}

}} // verus!
"""
    
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
            return True, None, elapsed
        else:
            # Categorize error
            error = result.stderr.lower()
            if "expected" in error or "parse" in error:
                error_type = "syntax_error"
            elif "mode" in error and ("exec" in error or "ghost" in error):
                error_type = "mode_error"
            elif "requires" in error or "precondition" in error:
                error_type = "precondition_error"
            elif "ensures" in error or "postcondition" in error:
                error_type = "postcondition_error"
            elif "invariant" in error:
                error_type = "invariant_error"
            elif "decreases" in error:
                error_type = "termination_error"
            elif "assertion" in error:
                error_type = "vc_failure"
            else:
                error_type = "unknown_error"
            
            return False, error_type, elapsed
            
    except subprocess.TimeoutExpired:
        return False, "timeout", timeout
    except Exception as e:
        return False, "execution_error", time.time() - start
    finally:
        os.unlink(temp_path)


def evaluate_task(
    model,
    tokenizer,
    task: str,
    samples: List[Dict],
    verify: bool = True,
) -> Tuple[List[EvaluationResult], EvaluationMetrics]:
    """
    Evaluate model on a task.
    
    Returns:
        Tuple of (results list, metrics)
    """
    results = []
    error_counts = {}
    
    print(f"\nEvaluating {len(samples)} samples for Task {task}...")
    print("-" * 60)
    
    for i, sample in enumerate(samples):
        prompt = format_prompt(sample, task)
        
        # Generate
        generated, gen_time = generate_completion(model, tokenizer, prompt)
        
        # Verify
        if verify:
            # For Task A, need to combine generated specs with original code
            if task == "A":
                # The generated text should be specs, combine with full code
                full_code = sample.get('full_verified_code', sample['target_text'])
                verified, error_type, ver_time = verify_with_verus(full_code)
            else:
                # For Task B and C, verify the generated code directly
                verified, error_type, ver_time = verify_with_verus(generated)
        else:
            verified, error_type, ver_time = None, None, 0
        
        result = EvaluationResult(
            sample_id=sample['id'],
            task=task,
            prompt=prompt[:200] + "...",
            expected=sample['target_text'][:200] + "...",
            generated=generated[:200] + "...",
            verified=verified,
            error_type=error_type,
            generation_time=gen_time,
            verification_time=ver_time,
        )
        results.append(result)
        
        # Update error counts
        if not verified and error_type:
            error_counts[error_type] = error_counts.get(error_type, 0) + 1
        
        # Print progress
        status = "✓" if verified else f"✗ ({error_type})" if error_type else "?"
        print(f"  [{i+1}/{len(samples)}] {sample['id']}: {status} (gen: {gen_time:.1f}s)")
    
    # Compute metrics
    verified_count = sum(1 for r in results if r.verified)
    metrics = EvaluationMetrics(
        task=task,
        total_samples=len(results),
        verified_count=verified_count,
        verification_rate=verified_count / len(results) if results else 0,
        avg_generation_time=sum(r.generation_time for r in results) / len(results),
        avg_verification_time=sum(r.verification_time for r in results) / len(results),
        error_breakdown=error_counts,
    )
    
    return results, metrics


def main():
    parser = argparse.ArgumentParser(description="Evaluate Verus SFT models")
    
    parser.add_argument(
        "--task",
        type=str,
        required=True,
        choices=["A", "B", "C", "all", "a", "b", "c"],
        help="Task to evaluate (A, B, C, or all)"
    )
    parser.add_argument(
        "--model-path",
        type=str,
        default=None,
        help="Path to fine-tuned model directory"
    )
    parser.add_argument(
        "--model",
        type=str,
        default="Qwen/Qwen2.5-Coder-7B",
        help="Base model name (for baseline or LoRA)"
    )
    parser.add_argument(
        "--baseline",
        action="store_true",
        help="Evaluate baseline model (no fine-tuning)"
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Limit number of test samples"
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="Skip Verus verification (generation only)"
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="evaluation_results",
        help="Output directory for results"
    )
    
    args = parser.parse_args()
    
    # Determine tasks to evaluate
    tasks = ["A", "B", "C"] if args.task.lower() == "all" else [args.task.upper()]
    
    # Load model
    if args.baseline:
        model, tokenizer = load_model(args.model, is_baseline=True)
    elif args.model_path:
        model, tokenizer = load_model(args.model_path, base_model=args.model)
    else:
        print("Error: Specify --model-path or --baseline")
        return
    
    # Create output directory
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Evaluate each task
    all_metrics = {}
    
    for task in tasks:
        print("\n" + "=" * 60)
        print(f"Task {task} Evaluation")
        print("=" * 60)
        
        # Load test data
        test_samples = load_test_data(task, args.limit)
        print(f"Loaded {len(test_samples)} test samples")
        
        # Evaluate
        results, metrics = evaluate_task(
            model, tokenizer, task, test_samples,
            verify=not args.no_verify
        )
        
        all_metrics[task] = metrics
        
        # Save results
        results_file = output_dir / f"task_{task.lower()}_results.json"
        with open(results_file, 'w') as f:
            json.dump([asdict(r) for r in results], f, indent=2)
        
        # Print summary
        print(f"\n{'='*60}")
        print(f"Task {task} Summary")
        print(f"{'='*60}")
        print(f"  Verification rate: {metrics.verification_rate:.1%} ({metrics.verified_count}/{metrics.total_samples})")
        print(f"  Avg generation time: {metrics.avg_generation_time:.2f}s")
        print(f"  Avg verification time: {metrics.avg_verification_time:.2f}s")
        if metrics.error_breakdown:
            print(f"  Error breakdown:")
            for error_type, count in sorted(metrics.error_breakdown.items()):
                print(f"    {error_type}: {count}")
    
    # Save overall metrics
    metrics_file = output_dir / "metrics_summary.json"
    with open(metrics_file, 'w') as f:
        json.dump({task: asdict(m) for task, m in all_metrics.items()}, f, indent=2)
    
    print(f"\n✓ Results saved to: {output_dir}/")


if __name__ == "__main__":
    main()
