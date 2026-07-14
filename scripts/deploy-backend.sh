#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${BACKEND_HOST:?missing BACKEND_HOST}"
: "${BACKEND_USER:?missing BACKEND_USER}"
: "${BACKEND_DEPLOY_DIR:?missing BACKEND_DEPLOY_DIR}"
: "${BACKEND_SERVICE:?missing BACKEND_SERVICE}"
: "${BACKEND_PORT:?missing BACKEND_PORT}"

mkdir -p ~/.ssh
chmod 700 ~/.ssh
printf '%s\n' "$SSH_PRIVATE_KEY" > ~/.ssh/deploy_key
chmod 600 ~/.ssh/deploy_key

cat >> ~/.ssh/config <<EOF
Host backend-deploy
    HostName ${BACKEND_HOST}
    User ${BACKEND_USER}
    IdentityFile ~/.ssh/deploy_key
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
EOF
chmod 600 ~/.ssh/config

# 上传源码到后端服务器
rsync -avz --delete \
  --exclude='.git/' \
  --exclude='backend/target/' \
  --exclude='node_modules/' \
  . backend-deploy:"$BACKEND_DEPLOY_DIR/"

# 编译并重启
ssh backend-deploy \
  "export PORT='${BACKEND_PORT}' \
   STATIC_DIR='${BACKEND_DEPLOY_DIR}' \
   CONCURRENCY='${CONCURRENCY:-4}' \
   TIMEOUT_SECONDS='${TIMEOUT_SECONDS:-120}' \
   MEMORY_LIMIT_MB='${MEMORY_LIMIT_MB:-512}' \
   DOCKER_IMAGE='${DOCKER_IMAGE:-rust-learning-playground:1.86}' \
   && cd '${BACKEND_DEPLOY_DIR}' \
   && cargo build --release \
   && sudo systemctl restart '${BACKEND_SERVICE}' \
   && sudo systemctl status '${BACKEND_SERVICE}' --no-pager \
   && sleep 2 \
   && curl -fsS http://localhost:${BACKEND_PORT}/ > /dev/null"

echo "后端部署完成"
