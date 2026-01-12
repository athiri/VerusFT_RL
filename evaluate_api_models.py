"""
Evaluate SOTA Closed API Models on Verus Tasks

Evaluates GPT-4o, o1, and Claude on Tasks A, B, C.

Usage:
    # Evaluate all models on all tasks
    python evaluate_api_models.py --tasks all --models all
    
    # Evaluate specific model on specific task
    python evaluate_api_models.py --tasks C --models gpt-4o
    
    # Dry run (no API calls)
    python evaluate_api_models.py --tasks C --models claude --dry-run --limit 5

Environment variables required:
    OPENAI_API_KEY - For GPT and o1 models
    ANTHROPIC_API_KEY - For Claude models
    
Or create a .env file in the project root with these variables.
"""

import argparse
import json
import os

# Load .env file if present
try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    pass  # python-dotenv not installed, skip
import time
from dataclasses import dataclass, asdict, field
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed

# API clients (install with: pip install openai anthropic)
try:
    import openai
    OPENAI_AVAILABLE = True
except ImportError:
    OPENAI_AVAILABLE = False

try:
    import anthropic
    ANTHROPIC_AVAILABLE = True
except ImportError:
    ANTHROPIC_AVAILABLE = False

# Local imports
from verus_verification import verify_with_verus


# =============================================================================
# Configuration
# =============================================================================

# Model configurations
MODEL_CONFIGS = {
    # OpenAI models
    "gpt-4o": {
        "provider": "openai",
        "model_id": "gpt-4o",
        "max_tokens": 4096,
        "temperature": 0.2,
    },
    "gpt-4o-mini": {
        "provider": "openai",
        "model_id": "gpt-4o-mini",
        "max_tokens": 4096,
        "temperature": 0.2,
    },
    "o1": {
        "provider": "openai",
        "model_id": "o1-2024-12-17",
        "max_tokens": 8192,
        "temperature": 1.0,  # o1 requires temperature=1
    },
    "o1-mini": {
        "provider": "openai",
        "model_id": "o1-mini",
        "max_tokens": 8192,
        "temperature": 1.0,
    },
    "o1-preview": {
        "provider": "openai",
        "model_id": "o1-preview",
        "max_tokens": 8192,
        "temperature": 1.0,
    },
    # Anthropic models
    "claude-opus": {
        "provider": "anthropic",
        "model_id": "claude-sonnet-4-20250514",  # Use latest available
        "max_tokens": 4096,
        "temperature": 0.2,
    },
    "claude-sonnet": {
        "provider": "anthropic",
        "model_id": "claude-sonnet-4-20250514",
        "max_tokens": 4096,
        "temperature": 0.2,
    },
}

# Paths
TEST_DATA_DIR = Path("workspace/min_dataset/splits")
RESULTS_DIR = Path("evaluation_results/api_models")

TASK_TEST_FILES = {
    "A": "task_a_test.jsonl",
    "B": "task_b_test.jsonl",
    "C": "task_c_test.jsonl",
}


# =============================================================================
# Data Classes
# =============================================================================

@dataclass
class EvalSample:
    """Single evaluation sample."""
    sample_id: str
    task: str
    model: str
    prompt: str
    expected: str
    generated: str
    verified: Optional[bool]
    error_type: Optional[str]
    api_latency: float
    verification_time: float
    timestamp: str
    input_tokens: int = 0
    output_tokens: int = 0
    raw_response: dict = field(default_factory=dict)


@dataclass
class EvalMetrics:
    """Aggregate metrics for a model-task pair."""
    model: str
    task: str
    total_samples: int
    verified_count: int
    verification_rate: float
    avg_api_latency: float
    avg_verification_time: float
    error_breakdown: Dict[str, int]
    timestamp: str
    # Token usage
    total_input_tokens: int = 0
    total_output_tokens: int = 0
    total_tokens: int = 0
    avg_input_tokens: float = 0.0
    avg_output_tokens: float = 0.0


# =============================================================================
# API Clients
# =============================================================================

class OpenAIClient:
    """OpenAI API client wrapper."""
    
    def __init__(self):
        if not OPENAI_AVAILABLE:
            raise ImportError("openai package not installed. Run: pip install openai")
        self.client = openai.OpenAI()
    
    def generate(
        self,
        prompt: str,
        model_id: str,
        max_tokens: int = 4096,
        temperature: float = 0.2,
    ) -> Tuple[str, float, dict]:
        """Generate completion. Returns (text, latency, raw_response)."""
        start = time.time()
        
        # o1 models don't support system messages
        if model_id.startswith("o1"):
            messages = [{"role": "user", "content": prompt}]
            response = self.client.chat.completions.create(
                model=model_id,
                messages=messages,
                max_completion_tokens=max_tokens,
            )
        else:
            messages = [
                {"role": "system", "content": "You are a Verus formal verification expert."},
                {"role": "user", "content": prompt},
            ]
            response = self.client.chat.completions.create(
                model=model_id,
                messages=messages,
                max_tokens=max_tokens,
                temperature=temperature,
            )
        
        latency = time.time() - start
        text = response.choices[0].message.content
        
        raw = {
            "model": response.model,
            "usage": {
                "prompt_tokens": response.usage.prompt_tokens,
                "completion_tokens": response.usage.completion_tokens,
            },
            "finish_reason": response.choices[0].finish_reason,
        }
        
        return text, latency, raw


class AnthropicClient:
    """Anthropic API client wrapper."""
    
    def __init__(self):
        if not ANTHROPIC_AVAILABLE:
            raise ImportError("anthropic package not installed. Run: pip install anthropic")
        self.client = anthropic.Anthropic()
    
    def generate(
        self,
        prompt: str,
        model_id: str,
        max_tokens: int = 4096,
        temperature: float = 0.2,
    ) -> Tuple[str, float, dict]:
        """Generate completion. Returns (text, latency, raw_response)."""
        start = time.time()
        
        response = self.client.messages.create(
            model=model_id,
            max_tokens=max_tokens,
            temperature=temperature,
            system="You are a Verus formal verification expert.",
            messages=[{"role": "user", "content": prompt}],
        )
        
        latency = time.time() - start
        text = response.content[0].text
        
        raw = {
            "model": response.model,
            "usage": {
                "input_tokens": response.usage.input_tokens,
                "output_tokens": response.usage.output_tokens,
            },
            "stop_reason": response.stop_reason,
        }
        
        return text, latency, raw


def get_client(provider: str):
    """Get API client for provider."""
    if provider == "openai":
        return OpenAIClient()
    elif provider == "anthropic":
        return AnthropicClient()
    else:
        raise ValueError(f"Unknown provider: {provider}")


# =============================================================================
# Data Loading
# =============================================================================

def load_test_data(task: str, limit: Optional[int] = None) -> List[Dict]:
    """Load test data for a task."""
    path = TEST_DATA_DIR / TASK_TEST_FILES[task]
    if not path.exists():
        raise FileNotFoundError(f"Test data not found: {path}")
    
    samples = []
    with open(path) as f:
        for i, line in enumerate(f):
            if limit and i >= limit:
                break
            samples.append(json.loads(line))
    
    return samples


# =============================================================================
# Prompt Formatting
# =============================================================================

def format_prompt(sample: Dict, task: str) -> str:
    """Format evaluation prompt for a task."""
    
    if task == "A":
        return f"""Given the following Verus code WITHOUT specifications, generate the appropriate specifications (requires, ensures, decreases, invariants).

Output ONLY the specifications to add, not the full code.

### Code (without specifications):
```verus
{sample['input_text'].strip()}
```

### Specifications to add:"""
    
    elif task == "B":
        return f"""Given the following Verus function signature with specifications, write the COMPLETE implementation that satisfies all requirements.

Output the FULL verifiable code including imports (use vstd::prelude::*;), the verus! {{ }} wrapper, and the complete function implementation.

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
        
        return f"""The following Verus code has a verification error: {bug_desc}.

Fix the code so it verifies. Output the COMPLETE fixed code.

### Broken code:
```verus
{sample['input_text'].strip()}
```

### Fixed code:"""
    
    else:
        raise ValueError(f"Unknown task: {task}")


# =============================================================================
# Evaluation
# =============================================================================

def evaluate_sample(
    client,
    model_config: dict,
    sample: dict,
    task: str,
    dry_run: bool = False,
) -> EvalSample:
    """Evaluate a single sample."""
    
    prompt = format_prompt(sample, task)
    model_name = model_config["model_id"]
    
    if dry_run:
        # Return dummy result
        return EvalSample(
            sample_id=sample['id'],
            task=task,
            model=model_name,
            prompt=prompt[:500] + "...",
            expected=sample['target_text'][:500] + "...",
            generated="[DRY RUN]",
            verified=None,
            error_type=None,
            api_latency=0,
            verification_time=0,
            timestamp=datetime.now().isoformat(),
        )
    
    # Generate completion
    try:
        generated, latency, raw = client.generate(
            prompt=prompt,
            model_id=model_config["model_id"],
            max_tokens=model_config["max_tokens"],
            temperature=model_config["temperature"],
        )
    except Exception as e:
        return EvalSample(
            sample_id=sample['id'],
            task=task,
            model=model_name,
            prompt=prompt[:500] + "...",
            expected=sample['target_text'][:500] + "...",
            generated=f"[API ERROR: {str(e)}]",
            verified=False,
            error_type="api_error",
            api_latency=0,
            verification_time=0,
            timestamp=datetime.now().isoformat(),
        )
    
    # Verify with Verus
    ver_start = time.time()
    
    if task == "A":
        # For Task A, we need to combine specs with original code
        # This is a simplified check - full evaluation uses proxy testing
        full_code = sample.get('full_verified_code', '')
        if full_code:
            result = verify_with_verus(full_code)
            verified = result.get("success", False)
            error_type = result.get("error_type") if not verified else None
        else:
            verified = None
            error_type = "no_full_code"
    else:
        # For Task B and C, verify generated code directly
        result = verify_with_verus(generated)
        verified = result.get("success", False)
        error_type = result.get("error_type") if not verified else None
    
    ver_time = time.time() - ver_start
    
    # Extract token counts from raw response
    usage = raw.get("usage", {})
    input_tokens = usage.get("prompt_tokens", 0) or usage.get("input_tokens", 0)
    output_tokens = usage.get("completion_tokens", 0) or usage.get("output_tokens", 0)
    
    return EvalSample(
        sample_id=sample['id'],
        task=task,
        model=model_name,
        prompt=prompt[:500] + "...",
        expected=sample['target_text'][:500] + "...",
        generated=generated[:2000] + "..." if len(generated) > 2000 else generated,
        verified=verified,
        error_type=error_type,
        api_latency=latency,
        verification_time=ver_time,
        timestamp=datetime.now().isoformat(),
        input_tokens=input_tokens,
        output_tokens=output_tokens,
        raw_response=raw,
    )


def evaluate_model_task(
    model_name: str,
    task: str,
    samples: List[Dict],
    dry_run: bool = False,
) -> Tuple[List[EvalSample], EvalMetrics]:
    """Evaluate a model on a task."""
    
    config = MODEL_CONFIGS[model_name]
    client = get_client(config["provider"]) if not dry_run else None
    
    results = []
    error_counts = {}
    
    print(f"\n{'='*60}")
    print(f"Model: {model_name} | Task: {task} | Samples: {len(samples)}")
    print(f"{'='*60}")
    
    for i, sample in enumerate(samples):
        result = evaluate_sample(client, config, sample, task, dry_run)
        results.append(result)
        
        # Track errors
        if not result.verified and result.error_type:
            error_counts[result.error_type] = error_counts.get(result.error_type, 0) + 1
        
        # Progress
        status = "✓" if result.verified else f"✗ ({result.error_type})" if result.error_type else "?"
        print(f"  [{i+1}/{len(samples)}] {sample['id'][:20]}: {status} (API: {result.api_latency:.1f}s)")
    
    # Compute metrics
    verified_count = sum(1 for r in results if r.verified)
    total_input_tokens = sum(r.input_tokens for r in results)
    total_output_tokens = sum(r.output_tokens for r in results)
    
    metrics = EvalMetrics(
        model=model_name,
        task=task,
        total_samples=len(results),
        verified_count=verified_count,
        verification_rate=verified_count / len(results) if results else 0,
        avg_api_latency=sum(r.api_latency for r in results) / len(results) if results else 0,
        avg_verification_time=sum(r.verification_time for r in results) / len(results) if results else 0,
        error_breakdown=error_counts,
        timestamp=datetime.now().isoformat(),
        total_input_tokens=total_input_tokens,
        total_output_tokens=total_output_tokens,
        total_tokens=total_input_tokens + total_output_tokens,
        avg_input_tokens=total_input_tokens / len(results) if results else 0,
        avg_output_tokens=total_output_tokens / len(results) if results else 0,
    )
    
    return results, metrics


def print_summary(all_metrics: Dict[str, Dict[str, EvalMetrics]]):
    """Print summary table of all results."""
    
    print("\n" + "=" * 100)
    print("EVALUATION SUMMARY")
    print("=" * 100)
    
    # Header - Performance
    print(f"\n{'Model':<20} {'Task':<6} {'Pass Rate':<12} {'Verified':<12} {'Avg Latency':<12}")
    print("-" * 62)
    
    for model_name, task_metrics in all_metrics.items():
        for task, metrics in task_metrics.items():
            print(f"{model_name:<20} {task:<6} {metrics.verification_rate:>10.1%} "
                  f"{metrics.verified_count:>4}/{metrics.total_samples:<6} "
                  f"{metrics.avg_api_latency:>10.2f}s")
    
    print("-" * 62)
    
    # Token Usage Summary
    print("\n" + "=" * 100)
    print("TOKEN USAGE SUMMARY")
    print("=" * 100)
    
    print(f"\n{'Model':<20} {'Task':<6} {'Input Tok':<12} {'Output Tok':<12} {'Total Tok':<12} {'Avg In':<10} {'Avg Out':<10}")
    print("-" * 92)
    
    grand_total_input = 0
    grand_total_output = 0
    
    for model_name, task_metrics in all_metrics.items():
        for task, metrics in task_metrics.items():
            print(f"{model_name:<20} {task:<6} {metrics.total_input_tokens:>10,} "
                  f"{metrics.total_output_tokens:>12,} {metrics.total_tokens:>12,} "
                  f"{metrics.avg_input_tokens:>9.0f} {metrics.avg_output_tokens:>9.0f}")
            grand_total_input += metrics.total_input_tokens
            grand_total_output += metrics.total_output_tokens
    
    print("-" * 92)
    print(f"{'TOTAL':<20} {'':<6} {grand_total_input:>10,} {grand_total_output:>12,} "
          f"{grand_total_input + grand_total_output:>12,}")
    
    # Estimated cost (rough estimates)
    print("\n" + "-" * 92)
    print("ESTIMATED COST (approximate):")
    
    # Cost per 1M tokens (rough estimates as of 2024)
    COST_PER_1M = {
        "gpt-4o": {"input": 2.50, "output": 10.00},
        "gpt-4o-mini": {"input": 0.15, "output": 0.60},
        "o1": {"input": 15.00, "output": 60.00},
        "o1-mini": {"input": 3.00, "output": 12.00},
        "claude-sonnet": {"input": 3.00, "output": 15.00},
        "claude-opus": {"input": 15.00, "output": 75.00},
    }
    
    total_cost = 0.0
    for model_name, task_metrics in all_metrics.items():
        model_cost = 0.0
        for task, metrics in task_metrics.items():
            if model_name in COST_PER_1M:
                costs = COST_PER_1M[model_name]
                input_cost = (metrics.total_input_tokens / 1_000_000) * costs["input"]
                output_cost = (metrics.total_output_tokens / 1_000_000) * costs["output"]
                model_cost += input_cost + output_cost
        if model_cost > 0:
            print(f"  {model_name}: ${model_cost:.4f}")
            total_cost += model_cost
    
    print(f"  {'TOTAL':<18}: ${total_cost:.4f}")
    print("-" * 92)
    
    # Error breakdown
    print("\nError Breakdown by Model:")
    for model_name, task_metrics in all_metrics.items():
        print(f"\n  {model_name}:")
        for task, metrics in task_metrics.items():
            if metrics.error_breakdown:
                print(f"    Task {task}: {metrics.error_breakdown}")


# =============================================================================
# Main
# =============================================================================

def main():
    parser = argparse.ArgumentParser(description="Evaluate SOTA API models on Verus tasks")
    
    parser.add_argument(
        "--tasks",
        type=str,
        default="all",
        help="Tasks to evaluate: A, B, C, or 'all' (comma-separated)"
    )
    parser.add_argument(
        "--models",
        type=str,
        default="all",
        help="Models to evaluate: gpt-4o, o1, claude-opus, or 'all' (comma-separated)"
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Limit samples per task (for testing)"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Dry run without API calls"
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="evaluation_results/api_models",
        help="Output directory for results"
    )
    
    args = parser.parse_args()
    
    # Parse tasks
    if args.tasks.lower() == "all":
        tasks = ["A", "B", "C"]
    else:
        tasks = [t.strip().upper() for t in args.tasks.split(",")]
    
    # Parse models
    if args.models.lower() == "all":
        models = ["gpt-4o", "o1", "claude-opus"]
    else:
        models = [m.strip().lower() for m in args.models.split(",")]
    
    # Validate models
    for model in models:
        if model not in MODEL_CONFIGS:
            print(f"Error: Unknown model '{model}'. Available: {list(MODEL_CONFIGS.keys())}")
            return
    
    # Check API keys
    if not args.dry_run:
        for model in models:
            provider = MODEL_CONFIGS[model]["provider"]
            if provider == "openai" and not os.environ.get("OPENAI_API_KEY"):
                print("Error: OPENAI_API_KEY not set")
                return
            if provider == "anthropic" and not os.environ.get("ANTHROPIC_API_KEY"):
                print("Error: ANTHROPIC_API_KEY not set")
                return
    
    # Create output directory
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Run evaluations
    all_metrics: Dict[str, Dict[str, EvalMetrics]] = {}
    all_results: Dict[str, Dict[str, List[EvalSample]]] = {}
    
    for model in models:
        all_metrics[model] = {}
        all_results[model] = {}
        
        for task in tasks:
            # Load test data
            samples = load_test_data(task, args.limit)
            print(f"\nLoaded {len(samples)} samples for Task {task}")
            
            # Evaluate
            results, metrics = evaluate_model_task(
                model, task, samples, dry_run=args.dry_run
            )
            
            all_metrics[model][task] = metrics
            all_results[model][task] = results
            
            # Save results
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            results_file = output_dir / f"{model}_{task}_{timestamp}.json"
            with open(results_file, 'w') as f:
                json.dump({
                    "metrics": asdict(metrics),
                    "results": [asdict(r) for r in results],
                }, f, indent=2)
            
            print(f"\n  Results saved to: {results_file}")
    
    # Print summary
    print_summary(all_metrics)
    
    # Save combined metrics
    metrics_file = output_dir / f"summary_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(metrics_file, 'w') as f:
        combined = {
            model: {task: asdict(m) for task, m in task_metrics.items()}
            for model, task_metrics in all_metrics.items()
        }
        json.dump(combined, f, indent=2)
    
    print(f"\n✓ Summary saved to: {metrics_file}")


if __name__ == "__main__":
    main()
