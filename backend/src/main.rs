use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    os::unix::process::CommandExt,
    path::PathBuf,
    sync::Arc,
    time::Instant,
};
use tokio::{
    fs,
    sync::{AcquireError, Semaphore},
};
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use uuid::Uuid;

const DEFAULT_TIMEOUT_SECONDS: u64 = 120;
const DEFAULT_MEMORY_LIMIT_MB: u64 = 512;
const DOCKER_IMAGE: &str = "rust-learning-playground:1.86";

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

    let port = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(9001);
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
    let t0 = Instant::now();
    let _permit = state
        .semaphore
        .clone()
        .try_acquire_owned()
        .map_err(|_| AppError::TooManyRequests)?;
    info!("acquire permit: {:?}", t0.elapsed());

    let id = Uuid::new_v4();
    let t1 = Instant::now();
    let temp_dir = tempfile::Builder::new()
        .prefix("rust-playground-")
        .tempdir_in("/tmp")
        .map_err(AppError::Internal)?;
    let workdir = temp_dir.path().to_path_buf();
    info!("temp dir: {:?}", t1.elapsed());

    let project_dir = workdir.join("project");
    let t2 = Instant::now();
    fs::create_dir_all(&project_dir).await.map_err(AppError::Internal)?;
    info!("mkdir: {:?}", t2.elapsed());

    // Write main.rs. The Docker image already contains a pre-built Cargo.toml,
    // Cargo.lock and target/ cache at /tmp/project; the runner only replaces main.rs.
    let t3 = Instant::now();
    fs::write(project_dir.join("main.rs"), &req.code)
        .await
        .map_err(AppError::Internal)?;
    info!("write main.rs: {:?}", t3.elapsed());

    let t4 = Instant::now();
    let output = run_in_docker(&state, &project_dir, &id).await?;
    info!("docker run: {:?}", t4.elapsed());

    let elapsed = t0.elapsed();
    info!("Request {} total: {:?}", id, elapsed);

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    info!("stdout len: {}, stderr len: {}", stdout.len(), stderr.len());

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
    let _container_name = format!("rust-playground-{}", id);
    let host_project = project_dir
        .canonicalize()
        .map_err(|e| AppError::Internal(e.into()))?;
    let host_output = project_dir.join("output");
    fs::create_dir_all(&host_output).await.map_err(AppError::Internal)?;

    let docker_image = state.docker_image.clone();
    let memory_limit_mb = state.memory_limit_mb;
    let timeout_seconds = state.timeout_seconds;

    let docker_runner = std::env::current_exe()
        .map_err(|e| AppError::Internal(e))?
        .parent()
        .ok_or_else(|| AppError::Internal(std::io::Error::new(std::io::ErrorKind::Other, "missing exe dir")))?
        .join("docker-runner");

    let output = tokio::task::spawn_blocking(move || {
        let mut cmd = std::process::Command::new(&docker_runner);
        cmd.arg(host_project.display().to_string())
            .arg(host_output.display().to_string())
            .arg(docker_image)
            .arg(format!("{}", memory_limit_mb))
            .arg(format!("{}", timeout_seconds))
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let output = unsafe {
            cmd.pre_exec(|| {
                // Create a new session so Docker CLI on macOS doesn't inherit
                // the parent's stdio/session state, which can cause ~20s delays.
                libc::setsid();
                Ok(())
            })
        }
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::DockerNotFound
            } else {
                AppError::Internal(e)
            }
        })?;

        Ok(output)
    })
    .await
    .map_err(|e| AppError::Internal(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    output
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
