#!/usr/bin/env python3
"""Synchronize inventory dependencies from Java imports in the migration scope."""

from __future__ import annotations

import argparse
import difflib
import re
from pathlib import Path


IMPORT_PATTERN = re.compile(
    r"^\s*import\s+(?P<static>static\s+)?(?P<target>[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)*)\s*;",
    re.MULTILINE,
)
PACKAGE_PATTERN = re.compile(r"^\s*package\s+(?P<package>[\w.]+)\s*;", re.MULTILINE)
INVENTORY_ROW_PATTERN = re.compile(r"^\| `(?P<fqcn>[^`]+)` \|.*\|.*\|.*\|.*\|$")


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


def source_files(source_roots: list[Path], known_fqcns: set[str]) -> dict[str, Path]:
    """Map each source-inventory top-level FQCN to its Java source file."""
    files: dict[str, Path] = {}
    for source_root in source_roots:
        if not source_root.is_dir():
            raise FileNotFoundError(f"Java source root does not exist: {source_root}")
        for path in source_root.rglob("*.java"):
            contents = path.read_text(encoding="utf-8")
            match = PACKAGE_PATTERN.search(contents)
            if match is None:
                continue
            fqcn = f"{match.group('package')}.{path.stem}"
            if fqcn in known_fqcns:
                files[fqcn] = path
    return files


def owning_inventory_type(target: str, known_fqcns: set[str]) -> str | None:
    """Resolve an ordinary or static import to its imported inventory type."""
    candidate = target
    while "." in candidate:
        if candidate in known_fqcns:
            return candidate
        candidate = candidate.rsplit(".", maxsplit=1)[0]
    return candidate if candidate in known_fqcns else None


def dependencies_for_source(path: Path, known_fqcns: set[str]) -> list[str]:
    """Return ordered, unique in-scope type dependencies imported by one Java file."""
    dependencies: list[str] = []
    for match in IMPORT_PATTERN.finditer(path.read_text(encoding="utf-8")):
        dependency = owning_inventory_type(match.group("target"), known_fqcns)
        if dependency is not None and dependency not in dependencies:
            dependencies.append(dependency)
    return dependencies


def replace_dependency_column(
    inventory: str, source_by_fqcn: dict[str, Path], known_fqcns: set[str]
) -> str:
    """Replace every table dependency cell with dependencies scanned from Java imports."""
    rows: list[str] = []
    for line in inventory.splitlines(keepends=True):
        match = INVENTORY_ROW_PATTERN.match(line.rstrip("\n"))
        if match is None or (path := source_by_fqcn.get(match.group("fqcn"))) is None:
            rows.append(line)
            continue
        cells = line.rstrip("\n").split("|")
        dependencies = dependencies_for_source(path, known_fqcns)
        cells[4] = f" {', '.join(dependencies) if dependencies else '-'} "
        rows.append("|".join(cells) + ("\n" if line.endswith("\n") else ""))
    return "".join(rows)


def main() -> int:
    """Synchronize the inventory or report the precise dependency-table diff."""
    arguments = parse_arguments()
    inventory = arguments.inventory.read_text(encoding="utf-8")
    known_fqcns = inventory_fqcns(inventory)
    source_by_fqcn = source_files(
        [arguments.common_mixin_source, arguments.common_model_source], known_fqcns
    )
    synchronized = replace_dependency_column(inventory, source_by_fqcn, known_fqcns)
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
