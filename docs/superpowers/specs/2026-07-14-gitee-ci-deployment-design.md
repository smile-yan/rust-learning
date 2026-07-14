# Gitee CI 自动部署设计文档

## 目标

实现一个 Gitee CI 流水线：当用户推送以 `v` 开头的 tag（如 `v1.0.0`）时，自动将本项目的前端静态资源和后端 Rust 服务分别部署到两台服务器。

## 部署架构

```
开发者 push tag v1.0.0
        │
        ▼
   Gitee CI 触发
        │
        ├──► 部署前端 ──► 前端服务器（Nginx/Caddy 静态站点目录）
        │
        └──► 部署后端 ──► 后端服务器（cargo build --release + systemd）
```

- **前端**：纯静态文件，通过 `scp`/`rsync` 上传到前端服务器的 Web 根目录。
- **后端**：通过 `scp` 将源码上传到后端服务器，SSH 登录后执行 `cargo build --release`，并由 `systemd` 服务管理启动/重启。
- **CI Runner**：使用 Gitee 共享 Runner，不依赖 Rust/Docker，只使用 `ssh`/`scp`。

## 触发条件

- 仅当推送 `v*` 开头的 tag 时触发。
- 分支推送、PR、其他 tag 不触发部署。

## Gitee CI/CD 变量清单

在 Gitee 仓库 **设置 → CI/CD → 变量** 中配置以下变量（建议全部设为「受保护」和「隐藏」）：

| 变量名 | 说明 | 示例 |
|---|---|---|
| `SSH_PRIVATE_KEY` | 用于登录两台服务器的 SSH 私钥 | `-----BEGIN OPENSSH PRIVATE KEY-----...` |
| `FRONTEND_HOST` | 前端服务器 IP 或域名 | `192.168.1.10` |
| `FRONTEND_USER` | 前端服务器登录用户 | `deploy` |
| `FRONTEND_WEB_ROOT` | 前端静态文件部署目录 | `/var/www/rust-learning` |
| `BACKEND_HOST` | 后端服务器 IP 或域名 | `192.168.1.11` |
| `BACKEND_USER` | 后端服务器登录用户 | `deploy` |
| `BACKEND_DEPLOY_DIR` | 后端源码部署目录 | `/opt/rust-learning` |
| `BACKEND_SERVICE` | 后端 systemd 服务名 | `rust-learning-backend` |
| `BACKEND_PORT` | 后端监听端口 | `9001` |
| `EVALUATE_URL` | 生产环境后端 `/evaluate.json` 地址 | `http://api.rust-learning.example.com/evaluate.json` |

可选变量（用于运行时覆盖）：

| 变量名 | 说明 | 默认值 |
|---|---|---|
| `STATIC_DIR` | 后端托管静态文件时的目录 | 与 `BACKEND_DEPLOY_DIR` 一致 |
| `DOCKER_IMAGE` | 后端编译运行使用的 Docker 镜像 | `rust-learning-playground:1.86` |
| `CONCURRENCY` | 最大并发编译任务数 | `4` |
| `TIMEOUT_SECONDS` | 单次编译运行超时 | `120` |
| `MEMORY_LIMIT_MB` | Docker 容器内存限制 | `512` |

## 部署流程

### 前端部署

1. CI 检出代码。
2. 安装 SSH 私钥。
3. 用 `sed` 将 `index.html` 中的 `evaluateUrl` 替换为生产地址 `EVALUATE_URL`。
4. 通过 `scp -r` 将以下文件/目录上传到 `FRONTEND_USER@FRONTEND_HOST:FRONTEND_WEB_ROOT/`：
   - `index.html`
   - `css/`
   - `js/`
   - `libs/`
   - `images/`
5. 可选：清理旧文件或设置文件权限。

### 后端部署

1. CI 检出代码。
2. 安装 SSH 私钥。
3. 通过 `scp` 将整个项目源码上传到 `BACKEND_USER@BACKEND_HOST:BACKEND_DEPLOY_DIR/`。
4. SSH 登录后端服务器并执行：
   - `cd BACKEND_DEPLOY_DIR`
   - `cargo build --release`
   - `sudo systemctl restart BACKEND_SERVICE`
5. 检查服务状态：
   - `systemctl status BACKEND_SERVICE`
   - `curl -f http://localhost:BACKEND_PORT/`

## 新增/修改文件

| 文件 | 说明 |
|---|---|
| `.gitee-ci.yml` | Gitee CI 流水线配置文件 |
| `scripts/deploy-frontend.sh` | 前端部署辅助脚本（可选，用于复杂逻辑） |
| `scripts/deploy-backend.sh` | 后端部署辅助脚本（可选） |
| `index.html` | CI 运行时动态替换 `evaluateUrl`，不修改仓库原始内容 |

## 安全注意事项

- `SSH_PRIVATE_KEY` 必须设为隐藏变量，禁止打印到日志。
- 部署用户建议使用非 root 用户，配合 `sudo` 权限白名单（`systemctl restart rust-learning-backend`、`systemctl status rust-learning-backend`）。
- 后端服务器需提前安装 Rust、`cargo`、`systemd` 服务单元文件，并配置好 Docker 编译镜像。
- 建议对 tag 使用「受保护 tag」策略，避免任意协作者推送 tag 即触发部署。

## 成功标准

- 推送 `v1.0.0` 后 Gitee CI 自动触发。
- 前端页面可以正常访问，`/evaluate.json` 请求指向生产后端地址。
- 后端 `/evaluate.json` 接口返回 200，Rust 代码可以正常编译执行。
- 部署失败时 CI 任务显示失败并输出日志。
