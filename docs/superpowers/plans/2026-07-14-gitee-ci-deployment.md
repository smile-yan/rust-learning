# Gitee CI 自动部署实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现推送 `v*` tag 时自动部署前端静态文件到前端服务器、后端 Rust 服务到后端服务器。

**Architecture:** 使用 Gitee 共享 Runner 执行 `.gitee-ci.yml`，通过 SSH/SCP 完成前后端部署；后端在目标服务器上执行 `cargo build --release` 并由 systemd 管理。

**Tech Stack:** Gitee CI（GitLab 兼容语法）、Bash、systemd、OpenSSH

## Global Constraints

- 触发条件：仅 `v*` 开头的 tag。
- 不修改 `index.html` 仓库源文件（CI 运行时动态替换 `evaluateUrl`）。
- 不在仓库中硬编码服务器密钥和地址。
- 后端部署使用服务器本地编译（`cargo build --release`）。
- 共享 Runner 不依赖 Rust/Docker，只使用 `ssh`/`scp`。

---

## File Structure

| 文件 | 职责 |
|---|---|
| `.gitee-ci.yml` | Gitee CI 流水线主配置 |
| `scripts/deploy-frontend.sh` | 前端部署脚本（SSH 配置 + 替换 URL + scp） |
| `scripts/deploy-backend.sh` | 后端部署脚本（SSH 配置 + 上传源码 + 编译重启） |
| `systemd/rust-learning-backend.service` | 后端 systemd 服务文件模板 |
| `docs/superpowers/specs/2026-07-14-gitee-ci-deployment-design.md` | 已确认的设计文档 |

---

### Task 1: 创建前端部署脚本

**Files:**
- Create: `scripts/deploy-frontend.sh`

**Interfaces:**
- Consumes: Gitee CI 变量 `SSH_PRIVATE_KEY`、`FRONTEND_USER`、`FRONTEND_HOST`、`FRONTEND_WEB_ROOT`、`EVALUATE_URL`
- Produces: 前端文件被上传到前端服务器

- [ ] **Step 1: 编写脚本**

```bash
#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${FRONTEND_USER:?missing FRONTEND_USER}"
: "${FRONTEND_HOST:?missing FRONTEND_HOST}"
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

# 在生产环境的 index.html 中替换 evaluateUrl
sed -i.bak "s|evaluateUrl: \"http://localhost:9001/evaluate.json\"|evaluateUrl: \"$EVALUATE_URL\"|" index.html
rm -f index.html.bak

# 上传静态文件到前端服务器
rsync -avz --delete \
  index.html css/ js/ libs/ images/ \
  frontend-deploy:"$FRONTEND_WEB_ROOT/"

# 验证前端可访问
curl -fsS "http://${FRONTEND_HOST}/" > /dev/null

echo "前端部署完成"
```

- [ ] **Step 2: 赋予执行权限**

```bash
chmod +x /Users/yanshili/me/projects/rust-projects/scripts/deploy-frontend.sh
```

- [ ] **Step 3: Bash 语法检查**

```bash
bash -n /Users/yanshili/me/projects/rust-projects/scripts/deploy-frontend.sh
```

Expected: 无输出，退出码 0。

- [ ] **Step 4: Commit**

```bash
git add scripts/deploy-frontend.sh
git commit -m "feat: add frontend deployment script for Gitee CI"
```

---

### Task 2: 创建后端部署脚本

**Files:**
- Create: `scripts/deploy-backend.sh`

**Interfaces:**
- Consumes: Gitee CI 变量 `SSH_PRIVATE_KEY`、`BACKEND_USER`、`BACKEND_HOST`、`BACKEND_DEPLOY_DIR`、`BACKEND_SERVICE`、`BACKEND_PORT`，以及可选变量 `CONCURRENCY`、`TIMEOUT_SECONDS`、`MEMORY_LIMIT_MB`、`DOCKER_IMAGE`
- Produces: 后端源码被上传到后端服务器并编译重启

- [ ] **Step 1: 编写脚本**

```bash
#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

: "${SSH_PRIVATE_KEY:?missing SSH_PRIVATE_KEY}"
: "${BACKEND_USER:?missing BACKEND_USER}"
: "${BACKEND_HOST:?missing BACKEND_HOST}"
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

# 编译并重启后端服务
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
```

- [ ] **Step 2: 赋予执行权限**

```bash
chmod +x /Users/yanshili/me/projects/rust-projects/scripts/deploy-backend.sh
```

- [ ] **Step 3: Bash 语法检查**

```bash
bash -n /Users/yanshili/me/projects/rust-projects/scripts/deploy-backend.sh
```

Expected: 无输出，退出码 0。

- [ ] **Step 4: Commit**

```bash
git add scripts/deploy-backend.sh
git commit -m "feat: add backend deployment script for Gitee CI"
```

---

### Task 3: 创建后端 systemd 服务文件模板

**Files:**
- Create: `systemd/rust-learning-backend.service`

**Interfaces:**
- Consumes: 后端部署目录、端口、服务名
- Produces: 可被复制到 `/etc/systemd/system/` 的 systemd 单元文件

- [ ] **Step 1: 编写服务文件**

```ini
[Unit]
Description=Rust Learning Backend
After=network.target

[Service]
Type=simple
User=deploy
WorkingDirectory=/opt/rust-learning
ExecStart=/opt/rust-learning/backend/target/release/rust-learning-backend
Restart=on-failure
RestartSec=5

Environment="PORT=9001"
Environment="STATIC_DIR=/opt/rust-learning"
Environment="CONCURRENCY=4"
Environment="TIMEOUT_SECONDS=120"
Environment="MEMORY_LIMIT_MB=512"
Environment="DOCKER_IMAGE=rust-learning-playground:1.86"

[Install]
WantedBy=multi-user.target
```

- [ ] **Step 2: Commit**

```bash
git add systemd/rust-learning-backend.service
git commit -m "chore: add systemd service template for backend"
```

---

### Task 4: 创建 Gitee CI 流水线配置

**Files:**
- Create: `.gitee-ci.yml`

**Interfaces:**
- Consumes: `scripts/deploy-frontend.sh`、`scripts/deploy-backend.sh`、Gitee CI 变量
- Produces: tag 触发时自动执行前后端部署

- [ ] **Step 1: 编写流水线**

```yaml
stages:
  - deploy

variables:
  DOCKER_IMAGE: "rust-learning-playground:1.86"
  CONCURRENCY: "4"
  TIMEOUT_SECONDS: "120"
  MEMORY_LIMIT_MB: "512"

# 仅在 v* tag 时触发
workflow:
  rules:
    - if: '$CI_COMMIT_TAG =~ /^v/'
      when: always
    - when: never

deploy-frontend:
  stage: deploy
  image: alpine:latest
  before_script:
    - apk add --no-cache openssh-client rsync
  script:
    - sh scripts/deploy-frontend.sh
  only:
    - tags

deploy-backend:
  stage: deploy
  image: alpine:latest
  before_script:
    - apk add --no-cache openssh-client
  script:
    - sh scripts/deploy-backend.sh
  only:
    - tags
```

- [ ] **Step 2: YAML 语法检查**

```bash
python3 -c "import yaml, sys; yaml.safe_load(open('/Users/yanshili/me/projects/rust-projects/.gitee-ci.yml')); print('YAML OK')"
```

Expected: 输出 `YAML OK`。如果缺少 PyYAML，先执行 `pip install pyyaml`。

- [ ] **Step 3: Commit**

```bash
git add .gitee-ci.yml
git commit -m "feat: add Gitee CI deployment pipeline"
```

---

### Task 5: 更新部署说明文档

**Files:**
- Modify: `README.md`（追加部署章节）

**Interfaces:**
- Consumes: 设计文档、流水线配置
- Produces: 用户可读的部署指南

- [ ] **Step 1: 在 README.md 末尾追加部署说明**

```markdown
## 自动部署

本项目已配置 Gitee CI，推送 `v*` 标签时自动部署：

```bash
git tag v1.0.0
git push origin v1.0.0
```

### 前置条件

1. 在 Gitee 仓库 **设置 → CI/CD → 变量** 中配置：
   - `SSH_PRIVATE_KEY`
   - `FRONTEND_HOST`、`FRONTEND_USER`、`FRONTEND_WEB_ROOT`
   - `BACKEND_HOST`、`BACKEND_USER`、`BACKEND_DEPLOY_DIR`、`BACKEND_SERVICE`、`BACKEND_PORT`
   - `EVALUATE_URL`
2. 后端服务器安装 Rust、Cargo、Docker 和 `rust-learning-playground:1.86` 镜像。
3. 后端服务器配置 systemd 服务：
   ```bash
   sudo cp systemd/rust-learning-backend.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable rust-learning-backend
   ```
4. 把 CI 使用的 SSH 公钥添加到前后端服务器的 `~/.ssh/authorized_keys`。

### 部署流程

- 前端：`scp` 上传静态文件到 `FRONTEND_WEB_ROOT`，并替换 `index.html` 中的 `evaluateUrl`。
- 后端：`scp` 上传源码到 `BACKEND_DEPLOY_DIR`，服务器上执行 `cargo build --release` 并重启服务。
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add Gitee CI deployment instructions"
```

---

### Task 6: 本地验证

**Files:**
- Read: `.gitee-ci.yml`
- Read: `scripts/deploy-frontend.sh`
- Read: `scripts/deploy-backend.sh`

**Interfaces:**
- Consumes: 已创建的脚本和流水线
- Produces: 语法与结构验证通过

- [ ] **Step 1: 验证 YAML 可解析**

```bash
cd /Users/yanshili/me/projects/rust-projects
python3 -c "import yaml; yaml.safe_load(open('.gitee-ci.yml')); print('YAML OK')"
```

- [ ] **Step 2: 验证 Bash 脚本无语法错误**

```bash
cd /Users/yanshili/me/projects/rust-projects
bash -n scripts/deploy-frontend.sh
bash -n scripts/deploy-backend.sh
echo "Bash scripts OK"
```

- [ ] **Step 3: 检查文件列表**

```bash
cd /Users/yanshili/me/projects/rust-projects
ls -la .gitee-ci.yml scripts/deploy-frontend.sh scripts/deploy-backend.sh systemd/rust-learning-backend.service
```

Expected: 四个文件均存在。

- [ ] **Step 4: Commit（如有 README 修改则一起提交，否则可跳过）**

```bash
git commit -m "ci: complete Gitee CI deployment setup" || true
```

---

## Spec Coverage Self-Check

| 设计文档要求 | 对应任务 |
|---|---|
| 推送 `v*` tag 触发 | Task 4 `.gitee-ci.yml` workflow rules |
| 前端 scp 部署 + URL 替换 | Task 1 `scripts/deploy-frontend.sh` |
| 后端服务器编译 + systemd 重启 | Task 2 `scripts/deploy-backend.sh` + Task 3 service file |
| 密钥和配置走 Gitee CI 变量 | Task 1/2 脚本中通过环境变量引用 |
| 文档说明 | Task 5 README.md |
| 本地语法验证 | Task 6 |

## Placeholder Scan

- 无 `TBD`、`TODO` 等占位符。
- 所有脚本、YAML、systemd 文件均给出完整代码。
- 变量名与设计文档一致。
