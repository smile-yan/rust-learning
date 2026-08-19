#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${FRONTEND_HOST:?missing FRONTEND_HOST}"
: "${FRONTEND_USER:?missing FRONTEND_USER}"
: "${FRONTEND_WEB_ROOT:?missing FRONTEND_WEB_ROOT}"
: "${EVALUATE_URL:?missing EVALUATE_URL}"

VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "unknown")

echo "部署版本: $VERSION"

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
# 上传前产物中不应再残留 localhost 地址
if grep -rlq "localhost:9001/evaluate.json" .vitepress/dist --include="*.html"; then
    echo "ERROR: 产物中仍残留 localhost evaluateUrl" >&2
    exit 1
fi

mkdir -p ~/.ssh
chmod 700 ~/.ssh
printf '%s\n' "$SSH_PRIVATE_KEY" > ~/.ssh/deploy_key
chmod 600 ~/.ssh/deploy_key

cat >> ~/.ssh/config <<EOF
Host frontend-deploy
    HostName ${FRONTEND_HOST}
    User ${FRONTEND_USER}
    IdentityFile ~/.ssh/deploy_key
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
EOF
chmod 600 ~/.ssh/config

# 上传 .vitepress/dist/ 目录（使用 scp，不依赖服务器端 rsync），带超时和重试
upload_files() {
    scp -o ConnectTimeout=30 -o ServerAliveInterval=30 -o ServerAliveCountMax=3 \
        -r .vitepress/dist/* frontend-deploy:"$FRONTEND_WEB_ROOT/"
}

retry=0
max_retry=3
while true; do
    if upload_files; then
        break
    fi
    retry=$((retry + 1))
    if [ "$retry" -ge "$max_retry" ]; then
        echo "ERROR: 上传静态文件失败，已重试 $max_retry 次" >&2
        exit 1
    fi
    echo "上传失败，5 秒后第 $retry 次重试..."
    sleep 5
done

# 验证部署
for i in 1 2 3; do
    if curl -fsS --max-time 30 "http://${FRONTEND_HOST}/" > /dev/null; then
        echo "前端部署完成"
        exit 0
    fi
    echo "首页访问校验失败，第 $i 次重试..."
    sleep 5
done

echo "ERROR: 部署后首页不可访问" >&2
exit 1
