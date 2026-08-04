#!/usr/bin/env python3
"""
根据最新 git tag 统一更新静态资源缓存版本号。

会扫描以下文件中的 v0.0.0-dev 占位符，并替换为最新 tag：
- index.html
- app.html
- js/app.js

用法：
    python3 scripts/bump_version.py
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = [
    ROOT / "index.html",
    ROOT / "app.html",
    ROOT / "js" / "app.js",
]
PLACEHOLDER = "__VERSION__"


def get_latest_tag() -> str:
    try:
        result = subprocess.run(
            ["git", "describe", "--tags", "--abbrev=0"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError:
        print("错误：无法获取最新 git tag，请确保仓库已有 tag。", file=sys.stderr)
        sys.exit(1)


def bump_file(path: Path, version: str) -> int:
    text = path.read_text(encoding="utf-8")
    new_text, count = re.subn(re.escape(PLACEHOLDER), version, text)
    if count:
        path.write_text(new_text, encoding="utf-8")
    return count


def main():
    version = get_latest_tag()
    print(f"最新 git tag: {version}")

    total = 0
    for path in FILES:
        if not path.exists():
            print(f"警告：文件不存在，跳过 {path}")
            continue
        count = bump_file(path, version)
        total += count
        print(f"  {path.relative_to(ROOT)}: 替换 {count} 处")

    print(f"\n共替换 {total} 处，缓存版本号已更新为 {version}")


if __name__ == "__main__":
    main()
