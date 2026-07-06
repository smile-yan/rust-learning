use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};
use tempfile::TempDir;
use tokio::{
    fs,
    process::Command,
    sync::{AcquireError, Semaphore},
    time::timeout,
};
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use uuid::Uuid;

const DEFAULT_TIMEOUT_SECONDS: u64 = 25;
const DEFAULT_MEMORY_LIMIT_MB: u64 = 256;
const DOCKER_IMAGE: &str = "rust:1.79-slim";

#[derive(Debug, Clone, Deserialize)]
struct EvaluateRequest {
    #[serde(default = "default_version")]
    version: String,
    #[serde(default = "default_edition")]
    edition: String,
    #[serde(default = "default_crate_type")]
    crate_type: String,
    #[serde(default = "default_mode")]
    mode: String,
    #[serde(default)]
    tests: bool,
    #[serde(default = "default_optimize")]
    optimize: String,
    code: String,
}

fn default_version() -> String { "stable".into() }
fn default_edition() -> String { "2021".into() }
fn default_crate_type() -> String { "bin".into() }
fn default_mode() -> String { "debug".into() }
fn default_optimize() -> String { "0".into() }

#[derive(Debug, Clone, Serialize)]
struct EvaluateResponse {
    result: String,
    error: String,
}

#[derive(Clone)]
struct AppState {
    semaphore: Arc<Semaphore>,
    timeout_seconds: u64,
    memory_limit_mb: u64,
    docker_image: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let concurrency = std::env::var("CONCURRENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let timeout_seconds = std::env::var("TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_TIMEOUT_SECONDS);
    let memory_limit_mb = std::env::var("MEMORY_LIMIT_MB")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MEMORY_LIMIT_MB);
    let docker_image = std::env::var("DOCKER_IMAGE")
        .unwrap_or_else(|_| DOCKER_IMAGE.into());
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "../".into());

    let state = AppState {
        semaphore: Arc::new(Semaphore::new(concurrency)),
        timeout_seconds,
        memory_limit_mb,
        docker_image: docker_image.clone(),
    };

    let app = Router::new()
        .route("/evaluate.json", post(evaluate))
        .nest_service("/", tower_http::services::ServeDir::new(static_dir))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    info!("Backend listening on http://0.0.0.0:{}", port);
    info!("Concurrency: {}, Timeout: {}s, Memory: {}MB, Docker: {}", concurrency, timeout_seconds, memory_limit_mb, docker_image);

    axum::serve(listener, app).await.unwrap();
}

async fn evaluate(
    State(state): State<AppState>,
    Json(req): Json<EvaluateRequest>,
) -> Result<Json<EvaluateResponse>, AppError> {
    let _permit = state
        .semaphore
        .clone()
        .try_acquire_owned()
        .map_err(|_| AppError::TooManyRequests)?;

    let id = Uuid::new_v4();
    let temp_dir = TempDir::new().map_err(AppError::Internal)?;
    let workdir = temp_dir.path().to_path_buf();

    let project_dir = workdir.join("project");
    fs::create_dir_all(&project_dir).await.map_err(AppError::Internal)?;

    // Write Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "playground"
version = "0.1.0"
edition = "{}"

[[bin]]
name = "playground"
path = "main.rs"
"#,
        req.edition
    );
    fs::write(project_dir.join("Cargo.toml"), cargo_toml)
        .await
        .map_err(AppError::Internal)?;

    // Write main.rs
    fs::write(project_dir.join("main.rs"), &req.code)
        .await
        .map_err(AppError::Internal)?;

    let start = Instant::now();
    let output = run_in_docker(&state, &project_dir, &id).await?;
    let elapsed = start.elapsed();
    info!("Request {} completed in {:?}", id, elapsed);

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    let (result, error) = if output.status.success() {
        (stdout, stderr)
    } else {
        let mut err = stderr;
        if err.is_empty() {
            err = stdout;
        }
        if err.is_empty() {
            err = format!("Process exited with status {}", output.status);
        }
        (String::new(), err)
    };

    Ok(Json(EvaluateResponse { result, error }))
}

async fn run_in_docker(
    state: &AppState,
    project_dir: &PathBuf,
    id: &Uuid,
) -> Result<std::process::Output, AppError> {
    let container_name = format!("rust-playground-{}", id);
    let host_project = project_dir
        .canonicalize()
        .map_err(|e| AppError::Internal(e.into()))?;

    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("--rm")
        .arg("--name")
        .arg(&container_name)
        .arg("--network")
        .arg("none")
        .arg("--cap-drop")
        .arg("ALL")
        .arg("--security-opt")
        .arg("no-new-privileges:true")
        .arg("--memory")
        .arg(format!("{}m", state.memory_limit_mb))
        .arg("--memory-swap")
        .arg(format!("{}m", state.memory_limit_mb))
        .arg("--cpus")
        .arg("1.0")
        .arg("--pids-limit")
        .arg("64")
        .arg("-v")
        .arg(format!("{}:/project:ro", host_project.display()))
        .arg("-w")
        .arg("/project")
        .arg(&state.docker_image)
        .arg("sh")
        .arg("-c")
        .arg("cp -r /project /tmp/project && cd /tmp/project && cargo build --quiet 2>&1 && ./target/debug/playground")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            AppError::DockerNotFound
        } else {
            AppError::Internal(e.into())
        }
    })?;

    let result = timeout(Duration::from_secs(state.timeout_seconds), child.wait_with_output())
        .await
        .map_err(|_| AppError::Timeout)?
        .map_err(AppError::Internal)?;

    Ok(result)
}

enum AppError {
    Internal(std::io::Error),
    DockerNotFound,
    Timeout,
    TooManyRequests,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Internal(e) => {
                error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", e))
            }
            AppError::DockerNotFound => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Docker is not available".into(),
            ),
            AppError::Timeout => (
                StatusCode::REQUEST_TIMEOUT,
                "Execution timeout".into(),
            ),
            AppError::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many concurrent requests".into(),
            ),
        };

        let body = Json(serde_json::json!({
            "result": "",
            "error": message,
        }));
        (status, body).into_response()
    }
}

impl From<AcquireError> for AppError {
    fn from(_: AcquireError) -> Self {
        AppError::TooManyRequests
    }
}
