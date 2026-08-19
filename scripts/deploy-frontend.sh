#!/usr/bin/env bash
# 上传并部署 scripts/build-frontend.sh 产出的 tar.gz 静态文件包
# 用法: scripts/deploy-frontend.sh [产物路径，默认 dist-frontend.tar.gz]
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${FRONTEND_HOST:?missing FRONTEND_HOST}"
: "${FRONTEND_USER:?missing FRONTEND_USER}"
: "${FRONTEND_WEB_ROOT:?missing FRONTEND_WEB_ROOT}"

ARCHIVE="${1:-dist-frontend.tar.gz}"
if [ ! -f "$ARCHIVE" ]; then
    echo "ERROR: 未找到构建产物 $ARCHIVE，请先运行 scripts/build-frontend.sh" >&2
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

# 上传部署：压缩包经 SSH 流式传输，远端清空 web 根目录后解包
# 解包前清空是为了避免旧部署残留（如已删除的 app.html、旧哈希 chunk）一直留在线上
upload_files() {
    ssh -o ConnectTimeout=30 -o ServerAliveInterval=30 -o ServerAliveCountMax=3 \
        frontend-deploy "
            set -e
            find \"$FRONTEND_WEB_ROOT\" -mindepth 1 -delete
            tar -xzf - -C \"$FRONTEND_WEB_ROOT\"
        " < "$ARCHIVE"
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
