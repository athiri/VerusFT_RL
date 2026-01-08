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

# Dafny syntax patterns - indicates LLM generated wrong language
DAFNY_PATTERNS = [
    r'\bvar\s+\w+\s*:=',           # var x :=
    r'\bset<\w+>',                  # set<int>
    r'\bseq<\w+>',                  # seq<int>  
    r'\blemma\s+\w+\s*\(',          # lemma name(
    r'\bfunction\s+\w+\s*\([^)]*\)\s*:\s*\w+',  # function name(...): type
    r':=\s*\w+\s*[\+\-]\s*\d+\s*;', # := x + 1;
    r'\ba\s*:\|\s*a\s+in\b',        # a :| a in (Dafny choice)
    # Dafny cardinality notation with pipes: |s1|, |s1 * s2|
    r'\bdecreases\s+\|',           # decreases |...| - Dafny cardinality in decreases
    r'\bensures\s+\|',             # ensures |...| - Dafny cardinality in ensures
    r'\|\w+\s*\*\s*\w+\|',         # |s1 * s2| - Dafny cardinality of set operation
    # Dafny empty set literal comparison
    r'\w+\s*!=\s*\{\s*\}',         # s1 != {} - comparing to empty set literal
    # Dafny set subset with <= operator
    r'\bassert\s+\w+\s*\*\s*\w+\s*<=\s*\w+\s*;',  # assert s1 * s2 <= s1;
    # Standalone spec clauses without fn keyword (Dafny method/lemma style)
    r'^\s*requires\s+[^(]+$',      # requires clause on its own line (not in fn signature)
    r'^\s*ensures\s+[^(]+$',       # ensures clause on its own line (not in fn signature)
]

# Invalid Verus patterns - LLM mistakes where spec clauses are used as function calls
# In valid Verus, requires/ensures are clauses in function signatures, not function calls
INVALID_VERUS_PATTERNS = [
    # requires() / ensures() / decreases() as function calls inside function body
    # These appear AFTER the opening brace of a function
    r'\{\s*\n\s+requires\s*\(',      # { \n    requires( - spec call after function start
    r'\{\s*\n\s+ensures\s*\(',       # { \n    ensures( - spec call after function start  
    # invariant([...]) with array brackets - invalid syntax
    r'\binvariant\s*\(\s*\[',        # invariant([ - array-style invariant call
    # decreases() as a standalone statement (not in function signature or loop header)
    r';\s*\n\s+decreases\s*\(',      # ; \n    decreases( - decreases after statement
]


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


def has_dafny_syntax(content: str) -> bool:
    """
    Check if content contains Dafny-specific syntax.
    This indicates the LLM generated code in the wrong verification language.
    NOT suitable for Verus verification.
    """
    import re
    for pattern in DAFNY_PATTERNS:
        # Use MULTILINE flag for patterns with ^ and $ anchors
        if re.search(pattern, content, re.MULTILINE):
            return True
    return False


def has_invalid_verus_syntax(content: str) -> bool:
    """
    Check if content contains invalid Verus syntax patterns.
    
    Common LLM mistakes:
    - Using requires(), ensures(), decreases() as function calls inside the function body
    - Using invariant([...]) with array brackets
    
    In valid Verus, these are specification clauses that appear in function signatures
    or loop headers, not as function calls.
    
    NOT suitable for Verus verification.
    """
    import re
    for pattern in INVALID_VERUS_PATTERNS:
        if re.search(pattern, content, re.MULTILINE):
            return True
    return False


def has_llm_explanation_text(content: str) -> bool:
    """
    Check if content contains LLM explanation text that would cause syntax errors.
    """
    import re
    explanation_patterns = [
        r'^The (key|main) (changes|fix|issue)',
        r'^Key (changes|fixes)',
        r'^Main (changes|fixes)',
        r'^I (made|added|fixed|changed)',
        r'^However,?\s',
        r'^If you (can|want|need)',
    ]
    for pattern in explanation_patterns:
        if re.search(pattern, content, re.MULTILINE | re.IGNORECASE):
            return True
    return False


def has_spec_keywords(content: str) -> bool:
    """Check if content has spec keywords"""
    content_lower = content.lower()
    return any(keyword in content_lower for keyword in SPEC_KEYWORDS)


def strip_comments_and_strings(content: str) -> str:
    """
    Remove comments and string literals to avoid false positives.
    Only count keywords in actual code.
    
    Properly handles:
    - Line comments (//)
    - Block comments (/* */)
    - String literals with escaped quotes (\") and escaped backslashes (\\)
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
            elif content[i] == string_char:
                # Count consecutive backslashes before the quote
                # An even number means the quote is NOT escaped (e.g., "test\\" ends here)
                # An odd number means the quote IS escaped (e.g., "test\"" continues)
                num_backslashes = 0
                j = i - 1
                while j >= 0 and content[j] == '\\':
                    num_backslashes += 1
                    j -= 1
                
                if num_backslashes % 2 == 0:
                    # Even backslashes (including 0) - quote terminates string
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


def extract_function_body(content: str, start_pos: int) -> str:
    """
    Extract just the function body using brace matching.
    
    Args:
        content: The full file content
        start_pos: Position right after the opening brace of the function
        
    Returns:
        The content of the function body (between { and })
    """
    depth = 1
    i = start_pos
    body_start = start_pos
    
    while i < len(content) and depth > 0:
        char = content[i]
        if char == '{':
            depth += 1
        elif char == '}':
            depth -= 1
        i += 1
    
    # Return content between braces (excluding the final })
    return content[body_start:i-1] if depth == 0 else ""


def has_recursive_spec_without_decreases(content: str) -> bool:
    """
    Check for recursive spec functions without decreases clause.
    
    Uses brace matching to extract only the function body,
    avoiding false positives from functions defined later in the file.
    """
    # Look for spec fn that calls itself
    spec_fn_pattern = r'spec\s+fn\s+(\w+)\s*\([^)]*\)\s*->[^{]*\{'
    
    for match in re.finditer(spec_fn_pattern, content):
        fn_name = match.group(1)
        fn_body_start = match.end()
        
        # Extract just this function's body using brace matching
        fn_body = extract_function_body(content, fn_body_start)
        
        # Check if function calls itself within its own body
        if f"{fn_name}(" in fn_body:
            # Check if decreases clause exists in the function declaration
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
    
    # Check for Dafny syntax (wrong language)
    is_dafny = has_dafny_syntax(content)
    
    # Check for invalid Verus syntax (LLM mistakes)
    invalid_verus = has_invalid_verus_syntax(content)
    
    # Check for LLM explanation text
    has_llm_text = has_llm_explanation_text(content)
    
    # Determine if candidate
    is_candidate = True
    rejection_reason = None
    
    if not has_block:
        is_candidate = False
        rejection_reason = "no_verus_block"
    elif is_dafny:
        # CRITICAL: Dafny code will not compile as Verus/Rust
        is_candidate = False
        rejection_reason = "dafny_syntax: contains Dafny code instead of Verus/Rust"
    elif invalid_verus:
        # CRITICAL: Invalid Verus syntax - spec clauses used as function calls
        is_candidate = False
        rejection_reason = "invalid_verus_syntax: spec clauses (requires/ensures/invariant) used as function calls instead of signature clauses"
    elif has_llm_text:
        # CRITICAL: LLM explanations cause syntax errors
        is_candidate = False
        rejection_reason = "llm_explanation_text: contains prose that causes syntax errors"
    elif not has_spec:
        is_candidate = False
        rejection_reason = "no_spec_keywords"
    elif is_stub:
        # CRITICAL: Files with assume(false) or unreached() are NOT suitable for SFT
        # They trivially verify without real implementation - see lines 46-50
        is_candidate = False
        rejection_reason = "stub_file: contains assume(false) or unreached()"
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

