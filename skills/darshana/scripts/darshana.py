#!/usr/bin/env python3
import sys
import os
import re
import json
import argparse

# --- P0: Paradigm (Constants) ---
QL_PATTERN = re.compile(r"\b(P[0-5][']?)\b")
HEADER_PATTERN = re.compile(r"^(#{1,6})\s+(.+)$")
LINK_PATTERN = re.compile(r"\[\[(.*?)\]\]")
FRONTMATTER_PATTERN = re.compile(r"^---\n(.*?)\n---", re.DOTALL)

def parse_frontmatter(content):
    """Extracts and naive-parses YAML frontmatter."""
    match = FRONTMATTER_PATTERN.match(content)
    if not match:
        return None, content
    
    fm_text = match.group(1)
    fm_data = {}
    for line in fm_text.split('\n'):
        if ':' in line:
            key, val = line.split(':', 1)
            fm_data[key.strip()] = val.strip()
            
    # Remove frontmatter from content for body processing
    body = content[match.end():]
    return fm_data, body

def get_ql_level(text):
    """Extracts QL P-Level from text if present."""
    matches = QL_PATTERN.findall(text)
    return matches[0] if matches else None

def scout(file_path):
    """P1: Scout - The Map"""
    if not os.path.exists(file_path):
        return {"error": "File not found"}
    
    with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
        full_text = f.read()

    fm_data, body = parse_frontmatter(full_text)
    
    skeleton = []
    lines = body.split('\n')
    
    # Calculate offset if frontmatter existed
    fm_lines = full_text[:full_text.find(body)].count('\n') if body != full_text else 0

    # Patterns for "Artifacts"
    # Matches "User:", "User: Hello", "## User", etc.
    CHAT_PATTERN = re.compile(r"^(?:#{0,6}\s*)?(?:User|Assistant|Frank|Model|System|Human|AI)(?::|\s-)\s*(.*)", re.IGNORECASE)
    QL_LINE_PATTERN = re.compile(r"^\s*[-*]?\s*\(?(P[0-5][']?|#[0-5])\)?[:\s-]", re.IGNORECASE)

    for i, line in enumerate(lines):
        line_stripped = line.strip()
        
        # 1. Header Match (Standard Markdown Headers)
        match_header = HEADER_PATTERN.match(line)
        if match_header:
            level = len(match_header.group(1))
            title = match_header.group(2).strip()
            ql = get_ql_level(title)
            skeleton.append({
                "line": i + 1 + fm_lines,
                "type": "header",
                "level": level,
                "content": title,
                "ql": ql
            })
            # Check if header is ALSO a chat turn (e.g. "## User")
            # If so, we let it be a header, but we don't duplicate it as a turn.
            continue

        # 2. QL Artifact Match (High priority - finding the logic)
        match_ql = QL_LINE_PATTERN.match(line)
        if match_ql:
            ql_val = match_ql.group(1)
            context = line_stripped[len(match_ql.group(0)):]
            skeleton.append({
                "line": i + 1 + fm_lines,
                "type": "ql_artifact",
                "level": 0,
                "content": f"{ql_val}: {context[:60]}...",
                "ql": ql_val
            })
            continue

        # 3. Chat Turn Match (Textual turns)
        match_chat = CHAT_PATTERN.match(line)
        if match_chat:
             content = match_chat.group(1).strip()
             if not content: content = "(Start of Turn)"
             skeleton.append({
                "line": i + 1 + fm_lines,
                "type": "turn",
                "level": 0,
                "content": f"{line.split(':')[0]}: {content[:50]}...",
                "ql": None
            })
             continue

    return {
        "file": os.path.basename(file_path),
        "lines": len(lines) + fm_lines,
        "chars": len(full_text),
        "frontmatter": fm_data,
        "skeleton": skeleton
    }

def read_section(file_path, target_header=None, target_ql=None):
    """P2: Read - The Lens"""
    if not os.path.exists(file_path):
        return "Error: File not found"

    with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
        lines = f.readlines()

    if not target_header and not target_ql:
        # Default: read first 50 lines (Abstract)
        return "".join(lines[:50]) + "\n... (use --section to read more)"

    # Find start line
    start_idx = -1
    found_level = 0
    
    for i, line in enumerate(lines):
        match = HEADER_PATTERN.match(line)
        if match:
            title = match.group(2)
            # Check Header Match
            if target_header and target_header.lower() in title.lower():
                start_idx = i
                found_level = len(match.group(1))
                break
            # Check QL Match
            if target_ql and target_ql in title:
                start_idx = i
                found_level = len(match.group(1))
                break
    
    if start_idx == -1:
        return f"Error: Section '{target_header or target_ql}' not found."

    # Find end line (next header of same or higher level)
    end_idx = len(lines)
    for i in range(start_idx + 1, len(lines)):
        match = HEADER_PATTERN.match(lines[i])
        if match:
            level = len(match.group(1))
            if level <= found_level:
                end_idx = i
                break
    
    return "".join(lines[start_idx:end_idx])

def extract_threads(file_path):
    """P3: Threads - The Weave"""
    if not os.path.exists(file_path):
        return []
    
    with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
        text = f.read()
        
    links = LINK_PATTERN.findall(text)
    # Dedup and sort
    return sorted(list(set(links)))

def main():
    parser = argparse.ArgumentParser(description="Darshana: Structural Document REPL")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # Scout
    p_scout = subparsers.add_parser("scout", help="Map the document topology")
    p_scout.add_argument("file", help="Path to markdown file")

    # Read
    p_read = subparsers.add_parser("read", help="Read specific section")
    p_read.add_argument("file", help="Path to markdown file")
    p_read.add_argument("--section", help="Header title substring")
    p_read.add_argument("--ql", help="QL Marker (e.g. P1)")

    # Threads
    p_threads = subparsers.add_parser("threads", help="Extract wikilinks")
    p_threads.add_argument("file", help="Path to markdown file")

    args = parser.parse_args()

    if args.command == "scout":
        print(json.dumps(scout(args.file), indent=2))
    elif args.command == "read":
        print(read_section(args.file, args.section, args.ql))
    elif args.command == "threads":
        threads = extract_threads(args.file)
        print(json.dumps(threads, indent=2))

if __name__ == "__main__":
    main()
