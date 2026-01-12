#!/usr/bin/env python3
"""
Convert Verus dataset to Qwen2/Qwen3 fine-tuning format.

Supports:
- Qwen2 format: {"messages": [{"role": "...", "content": "..."}]}
- Qwen3 format: {"conversations": [{"role": "...", "content": "..."}]}
"""

import json
import argparse
from pathlib import Path
from typing import Dict, List, Any

# System prompts for each task type
SYSTEM_PROMPTS = {
    "task_a": """You are a Verus formal verification expert. Given Verus code without specifications, add the appropriate specifications including:
- requires: preconditions that must hold before function execution
- ensures: postconditions that must hold after function execution  
- decreases: termination measures for loops/recursion
- invariant: loop invariants that hold at each iteration

Output only the specifications to add, one per line.""",

    "task_b": """You are a Verus formal verification expert. Given a function signature with specifications (requires, ensures), implement the complete verified Verus code that satisfies the specifications.

Output the complete Verus code including imports, the function implementation, and any necessary helper functions or assertions.""",

    "task_c": """You are a Verus formal verification expert. Given Verus code that has missing or incorrect specifications, fix the code to make it verify correctly.

The code may be missing:
- requires clauses (preconditions)
- ensures clauses (postconditions)  
- invariant clauses (loop invariants)
- decreases clauses (termination measures)
- assert statements (proof hints)

Output the complete corrected Verus code."""
}


def convert_to_qwen2_format(item: Dict[str, Any]) -> Dict[str, Any]:
    """Convert a single item to Qwen2 messages format."""
    task = item.get("task", "task_a")
    input_text = item.get("input_text", "")
    target_text = item.get("target_text", "")
    
    system_prompt = SYSTEM_PROMPTS.get(task, SYSTEM_PROMPTS["task_a"])
    
    # Format user content based on task type
    if task == "task_a":
        user_content = f"Add specifications to the following Verus code:\n\n```verus\n{input_text}\n```"
    elif task == "task_b":
        user_content = f"Implement the following Verus function with its specifications:\n\n```\n{input_text}\n```"
    else:  # task_c
        user_content = f"Fix the following Verus code to make it verify correctly:\n\n```verus\n{input_text}\n```"
    
    # Format assistant response
    if task == "task_a":
        assistant_content = target_text
    else:
        assistant_content = f"```verus\n{target_text}\n```"
    
    return {
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content},
            {"role": "assistant", "content": assistant_content}
        ]
    }


def convert_to_qwen3_format(item: Dict[str, Any]) -> Dict[str, Any]:
    """Convert a single item to Qwen3 conversations format."""
    task = item.get("task", "task_a")
    input_text = item.get("input_text", "")
    target_text = item.get("target_text", "")
    
    system_prompt = SYSTEM_PROMPTS.get(task, SYSTEM_PROMPTS["task_a"])
    
    # Format user content based on task type
    if task == "task_a":
        user_content = f"Add specifications to the following Verus code:\n\n```verus\n{input_text}\n```"
    elif task == "task_b":
        user_content = f"Implement the following Verus function with its specifications:\n\n```\n{input_text}\n```"
    else:  # task_c
        user_content = f"Fix the following Verus code to make it verify correctly:\n\n```verus\n{input_text}\n```"
    
    # Format assistant response
    if task == "task_a":
        assistant_content = target_text
    else:
        assistant_content = f"```verus\n{target_text}\n```"
    
    return {
        "conversations": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content},
            {"role": "assistant", "content": assistant_content}
        ]
    }


def process_dataset(
    input_path: str,
    output_dir: str,
    format_type: str = "both",
    tasks: List[str] = None,
    only_verified: bool = True
):
    """
    Process the dataset and convert to Qwen format.
    
    Args:
        input_path: Path to input JSONL file
        output_dir: Directory for output files
        format_type: "qwen2", "qwen3", or "both"
        tasks: List of tasks to include (e.g., ["task_a", "task_b", "task_c"])
        only_verified: Only include verified examples
    """
    input_path = Path(input_path)
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    if tasks is None:
        tasks = ["task_a", "task_b", "task_c"]
    
    # Statistics
    stats = {
        "total": 0,
        "converted": 0,
        "skipped_unverified": 0,
        "skipped_task": 0,
        "by_task": {"task_a": 0, "task_b": 0, "task_c": 0}
    }
    
    # Collect converted items
    qwen2_items = []
    qwen3_items = []
    
    print(f"Reading from: {input_path}")
    
    with open(input_path, 'r') as f:
        for line_num, line in enumerate(f, 1):
            if not line.strip():
                continue
            
            try:
                item = json.loads(line)
                stats["total"] += 1
                
                # Filter by verification status
                if only_verified:
                    verified = item.get("verified", False)
                    if not verified:
                        verification = item.get("verification", {})
                        verified = verification.get("verified", False)
                    if not verified:
                        stats["skipped_unverified"] += 1
                        continue
                
                # Filter by task type
                task = item.get("task", "unknown")
                if task not in tasks:
                    stats["skipped_task"] += 1
                    continue
                
                # Convert
                if format_type in ["qwen2", "both"]:
                    qwen2_items.append(convert_to_qwen2_format(item))
                if format_type in ["qwen3", "both"]:
                    qwen3_items.append(convert_to_qwen3_format(item))
                
                stats["converted"] += 1
                stats["by_task"][task] = stats["by_task"].get(task, 0) + 1
                
                if stats["total"] % 1000 == 0:
                    print(f"  Processed {stats['total']} items...")
                    
            except json.JSONDecodeError as e:
                print(f"  Warning: JSON decode error at line {line_num}: {e}")
                continue
    
    # Write output files
    if format_type in ["qwen2", "both"] and qwen2_items:
        output_file = output_dir / "dataset_qwen2.jsonl"
        with open(output_file, 'w') as f:
            for item in qwen2_items:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
        print(f"Written Qwen2 format: {output_file} ({len(qwen2_items)} items)")
    
    if format_type in ["qwen3", "both"] and qwen3_items:
        output_file = output_dir / "dataset_qwen3.jsonl"
        with open(output_file, 'w') as f:
            for item in qwen3_items:
                f.write(json.dumps(item, ensure_ascii=False) + '\n')
        print(f"Written Qwen3 format: {output_file} ({len(qwen3_items)} items)")
    
    # Also create task-specific files
    if len(tasks) > 1:
        for task in tasks:
            task_items_qwen2 = []
            task_items_qwen3 = []
            
            with open(input_path, 'r') as f:
                for line in f:
                    if not line.strip():
                        continue
                    try:
                        item = json.loads(line)
                        if item.get("task") != task:
                            continue
                        if only_verified:
                            verified = item.get("verified", False)
                            if not verified:
                                verification = item.get("verification", {})
                                verified = verification.get("verified", False)
                            if not verified:
                                continue
                        
                        if format_type in ["qwen2", "both"]:
                            task_items_qwen2.append(convert_to_qwen2_format(item))
                        if format_type in ["qwen3", "both"]:
                            task_items_qwen3.append(convert_to_qwen3_format(item))
                    except:
                        continue
            
            if task_items_qwen2:
                output_file = output_dir / f"{task}_qwen2.jsonl"
                with open(output_file, 'w') as f:
                    for item in task_items_qwen2:
                        f.write(json.dumps(item, ensure_ascii=False) + '\n')
                print(f"Written {task} Qwen2: {output_file} ({len(task_items_qwen2)} items)")
            
            if task_items_qwen3:
                output_file = output_dir / f"{task}_qwen3.jsonl"
                with open(output_file, 'w') as f:
                    for item in task_items_qwen3:
                        f.write(json.dumps(item, ensure_ascii=False) + '\n')
                print(f"Written {task} Qwen3: {output_file} ({len(task_items_qwen3)} items)")
    
    # Print statistics
    print("\n" + "="*50)
    print("Conversion Statistics:")
    print("="*50)
    print(f"Total items read:      {stats['total']}")
    print(f"Converted:             {stats['converted']}")
    print(f"Skipped (unverified):  {stats['skipped_unverified']}")
    print(f"Skipped (wrong task):  {stats['skipped_task']}")
    print(f"\nBy task:")
    for task, count in stats["by_task"].items():
        print(f"  {task}: {count}")
    
    return stats


def main():
    parser = argparse.ArgumentParser(
        description="Convert Verus dataset to Qwen2/Qwen3 fine-tuning format"
    )
    parser.add_argument(
        "--input", "-i",
        default="workspace/min_dataset/dataset.jsonl",
        help="Input JSONL file path"
    )
    parser.add_argument(
        "--output", "-o",
        default="workspace/training_tasks/qwen_format",
        help="Output directory"
    )
    parser.add_argument(
        "--format", "-f",
        choices=["qwen2", "qwen3", "both"],
        default="both",
        help="Output format type"
    )
    parser.add_argument(
        "--tasks", "-t",
        nargs="+",
        choices=["task_a", "task_b", "task_c"],
        default=["task_a", "task_b", "task_c"],
        help="Tasks to include"
    )
    parser.add_argument(
        "--include-unverified",
        action="store_true",
        help="Include unverified examples"
    )
    
    args = parser.parse_args()
    
    process_dataset(
        input_path=args.input,
        output_dir=args.output,
        format_type=args.format,
        tasks=args.tasks,
        only_verified=not args.include_unverified
    )


if __name__ == "__main__":
    main()
