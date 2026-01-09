"""
Multi-Task Training Script for Verus SFT

Train language models on Verus verification tasks:
- Task A: Code → Specifications
- Task B: Specifications → Verified Code  
- Task C: Error-Guided Repair

Usage:
    # Train on Task A (spec generation)
    python train_task.py --task A --epochs 3
    
    # Train on all tasks (multi-task)
    python train_task.py --task multi --epochs 5
    
    # Train on Task C with custom model
    python train_task.py --task C --model Qwen/Qwen2.5-Coder-1.5B --epochs 3
"""

import argparse
import json
import os
from pathlib import Path
from typing import Optional

from datasets import Dataset
from transformers import AutoTokenizer, AutoModelForCausalLM
from transformers.utils import is_accelerate_available
from trl import SFTTrainer, SFTConfig

try:
    from peft import LoraConfig
except ImportError:
    LoraConfig = None


# Default paths
TRAINING_DATA_DIR = Path("workspace/training_tasks")
OUTPUT_DIR = Path("sft_output")

# Task file mapping
TASK_FILES = {
    "A": "task_a_sft.jsonl",
    "B": "task_b_sft.jsonl", 
    "C": "task_c_sft.jsonl",
    "multi": "multi_task_sft.jsonl",
}

TASK_DESCRIPTIONS = {
    "A": "Code → Specifications (generate requires/ensures from code)",
    "B": "Specifications → Code (generate verified implementation)",
    "C": "Error-Guided Repair (fix verification errors)",
    "multi": "Multi-task (all tasks combined)",
}


def load_training_data(task: str, data_path: Optional[str] = None, limit: Optional[int] = None) -> Dataset:
    """
    Load training data for the specified task.
    
    Args:
        task: Task identifier (A, B, C, or multi)
        data_path: Optional custom path to data file
        limit: Optional limit on number of examples
        
    Returns:
        HuggingFace Dataset
    """
    if data_path:
        path = Path(data_path)
    else:
        if task not in TASK_FILES:
            raise ValueError(f"Unknown task: {task}. Choose from: {list(TASK_FILES.keys())}")
        path = TRAINING_DATA_DIR / TASK_FILES[task]
    
    if not path.exists():
        raise FileNotFoundError(
            f"Training data not found at {path}. "
            f"Run 'python verify_and_create_tasks.py' first to create training data."
        )
    
    examples = []
    with open(path, 'r') as f:
        for i, line in enumerate(f):
            if limit and i >= limit:
                break
            examples.append(json.loads(line))
    
    return Dataset.from_list(examples)


def train(
    task: str,
    model_name: str = "Qwen/Qwen2.5-Coder-7B",
    output_dir: str = "./sft_output",
    data_path: Optional[str] = None,
    num_epochs: int = 3,
    batch_size: int = 2,
    learning_rate: float = 2e-4,
    max_seq_length: int = 2048,
    use_lora: bool = True,
    lora_rank: int = 16,
    gradient_accumulation_steps: int = 4,
    limit_examples: Optional[int] = None,
):
    """
    Train on the specified task.
    
    Args:
        task: Task to train on (A, B, C, or multi)
        model_name: Base model to fine-tune
        output_dir: Directory to save model
        data_path: Optional custom data path
        num_epochs: Number of training epochs
        batch_size: Per-device batch size
        learning_rate: Learning rate
        max_seq_length: Maximum sequence length
        use_lora: Whether to use LoRA
        lora_rank: LoRA rank if using LoRA
        gradient_accumulation_steps: Gradient accumulation steps
        limit_examples: Limit number of training examples (for testing)
    """
    task = task.upper() if task.lower() != "multi" else "multi"
    
    print("=" * 60)
    print(f"Training Task: {task}")
    print(f"Description: {TASK_DESCRIPTIONS.get(task, 'Unknown')}")
    print("=" * 60)
    
    # Load data
    print(f"\nLoading training data...")
    train_dataset = load_training_data(task, data_path, limit_examples)
    print(f"Loaded {len(train_dataset)} training examples")
    
    # Load model and tokenizer
    print(f"\nLoading model: {model_name}")
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    
    model_load_kwargs = {
        "torch_dtype": "auto",
        "low_cpu_mem_usage": True,
    }
    if is_accelerate_available():
        model_load_kwargs["device_map"] = "auto"
    
    model = AutoModelForCausalLM.from_pretrained(model_name, **model_load_kwargs)
    model.config.pad_token_id = tokenizer.pad_token_id
    
    # Configure output directory
    task_output_dir = f"{output_dir}/task_{task.lower()}"
    
    # Training config
    config = SFTConfig(
        output_dir=task_output_dir,
        per_device_train_batch_size=batch_size,
        num_train_epochs=num_epochs,
        max_seq_length=max_seq_length,
        learning_rate=learning_rate,
        gradient_accumulation_steps=gradient_accumulation_steps,
        logging_steps=10,
        save_strategy="epoch",
        save_total_limit=2,
        bf16=True,  # Use bfloat16 for efficiency
        gradient_checkpointing=True,  # Save memory
        report_to="none",  # Disable wandb/tensorboard for simplicity
    )
    
    # LoRA config
    lora_config = None
    if use_lora and LoraConfig is not None:
        lora_config = LoraConfig(
            r=lora_rank,
            lora_alpha=lora_rank * 2,
            target_modules=["q_proj", "k_proj", "v_proj", "o_proj"],
            lora_dropout=0.05,
            bias="none",
            task_type="CAUSAL_LM",
        )
        print(f"\nUsing LoRA with rank={lora_rank}")
    
    # Create trainer
    trainer = SFTTrainer(
        model=model,
        train_dataset=train_dataset,
        peft_config=lora_config,
        args=config,
    )
    
    # Train
    print(f"\nStarting training...")
    print(f"  Epochs: {num_epochs}")
    print(f"  Batch size: {batch_size}")
    print(f"  Learning rate: {learning_rate}")
    print(f"  Max sequence length: {max_seq_length}")
    print(f"  Output: {task_output_dir}")
    print()
    
    trainer.train()
    
    # Save
    trainer.model.save_pretrained(task_output_dir)
    tokenizer.save_pretrained(task_output_dir)
    
    print(f"\n✓ Training complete!")
    print(f"  Model saved to: {task_output_dir}")
    
    # Save training metadata
    metadata = {
        "task": task,
        "model_name": model_name,
        "num_examples": len(train_dataset),
        "num_epochs": num_epochs,
        "batch_size": batch_size,
        "learning_rate": learning_rate,
        "use_lora": use_lora,
        "lora_rank": lora_rank if use_lora else None,
    }
    with open(f"{task_output_dir}/training_metadata.json", 'w') as f:
        json.dump(metadata, f, indent=2)
    
    return task_output_dir


def main():
    parser = argparse.ArgumentParser(
        description="Train on Verus verification tasks",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    # Train on spec generation (Task A)
    python train_task.py --task A --epochs 3
    
    # Train on all tasks
    python train_task.py --task multi --epochs 5
    
    # Quick test run with limited data
    python train_task.py --task A --epochs 1 --limit 100
    
    # Use smaller model for less VRAM
    python train_task.py --task A --model Qwen/Qwen2.5-Coder-1.5B
        """
    )
    
    parser.add_argument(
        "--task",
        type=str,
        required=True,
        choices=["A", "B", "C", "multi", "a", "b", "c"],
        help="Task to train: A (code→spec), B (spec→code), C (repair), multi (all)"
    )
    parser.add_argument(
        "--model",
        type=str,
        default="Qwen/Qwen2.5-Coder-7B",
        help="Base model to fine-tune"
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="./sft_output",
        help="Output directory for trained model"
    )
    parser.add_argument(
        "--data",
        type=str,
        default=None,
        help="Custom path to training data JSONL file"
    )
    parser.add_argument(
        "--epochs",
        type=int,
        default=3,
        help="Number of training epochs"
    )
    parser.add_argument(
        "--batch-size",
        type=int,
        default=2,
        help="Per-device batch size"
    )
    parser.add_argument(
        "--lr",
        type=float,
        default=2e-4,
        help="Learning rate"
    )
    parser.add_argument(
        "--max-seq-length",
        type=int,
        default=2048,
        help="Maximum sequence length"
    )
    parser.add_argument(
        "--no-lora",
        action="store_true",
        help="Disable LoRA (full fine-tuning)"
    )
    parser.add_argument(
        "--lora-rank",
        type=int,
        default=16,
        help="LoRA rank"
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Limit number of training examples (for testing)"
    )
    
    args = parser.parse_args()
    
    train(
        task=args.task,
        model_name=args.model,
        output_dir=args.output_dir,
        data_path=args.data,
        num_epochs=args.epochs,
        batch_size=args.batch_size,
        learning_rate=args.lr,
        max_seq_length=args.max_seq_length,
        use_lora=not args.no_lora,
        lora_rank=args.lora_rank,
        limit_examples=args.limit,
    )


if __name__ == "__main__":
    main()
