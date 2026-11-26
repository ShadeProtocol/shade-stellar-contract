#!/usr/bin/env python3
"""Check reference coverage without dependencies or a Rust build.

Error enums must use unit variants with literal integer discriminants or implicit
successors. Unsupported expressions fail explicitly rather than being skipped.
"""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]


def source_text(text):
    """Remove strings and comments, including nested Rust block comments."""
    end = 0
    parts = []
    pattern = re.compile(r'/\*|//[^\n]*|"(?:\\.|[^"\\])*"')
    while match := pattern.search(text, end):
        parts.append(text[end:match.start()])
        token = match.group()
        if token == '/*':
            depth = 1
            cursor = match.end()
            while depth:
                delimiter = re.search(r'/\*|\*/', text[cursor:])
                if delimiter is None:
                    raise ValueError('unterminated block comment')
                depth += 1 if delimiter.group() == '/*' else -1
                cursor += delimiter.end()
            token = text[match.start():cursor]
            end = cursor
        else:
            end = match.end()
        parts.append('\n' * token.count('\n') + ' ')
    return ''.join(parts) + text[end:]


def section(text, heading_pattern, level):
    match = re.search(heading_pattern, text, re.M)
    if not match:
        return ''
    rest = text[match.end():]
    boundary = re.search(r'^#{1,' + str(level) + r'}\s', rest, re.M)
    return rest[:boundary.start()] if boundary else rest


def reference_section(text, path):
    text = re.sub(r'<!--.*?-->|^```[^\n]*\n.*?^```[^\n]*$', '', text,
                  flags=re.S | re.M)
    return section(text, r'^## .*`' + re.escape(path) + r'`.*$', 2)


def error_variants(body):
    body = re.sub(r'#\[[^\]]*\]', '', body)
    code = -1
    result = []
    for entry in body.split(','):
        if not entry.strip():
            continue
        match = re.fullmatch(r'\s*(\w+)(?:\s*=\s*(0[xX][\da-fA-F_]+|'
                             r'0[bB][01_]+|0[oO][0-7_]+|[\d_]+)'
                             r'(?:u32)?)?\s*', entry)
        if not match:
            raise ValueError(f'unsupported error variant: {entry.strip()}')
        name, literal = match.groups()
        if literal:
            literal = literal.replace('_', '')
            base = 0 if literal.lower().startswith(('0x', '0b', '0o')) else 10
            code = int(literal, base)
        else:
            code += 1
        result.append((name, code))
    return result


def check(root):
    problems = []
    counts = {'events': 0, 'errors': 0}
    for kind in counts:
        page = root / 'docs/reference' / f'{kind}.md'
        docs = page.read_text(encoding='utf-8')
        paths = sorted(root.glob(f'contracts/*/src/{kind}.rs'))
        if not paths:
            problems.append(f'No contracts/*/src/{kind}.rs files found')
        for path in paths:
            relative = path.relative_to(root).as_posix()
            text = source_text(path.read_text(encoding='utf-8'))
            documented = reference_section(docs, relative)
            if kind == 'events':
                for name in re.findall(r'\bfn\s+(publish_\w+_event)\s*\(', text):
                    counts[kind] += 1
                    if not re.search(r'^\|\s*`' + re.escape(name) + r'`\s*\|',
                                     documented, re.M):
                        problems.append(f'{relative}: missing event helper {name} in {page.name}')
            else:
                enums = re.findall(r'\benum\s+(\w+)\s*\{([^{}]*)\}', text, re.S)
                declarations = re.findall(r'\benum\s+(\w+)', text)
                if not enums or len(enums) != len(declarations):
                    problems.append(f'{relative}: missing or unsupported error enum body')
                for enum, body in enums:
                    rows = section(documented, r'^### `' + re.escape(enum) + r'`\s*$', 3)
                    try:
                        variants = error_variants(body)
                    except ValueError as error:
                        problems.append(f'{relative}::{enum}: {error}')
                        continue
                    for name, code in variants:
                        counts[kind] += 1
                        values = re.findall(r'^\|\s*`' + re.escape(name) +
                                            r'`\s*\|\s*([^|]+)\|', rows, re.M)
                        if not values:
                            problems.append(f'{relative}::{enum}::{name}: missing error entry in {page.name}')
                        elif len(values) != 1 or values[0].strip() != str(code):
                            problems.append(f'{relative}::{enum}::{name}: expected code {code}, '
                                            f'documented {", ".join(v.strip() for v in values)}')
    return problems, counts


def main():
    try:
        problems, counts = check(ROOT)
    except (OSError, ValueError) as error:
        print(f'Reference check failed: {error}', file=sys.stderr)
        return 1
    if problems:
        print('Reference documentation is incomplete or inconsistent:', file=sys.stderr)
        for problem in problems:
            print(f'  - {problem}', file=sys.stderr)
        return 1
    print(f'Reference docs OK: {counts["events"]} event helpers, {counts["errors"]} error variants')
    return 0


if __name__ == '__main__':
    sys.exit(main())
