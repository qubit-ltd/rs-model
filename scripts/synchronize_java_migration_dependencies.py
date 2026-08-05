#!/usr/bin/env python3
"""Synchronize inventory dependencies from Java imports in the migration scope."""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path


IMPORT_PATTERN = re.compile(
    r"^\s*import\s+(?P<static>static\s+)?(?P<target>[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)*)\s*;",
    re.MULTILINE,
)
PACKAGE_PATTERN = re.compile(r"^\s*package\s+(?P<package>[\w.]+)\s*;", re.MULTILINE)
INVENTORY_ROW_PATTERN = re.compile(r"^\| `(?P<fqcn>[^`]+)` \|.*\|.*\|.*\|.*\|$")
PUBLIC_TYPE_PATTERN = re.compile(
    r"\bpublic\s+(?:(?:abstract|final|non-sealed|sealed|static|strictfp)\s+)*"
    r"(?:class|enum|interface|record|@interface)\s+(?P<name>[A-Za-z_]\w*)"
)
JAVA_STRUCTURE_PATTERN = re.compile(
    rf"(?P<declaration>{PUBLIC_TYPE_PATTERN.pattern})|(?P<open>\{{)|(?P<close>\}})"
)


def java_code_only(source: str) -> str:
    """Replace Java comments and literal bodies with spaces while preserving newlines."""
    output: list[str] = []
    index = 0
    state = "code"
    while index < len(source):
        character = source[index]
        next_character = source[index + 1] if index + 1 < len(source) else ""
        next_three = source[index : index + 3]
        if state == "code":
            if next_three == '\"\"\"':
                output.extend("   ")
                index += 3
                state = "text_block"
            elif character == '"':
                output.append(" ")
                index += 1
                state = "string"
            elif character == "'":
                output.append(" ")
                index += 1
                state = "character"
            elif character == "/" and next_character == "/":
                output.extend("  ")
                index += 2
                state = "line_comment"
            elif character == "/" and next_character == "*":
                output.extend("  ")
                index += 2
                state = "block_comment"
            else:
                output.append(character)
                index += 1
        elif state == "line_comment":
            output.append("\n" if character == "\n" else " ")
            index += 1
            if character == "\n":
                state = "code"
        elif state == "block_comment":
            if character == "*" and next_character == "/":
                output.extend("  ")
                index += 2
                state = "code"
            else:
                output.append("\n" if character == "\n" else " ")
                index += 1
        elif state == "text_block":
            if next_three == '\"\"\"':
                output.extend("   ")
                index += 3
                state = "code"
            else:
                output.append("\n" if character == "\n" else " ")
                index += 1
        else:
            if character == "\\":
                output.append(" ")
                index += 1
                if index < len(source):
                    escaped = source[index]
                    output.append("\n" if escaped == "\n" else " ")
                    index += 1
            elif (state == "string" and character == '"') or (
                state == "character" and character == "'"
            ):
                output.append(" ")
                index += 1
                state = "code"
            else:
                output.append("\n" if character == "\n" else " ")
                index += 1
    return "".join(output)


def parse_arguments() -> argparse.Namespace:
    """Parse the Java source roots and inventory path to synchronize."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--common-mixin-source", required=True, type=Path)
    parser.add_argument("--common-model-source", required=True, type=Path)
    parser.add_argument("--inventory", required=True, type=Path)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if synchronizing dependencies would change the inventory",
    )
    return parser.parse_args()


def inventory_fqcns(inventory: str) -> set[str]:
    """Return every Java FQCN recorded in the inventory table."""
    return {
        match.group("fqcn")
        for line in inventory.splitlines()
        if (match := INVENTORY_ROW_PATTERN.match(line)) is not None
    }


def public_declarations(
    source_roots: list[Path],
) -> tuple[dict[str, Path], set[str]]:
    """Extract all public top-level and nested Java declarations from the source roots."""
    declarations: dict[str, Path] = {}
    top_level_declarations: set[str] = set()
    for source_root in source_roots:
        if not source_root.is_dir():
            raise FileNotFoundError(f"Java source root does not exist: {source_root}")
        for path in source_root.rglob("*.java"):
            contents = path.read_text(encoding="utf-8")
            match = PACKAGE_PATTERN.search(contents)
            if match is None:
                continue
            package = match.group("package")
            nesting: list[tuple[str, int]] = []
            pending_type: str | None = None
            brace_depth = 0
            for token in JAVA_STRUCTURE_PATTERN.finditer(java_code_only(contents)):
                if token.group("declaration") is not None:
                    pending_type = token.group("name")
                elif token.group("open") is not None:
                    brace_depth += 1
                    if pending_type is not None:
                        fqcn = ".".join([package, *(name for name, _ in nesting), pending_type])
                        if fqcn in declarations:
                            raise ValueError(f"duplicate public declaration: {fqcn}")
                        declarations[fqcn] = path
                        if not nesting:
                            top_level_declarations.add(fqcn)
                        nesting.append((pending_type, brace_depth))
                        pending_type = None
                elif token.group("close") is not None:
                    while nesting and nesting[-1][1] == brace_depth:
                        nesting.pop()
                    brace_depth -= 1
    return declarations, top_level_declarations


def owning_source_type(target: str, source_fqcns: set[str]) -> str | None:
    """Resolve an ordinary or static import to its imported source declaration."""
    candidate = target
    while "." in candidate:
        if candidate in source_fqcns:
            return candidate
        candidate = candidate.rsplit(".", maxsplit=1)[0]
    return candidate if candidate in source_fqcns else None


def dependencies_for_source(path: Path, source_fqcns: set[str]) -> list[str]:
    """Return ordered, unique in-scope type dependencies imported by one Java file."""
    dependencies: list[str] = []
    for match in IMPORT_PATTERN.finditer(path.read_text(encoding="utf-8")):
        dependency = owning_source_type(match.group("target"), source_fqcns)
        if dependency is not None and dependency not in dependencies:
            dependencies.append(dependency)
    return dependencies


def replace_dependency_column(
    inventory: str, source_by_fqcn: dict[str, Path], source_fqcns: set[str]
) -> str:
    """Replace every table dependency cell with dependencies scanned from Java imports."""
    rows: list[str] = []
    for line in inventory.splitlines(keepends=True):
        match = INVENTORY_ROW_PATTERN.match(line.rstrip("\n"))
        if match is None or (path := source_by_fqcn.get(match.group("fqcn"))) is None:
            rows.append(line)
            continue
        cells = line.rstrip("\n").split("|")
        dependencies = dependencies_for_source(path, source_fqcns)
        cells[4] = f" {', '.join(dependencies) if dependencies else '-'} "
        rows.append("|".join(cells) + ("\n" if line.endswith("\n") else ""))
    return "".join(rows)


def declaration_drift(inventory_fqcns: set[str], source_fqcns: set[str]) -> str:
    """Describe public source declarations absent from either side of the inventory."""
    diagnostics: list[str] = []
    missing_from_inventory = sorted(source_fqcns - inventory_fqcns)
    missing_from_source = sorted(inventory_fqcns - source_fqcns)
    if missing_from_inventory:
        diagnostics.append(
            "public declarations missing from inventory:\n"
            + "".join(f"  {fqcn}\n" for fqcn in missing_from_inventory)
        )
    if missing_from_source:
        diagnostics.append(
            "inventory declarations missing from source:\n"
            + "".join(f"  {fqcn}\n" for fqcn in missing_from_source)
        )
    return "".join(diagnostics)


def main() -> int:
    """Synchronize the inventory or report the precise dependency-table diff."""
    arguments = parse_arguments()
    inventory = arguments.inventory.read_text(encoding="utf-8")
    recorded_fqcns = inventory_fqcns(inventory)
    source_by_fqcn, top_level_fqcns = public_declarations(
        [arguments.common_mixin_source, arguments.common_model_source]
    )
    source_fqcns = set(source_by_fqcn)
    if diagnostics := declaration_drift(recorded_fqcns, source_fqcns):
        print(diagnostics, end="", file=sys.stderr)
        return 1
    synchronized = replace_dependency_column(
        inventory,
        {fqcn: source_by_fqcn[fqcn] for fqcn in top_level_fqcns},
        source_fqcns,
    )
    if arguments.check:
        if synchronized == inventory:
            return 0
        print(
            "".join(
                difflib.unified_diff(
                    inventory.splitlines(keepends=True),
                    synchronized.splitlines(keepends=True),
                    fromfile=str(arguments.inventory),
                    tofile=f"{arguments.inventory} (synchronized)",
                )
            ),
            end="",
        )
        return 1
    arguments.inventory.write_text(synchronized, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
