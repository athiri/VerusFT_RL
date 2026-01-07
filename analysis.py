"""
Dependency and structure analysis for Verus files.
Fast pre-filtering before expensive verification.
"""

import re
from pathlib import Path
from typing import List, Set, Tuple, Dict, Optional
from dataclasses import dataclass

# Allowed dependencies for strict filtering
ALLOWED_DEPS = {
    "vstd",
    "std",
    "core",
    "verus_builtin",
    "verus_builtin_macros",
}

# Verus tokens that indicate verification code
VERUS_TOKENS = {
    "verus!",
    "#[verus::",
    "requires",
    "ensures",
    "decreases",
    "invariant",
    "ghost",
    "proof",
    "spec",
    "exec",
    "reveal",
    "opens_invariants",
}

# Spec keywords
SPEC_KEYWORDS = {
    "requires",
    "ensures",
    "invariant",
    "decreases",
    "spec fn",
    "proof fn",
}

# Bogus stub patterns - files containing these are NOT suitable for SFT
STUB_PATTERNS = {
    "assume(false)",
    "unreached()",
}


@dataclass
class FileAnalysis:
    """Analysis results for a single file"""
    path: Path
    has_verus_block: bool
    has_spec_keywords: bool
    dependencies: List[str]
    external_dependencies: List[str]
    file_type: str  # "main", "lib", "standalone"
    has_external_types: bool
    has_recursive_spec: bool
    missing_decreases: bool
    score: int
    line_count: int
    is_candidate: bool
    is_stub: bool = False  # True if contains assume(false) or unreached()
    rejection_reason: Optional[str] = None


def extract_use_statements(content: str) -> List[str]:
    """Extract all use statements from Rust code"""
    pattern = re.compile(r'^use\s+([\w:]+)', re.MULTILINE)
    deps = []
    for match in pattern.finditer(content):
        dep = match.group(1)
        # Extract the root crate name (before ::)
        if '::' in dep:
            root = dep.split('::')[0]
        else:
            root = dep
        deps.append(root)
    return list(set(deps))


def check_dependencies(deps: List[str]) -> Tuple[bool, List[str]]:
    """Check if dependencies are allowed. Returns (is_allowed, external_deps)"""
    external = []
    for dep in deps:
        if dep not in ALLOWED_DEPS:
            external.append(dep)
    return len(external) == 0, external


def has_verus_block(content: str) -> bool:
    """Check if content is wrapped in verus! block"""
    return "verus!" in content


def has_stub_patterns(content: str) -> bool:
    """
    Check if content contains bogus stub patterns.
    Files with these patterns trivially "verify" but have no real implementation.
    NOT suitable for SFT training.
    """
    return any(pattern in content for pattern in STUB_PATTERNS)


def has_spec_keywords(content: str) -> bool:
    """Check if content has spec keywords"""
    content_lower = content.lower()
    return any(keyword in content_lower for keyword in SPEC_KEYWORDS)


def strip_comments_and_strings(content: str) -> str:
    """
    Remove comments and string literals to avoid false positives.
    Only count keywords in actual code.
    """
    result = []
    i = 0
    in_string = False
    string_char = None
    
    while i < len(content):
        # Check for line comment
        if not in_string and i + 1 < len(content) and content[i:i+2] == '//':
            # Skip to end of line
            while i < len(content) and content[i] != '\n':
                i += 1
            continue
        
        # Check for block comment
        if not in_string and i + 1 < len(content) and content[i:i+2] == '/*':
            # Skip to end of block comment
            i += 2
            while i + 1 < len(content) and content[i:i+2] != '*/':
                i += 1
            i += 2
            continue
        
        # Check for string start/end
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
        
        # Only add non-string content
        if not in_string:
            result.append(content[i])
        
        i += 1
    
    return ''.join(result)


def count_verus_tokens(content: str) -> int:
    """Count occurrences of Verus tokens in actual code (not comments/strings)"""
    clean_content = strip_comments_and_strings(content)
    return sum(clean_content.count(token) for token in VERUS_TOKENS)


def score_file(content: str) -> int:
    """
    Score file based on Verus constructs in actual code.
    Strips comments and strings first to avoid false positives.
    """
    clean_content = strip_comments_and_strings(content)
    verus_count = sum(clean_content.count(token) for token in VERUS_TOKENS)
    spec_count = sum(clean_content.count(kw) for kw in SPEC_KEYWORDS)
    return verus_count + spec_count


def detect_file_type(filepath: Path, content: str) -> str:
    """Detect if file is main.rs, lib.rs, or standalone"""
    if filepath.name == "main.rs":
        return "main"
    elif filepath.name == "lib.rs":
        return "lib"
    else:
        # Check for main function
        if re.search(r'\bfn\s+main\s*\(', content):
            return "main"
        return "standalone"


def has_external_types(content: str) -> bool:
    """Check for types declared outside verus! block that might be used inside"""
    # Look for struct/enum/type declarations before verus! block
    verus_pos = content.find("verus!")
    if verus_pos == -1:
        return False
    
    before_verus = content[:verus_pos]
    # Check for struct/enum/type declarations
    has_struct = bool(re.search(r'struct\s+\w+', before_verus))
    has_enum = bool(re.search(r'enum\s+\w+', before_verus))
    has_type = bool(re.search(r'type\s+\w+', before_verus))
    
    if has_struct or has_enum or has_type:
        # Check if these types are used inside verus! block
        after_verus = content[verus_pos:]
        # Simple heuristic: if type name appears in verus block
        # This is a conservative check - might have false positives
        return True
    
    return False


def has_recursive_spec_without_decreases(content: str) -> bool:
    """Check for recursive spec functions without decreases clause"""
    # Look for spec fn that calls itself
    spec_fn_pattern = r'spec\s+fn\s+(\w+)\s*\([^)]*\)\s*->[^{]*\{'
    
    for match in re.finditer(spec_fn_pattern, content):
        fn_name = match.group(1)
        fn_body_start = match.end()
        # Find the function body (simplified - look for matching braces)
        fn_body = content[fn_body_start:]
        
        # Check if function calls itself
        if f"{fn_name}(" in fn_body:
            # Check if decreases clause exists
            fn_decl = content[match.start():fn_body_start]
            if "decreases" not in fn_decl:
                return True
    
    return False


def analyze_file(filepath: Path) -> FileAnalysis:
    """Perform comprehensive analysis on a single file"""
    try:
        content = filepath.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return FileAnalysis(
            path=filepath,
            has_verus_block=False,
            has_spec_keywords=False,
            dependencies=[],
            external_dependencies=[],
            file_type="unknown",
            has_external_types=False,
            has_recursive_spec=False,
            missing_decreases=False,
            score=0,
            line_count=0,
            is_candidate=False,
            rejection_reason="read_error"
        )
    
    line_count = len(content.splitlines())
    
    # Basic checks
    has_block = has_verus_block(content)
    has_spec = has_spec_keywords(content)
    is_stub = has_stub_patterns(content)
    deps = extract_use_statements(content)
    is_allowed, external_deps = check_dependencies(deps)
    score = score_file(content)
    file_type = detect_file_type(filepath, content)
    external_types = has_external_types(content)
    recursive_issue = has_recursive_spec_without_decreases(content)
    
    # Determine if candidate
    is_candidate = True
    rejection_reason = None
    
    if not has_block:
        is_candidate = False
        rejection_reason = "no_verus_block"
    elif not has_spec:
        is_candidate = False
        rejection_reason = "no_spec_keywords"
    elif not is_allowed:
        is_candidate = False
        rejection_reason = f"external_dependencies: {external_deps}"
    elif line_count < 5:
        is_candidate = False
        rejection_reason = "too_small"
    elif external_types:
        # Warning but not necessarily rejection - depends on context
        # We'll let verification catch this
        pass
    elif recursive_issue:
        # Warning - verification will catch this
        pass
    
    return FileAnalysis(
        path=filepath,
        has_verus_block=has_block,
        has_spec_keywords=has_spec,
        dependencies=deps,
        external_dependencies=external_deps,
        file_type=file_type,
        has_external_types=external_types,
        has_recursive_spec=recursive_issue,
        missing_decreases=recursive_issue,
        score=score,
        line_count=line_count,
        is_candidate=is_candidate,
        is_stub=is_stub,
        rejection_reason=rejection_reason
    )

