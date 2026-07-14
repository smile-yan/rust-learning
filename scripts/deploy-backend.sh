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

# 打包源码（用于静态文件和后端源码）
tar czf /tmp/backend.tar.gz \
  --exclude='.git' \
  --exclude='backend/target' \
  --exclude='node_modules' \
  .

echo "上传源码压缩包到服务器..."
scp -o ConnectTimeout=30 -o BatchMode=yes /tmp/backend.tar.gz backend-deploy:/tmp/backend.tar.gz

# 如果存在预编译二进制（GitHub Actions 构建），一并上传
if [ -d /tmp/backend-release ] && [ -f /tmp/backend-release/rust-learning-backend ] && [ -f /tmp/backend-release/docker-runner ]; then
  echo "打包预编译二进制..."
  tar czf /tmp/backend-binaries.tar.gz -C /tmp/backend-release rust-learning-backend docker-runner
  echo "上传预编译二进制到服务器..."
  scp -o ConnectTimeout=30 -o BatchMode=yes /tmp/backend-binaries.tar.gz backend-deploy:/tmp/backend-binaries.tar.gz
  PREBUILT_BINARIES=1
else
  echo "未找到预编译二进制，将回退到服务器端 cargo build"
  PREBUILT_BINARIES=0
fi

# 生成并上传 systemd 服务文件
echo "生成 systemd 服务文件..."
cat > "/tmp/${SERVICE_FILE}" <<EOF
[Unit]
Description=Rust Learning Backend
After=network.target

[Service]
Type=simple
User=${BACKEND_USER}
WorkingDirectory=${BACKEND_DEPLOY_DIR}
ExecStart=${BACKEND_DEPLOY_DIR}/backend/target/release/rust-learning-backend
Restart=always
RestartSec=5
StartLimitIntervalSec=0

Environment="PORT=${BACKEND_PORT}"
Environment="STATIC_DIR=${BACKEND_DEPLOY_DIR}"
Environment="CONCURRENCY=${CONCURRENCY:-4}"
Environment="TIMEOUT_SECONDS=${TIMEOUT_SECONDS:-120}"
Environment="MEMORY_LIMIT_MB=${MEMORY_LIMIT_MB:-512}"
Environment="DOCKER_IMAGE=${DOCKER_IMAGE:-rust-learning-playground:1.86}"

[Install]
WantedBy=multi-user.target
EOF

echo "上传服务文件..."
scp -o ConnectTimeout=30 -o BatchMode=yes "/tmp/${SERVICE_FILE}" backend-deploy:/tmp/${SERVICE_FILE}

# 解压、编译、重启
echo "连接服务器执行部署脚本..."
ssh -o ConnectTimeout=30 -o BatchMode=yes backend-deploy \
  "set -e; \
   echo '创建后端目录...'; \
   mkdir -p '${BACKEND_DEPLOY_DIR}/backend/target/release'; \
   echo '解压源码...'; \
   tar xzf /tmp/backend.tar.gz -C '${BACKEND_DEPLOY_DIR}'; \
   rm -f /tmp/backend.tar.gz; \
   if [ '${PREBUILT_BINARIES}' = '1' ]; then \
     echo '使用 GitHub Actions 预编译的二进制...'; \
     tar xzf /tmp/backend-binaries.tar.gz -C '${BACKEND_DEPLOY_DIR}/backend/target/release'; \
     rm -f /tmp/backend-binaries.tar.gz; \
     ls -lh '${BACKEND_DEPLOY_DIR}/backend/target/release'; \
   fi; \
   export PORT='${BACKEND_PORT}'; \
   export STATIC_DIR='${BACKEND_DEPLOY_DIR}'; \
   export CONCURRENCY='${CONCURRENCY:-4}'; \
   export TIMEOUT_SECONDS='${TIMEOUT_SECONDS:-120}'; \
   export MEMORY_LIMIT_MB='${MEMORY_LIMIT_MB:-512}'; \
   export DOCKER_IMAGE='${DOCKER_IMAGE:-rust-learning-playground:1.86}'; \
   if [ '${PREBUILT_BINARIES}' = '0' ]; then \
     if ! command -v cargo >/dev/null 2>&1; then \
       echo '错误：服务器上未找到 cargo，请先安装 Rust 工具链。' >&2; \
       exit 1; \
     fi; \
     . "\$HOME/.cargo/env"; \
     mkdir -p "\$HOME/.cargo"; \
     printf '%s\n' '[source.crates-io]' 'replace-with = \"ustc\"' '' '[source.ustc]' 'registry = \"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\"' > "\$HOME/.cargo/config.toml"; \
     echo '已配置 USTC cargo sparse 镜像'; \
     cd '${BACKEND_DEPLOY_DIR}/backend'; \
     cargo build --release; \
   fi; \
   echo '检查 Docker 守护进程...'; \
   timeout 10 docker info >/dev/null; \
   image='${DOCKER_IMAGE:-rust-learning-playground:1.86}'; \
   if ! docker image inspect "\$image" >/dev/null 2>&1; then \
     echo "Docker 镜像 \$image 不存在，开始从仓库拉取..."; \
     timeout 300 docker pull "\$image"; \
   fi; \
   if ! sudo systemctl cat '${SERVICE_NAME}' >/dev/null 2>&1; then \
     echo '正在安装 systemd 服务...'; \
     sudo mv /tmp/${SERVICE_FILE} /etc/systemd/system/${SERVICE_FILE}; \
     sudo systemctl daemon-reload; \
     sudo systemctl enable '${SERVICE_NAME}'; \
   fi; \
   echo '重启后端服务...'; \
   sudo systemctl restart '${SERVICE_NAME}'; \
   sleep 2; \
   for i in 1 2 3 4 5; do \
     if curl -fsS --max-time 5 http://localhost:${BACKEND_PORT}/ > /dev/null 2>&1; then \
       echo '后端健康检查通过'; \
       break; \
     fi; \
     echo "健康检查 \$i/5 失败，等待..."; \
     sleep 3; \
   done; \
   if ! curl -fsS --max-time 5 http://localhost:${BACKEND_PORT}/ > /dev/null 2>&1; then \
     echo '后端健康检查失败，服务状态和日志如下：'; \
     sudo systemctl status '${SERVICE_NAME}' --no-pager -l || true; \
     sudo journalctl -u '${SERVICE_NAME}' -n 50 --no-pager || true; \
     exit 1; \
   fi; \
   sudo systemctl status '${SERVICE_NAME}' --no-pager; "

echo "后端部署完成"
