#!/usr/bin/env python3
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHAPTERS = ROOT / "js" / "chapters.json"


def load_json():
    with CHAPTERS.open("r", encoding="utf-8") as f:
        return json.load(f)


def count_chapters(modules):
    return sum(len(m["chapters"]) for m in modules)


def check_image_refs(modules):
    refs = set()
    for m in modules:
        for c in m["chapters"]:
            refs.update(re.findall(r"!\[.*?\]\((images/[^)]+)\)", c.get("theory", "")))
    missing = [r for r in refs if not (ROOT / r).exists()]
    if missing:
        print("Missing images:", missing)
        return False
    print(f"Image refs OK: {len(refs)} refs, all files exist")
    return True


def check_length(modules):
    failures = []
    for mi, m in enumerate(modules):
        for ci, c in enumerate(m["chapters"]):
            theory = c.get("theory", "")
            if len(theory) < 300:
                failures.append(f"module {mi} chapter {ci} too short ({len(theory)} chars)")
    if failures:
        print("Length failures:", failures)
        return False
    print("Length check OK")
    return True


def main():
    try:
        modules = load_json()
    except json.JSONDecodeError as e:
        print("JSON parse error:", e)
        sys.exit(1)

    print(f"Total chapters: {count_chapters(modules)}")
    ok = check_image_refs(modules) and check_length(modules)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
