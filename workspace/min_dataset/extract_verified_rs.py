#!/usr/bin/env python3
"""Extract unique verified .rs files from the dataset.jsonl"""

import json
import os
import hashlib
from pathlib import Path

def main():
    # Paths
    dataset_path = Path(__file__).parent / "dataset.jsonl"
    output_dir = Path(__file__).parent / "verified_sources"
    
    # Create output directory
    output_dir.mkdir(exist_ok=True)
    
    # Track unique codes by hash to avoid duplicates
    seen_hashes = set()
    extracted_count = 0
    
    # Read dataset and extract unique verified code
    with open(dataset_path, 'r') as f:
        for line in f:
            entry = json.loads(line)
            code = entry.get('full_verified_code', '')
            
            if not code:
                continue
            
            # Hash the code to detect duplicates
            code_hash = hashlib.md5(code.encode()).hexdigest()[:12]
            
            if code_hash in seen_hashes:
                continue
            
            seen_hashes.add(code_hash)
            
            # Generate filename from metadata
            original_id = entry.get('metadata', {}).get('original_id', '')
            func_name = entry.get('metadata', {}).get('function_name', 'unknown')
            source = entry.get('source', 'unknown')
            
            # Create meaningful filename
            if original_id:
                filename = f"{source}_{func_name}_{original_id}.rs"
            else:
                filename = f"{source}_{func_name}_{code_hash}.rs"
            
            # Sanitize filename
            filename = filename.replace('/', '_').replace('\\', '_').replace(' ', '_')
            
            # Write the file
            output_path = output_dir / filename
            with open(output_path, 'w') as out_f:
                out_f.write(code)
            
            extracted_count += 1
    
    print(f"Extracted {extracted_count} unique verified .rs files to {output_dir}")
    print(f"Total unique code hashes: {len(seen_hashes)}")

if __name__ == "__main__":
    main()
