use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let log_path = format!("/tmp/docker-runner-{}.log", std::process::id());
    let mut log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .unwrap();

    writeln!(log, "started args={:?}", args).ok();

    if args.len() != 6 {
        writeln!(log, "bad args").ok();
        eprintln!(
            "Usage: docker-runner <project_dir> <output_dir> <docker_image> <memory_mb> <timeout_seconds>"
        );
        std::process::exit(1);
    }

    let project_dir = &args[1];
    let output_dir = &args[2];
    let docker_image = &args[3];
    let memory_mb = &args[4];
    let timeout_seconds: u64 = args[5].parse().expect("timeout_seconds must be a number");

    writeln!(log, "project_dir={} output_dir={}", project_dir, output_dir).ok();

    let start = Instant::now();

    // Inside the container the project is mounted at /project, regardless of
    // the host path. Replace only main.rs so Cargo reuses the warm target/ cache.
    let shell_cmd = "mkdir -p /output && cp /project/main.rs /tmp/project/main.rs && cd /tmp/project && cargo build --offline --quiet >/output/stdout.txt 2>/output/stderr.txt && ./target/debug/playground >>/output/stdout.txt 2>>/output/stderr.txt".to_string();

    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("--rm")
        .arg("--network")
        .arg("none")
        .arg("--cap-drop")
        .arg("ALL")
        .arg("--security-opt")
        .arg("no-new-privileges:true")
        .arg("--memory")
        .arg(format!("{}m", memory_mb))
        .arg("--memory-swap")
        .arg(format!("{}m", memory_mb))
        .arg("--cpus")
        .arg("1.0")
        .arg("--pids-limit")
        .arg("64")
        .arg("-v")
        .arg(format!("{}:/project:ro", project_dir))
        .arg("-v")
        .arg(format!("{}:/output", output_dir))
        .arg("-w")
        .arg("/project")
        .arg(docker_image)
        .arg("sh")
        .arg("-c")
        .arg(&shell_cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd.output().expect("failed to run docker");

    writeln!(
        log,
        "docker status={:?} elapsed={:?} stdout_len={} stderr_len={}",
        output.status,
        start.elapsed(),
        output.stdout.len(),
        output.stderr.len()
    )
    .ok();

    // Read output files from host output_dir
    let stdout_from_file = std::fs::read(format!("{}/stdout.txt", output_dir)).unwrap_or_default();
    let stderr_from_file = std::fs::read(format!("{}/stderr.txt", output_dir)).unwrap_or_default();
    writeln!(
        log,
        "file stdout_len={} stderr_len={}",
        stdout_from_file.len(),
        stderr_from_file.len()
    )
    .ok();

    // Forward to our stdout/stderr. If the container failed before writing
    // the output files, also forward Docker's own stderr so we can diagnose it.
    std::io::Write::write_all(&mut std::io::stderr(), &stderr_from_file)
        .ok();
    std::io::Write::write_all(&mut std::io::stdout(), &stdout_from_file)
        .ok();
    if !output.status.success() && stdout_from_file.is_empty() && stderr_from_file.is_empty() {
        std::io::Write::write_all(&mut std::io::stderr(), &output.stderr).ok();
        std::io::Write::write_all(&mut std::io::stdout(), &output.stdout).ok();
    }

    if start.elapsed() > Duration::from_secs(timeout_seconds) {
        std::process::exit(124);
    }
}
