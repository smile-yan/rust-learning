#!/usr/bin/env bash
# 构建前端静态文件并打包为 tar.gz 产物
# 产出 dist-frontend.tar.gz，供 scripts/deploy-frontend.sh 上传部署
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${EVALUATE_URL:?missing EVALUATE_URL}"

VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "unknown")

echo "构建版本: $VERSION"

# 安装依赖并构建
npm ci
npm run build

# 把版本号与生产环境 evaluateUrl 注入到所有 HTML
# 注意：VitePress 构建会压缩 head 内联脚本，产物中是 evaluateUrl:"...（无空格），模式需容忍空格差异
find .vitepress/dist -name "*.html" | while read -r f; do
    sed -i.bak \
        -e "s|__VERSION__|$VERSION|g" \
        -e "s|evaluateUrl: *\"http://localhost:9001/evaluate.json\"|evaluateUrl: \"$EVALUATE_URL\"|g" \
        "$f"
    rm -f "$f.bak"
done

# 注入必须生效，否则线上「运行」按钮会指向 localhost，直接判失败
if ! grep -rq "evaluateUrl: *\"$EVALUATE_URL\"" .vitepress/dist --include="*.html"; then
    echo "ERROR: evaluateUrl 注入失败，产物中未找到 $EVALUATE_URL" >&2
    exit 1
fi
# 打包前产物中不应再残留 localhost 地址
if grep -rlq "localhost:9001/evaluate.json" .vitepress/dist --include="*.html"; then
    echo "ERROR: 产物中仍残留 localhost evaluateUrl" >&2
    exit 1
fi

# 打包产物（排除 macOS 的 .DS_Store，避免污染部署目录）
tar -czf dist-frontend.tar.gz --exclude='.DS_Store' -C .vitepress/dist .
echo "构建产物: dist-frontend.tar.gz"
