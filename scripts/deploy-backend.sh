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

SERVICE_NAME="${BACKEND_SERVICE%.service}"
SERVICE_FILE="${SERVICE_NAME}.service"

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
    ServerAliveInterval 30
    ServerAliveCountMax 10
EOF
chmod 600 ~/.ssh/config

# 打包并上传源码到后端服务器（不依赖服务器端 rsync）
tar czf /tmp/backend.tar.gz \
  --exclude='.git' \
  --exclude='backend/target' \
  --exclude='node_modules' \
  .

scp /tmp/backend.tar.gz backend-deploy:/tmp/backend.tar.gz

# 生成并上传 systemd 服务文件
cat > "/tmp/${SERVICE_FILE}" <<EOF
[Unit]
Description=Rust Learning Backend
After=network.target

[Service]
Type=simple
User=${BACKEND_USER}
WorkingDirectory=${BACKEND_DEPLOY_DIR}
ExecStart=${BACKEND_DEPLOY_DIR}/backend/target/release/rust-learning-backend
Restart=on-failure
RestartSec=5

Environment="PORT=${BACKEND_PORT}"
Environment="STATIC_DIR=${BACKEND_DEPLOY_DIR}"
Environment="CONCURRENCY=${CONCURRENCY:-4}"
Environment="TIMEOUT_SECONDS=${TIMEOUT_SECONDS:-120}"
Environment="MEMORY_LIMIT_MB=${MEMORY_LIMIT_MB:-512}"
Environment="DOCKER_IMAGE=${DOCKER_IMAGE:-rust-learning-playground:1.86}"

[Install]
WantedBy=multi-user.target
EOF

scp "/tmp/${SERVICE_FILE}" backend-deploy:/tmp/${SERVICE_FILE}

# 解压、编译、重启
ssh backend-deploy \
  "set -e; \
   mkdir -p '${BACKEND_DEPLOY_DIR}'; \
   tar xzf /tmp/backend.tar.gz -C '${BACKEND_DEPLOY_DIR}'; \
   rm -f /tmp/backend.tar.gz; \
   export PORT='${BACKEND_PORT}'; \
   export STATIC_DIR='${BACKEND_DEPLOY_DIR}'; \
   export CONCURRENCY='${CONCURRENCY:-4}'; \
   export TIMEOUT_SECONDS='${TIMEOUT_SECONDS:-120}'; \
   export MEMORY_LIMIT_MB='${MEMORY_LIMIT_MB:-512}'; \
   export DOCKER_IMAGE='${DOCKER_IMAGE:-rust-learning-playground:1.86}'; \
   if ! command -v cargo >/dev/null 2>&1; then \
     echo '错误：服务器上未找到 cargo，请先安装 Rust 工具链。' >&2; \
     exit 1; \
   fi; \
   . "\$HOME/.cargo/env"; \
   if [ ! -f "\$HOME/.cargo/config.toml" ]; then \
     mkdir -p "\$HOME/.cargo"; \
     printf '%s\n' '[source.crates-io]' 'replace-with = \"ustc\"' '' '[source.ustc]' 'registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"' > "\$HOME/.cargo/config.toml"; \
     echo '已配置 USTC cargo 镜像，后续下载会更快'; \
   fi; \
   cd '${BACKEND_DEPLOY_DIR}/backend'; \
   cargo build --release; \
   if ! sudo systemctl cat '${SERVICE_NAME}' >/dev/null 2>&1; then \
     echo '正在安装 systemd 服务...'; \
     sudo mv /tmp/${SERVICE_FILE} /etc/systemd/system/${SERVICE_FILE}; \
     sudo systemctl daemon-reload; \
     sudo systemctl enable '${SERVICE_NAME}'; \
   fi; \
   sudo systemctl restart '${SERVICE_NAME}'; \
   sudo systemctl status '${SERVICE_NAME}' --no-pager; \
   sleep 2; \
   curl -fsS http://localhost:${BACKEND_PORT}/ > /dev/null"

echo "后端部署完成"
