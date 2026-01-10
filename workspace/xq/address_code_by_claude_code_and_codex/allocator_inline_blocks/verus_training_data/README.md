# Verus training data from allocator_inline_blocks

This folder contains JSONL datasets built from code blocks in `allocator_inline_blocks/*.rs`.
Each line is one example with the following fields:

- id: unique identifier
- task: code_completion | spec_completion | exec_to_spec | proof_generation
- language: verus
- prompt: input text
- completion: target text
- source: {"file": "...", "item": "..."}

Files:
- code_completion.jsonl
- spec_completion.jsonl
- exec_to_spec.jsonl
- proof_generation.jsonl

How to use:
- code_completion: append `completion` to `prompt`.
- spec_completion: replace the `/* TODO */` marker in `prompt` with `completion`.
- exec_to_spec: replace the `/* TODO: add requires/ensures */` marker in `prompt` with `completion`.
- proof_generation: replace the `/* TODO */` marker in `prompt` with `completion`.

Example loader (Python):

```python
import json
from pathlib import Path

path = Path("allocator_inline_blocks/verus_training_data/spec_completion.jsonl")
for line in path.read_text().splitlines():
    ex = json.loads(line)
    merged = ex["prompt"].replace("/* TODO */", ex["completion"])
    # use merged for training or evaluation
```

Notes:
- Prompts/completions are snippets from the source files, with minor cleanup (e.g., line_count markers removed) when present.
- The datasets are intentionally small and can be extended with more examples.
