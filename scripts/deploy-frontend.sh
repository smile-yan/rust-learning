#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${FRONTEND_HOST:?missing FRONTEND_HOST}"
: "${FRONTEND_USER:?missing FRONTEND_USER}"
: "${FRONTEND_WEB_ROOT:?missing FRONTEND_WEB_ROOT}"
: "${EVALUATE_URL:?missing EVALUATE_URL}"

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

# 替换生产环境 evaluateUrl
sed -i.bak "s|evaluateUrl: \"http://localhost:9001/evaluate.json\"|evaluateUrl: \"$EVALUATE_URL\"|" index.html
rm -f index.html.bak

# 上传静态文件
rsync -avz --delete \
  index.html css/ js/ libs/ images/ \
  frontend-deploy:"$FRONTEND_WEB_ROOT/"

curl -fsS "http://${FRONTEND_HOST}/" > /dev/null

echo "前端部署完成"
