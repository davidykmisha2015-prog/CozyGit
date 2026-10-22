use axum::{
    body::Bytes,
    extract::{Path, Query},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct GitQuery {
    service: Option<String>,
}

#[tokio::main]
async fn main() {
    // Directory where all bare repositories are stored
    let repos_dir = "./repos";
    std::fs::create_dir_all(repos_dir).unwrap();

    // Routes for the Git server
    let app = Router::new()
        // Landing / index test page
        .route(
            "/",
            get(|| async {
                "☕ CozyGit v0.1.0 is running!\n\nTry in terminal:\n  git clone http://localhost:3000/test.git\n"
            }),
        )
        // Health check endpoint
        .route("/status", get(get_status_from_server))
        // Step 1: Git client queries branches and commits (git clone / pull / push)
        .route("/{repo}/info/refs", get(handle_info_refs))
        // Step 2: Git client fetches objects (git clone / pull)
        .route("/{repo}/git-upload-pack", post(handle_upload_pack))
        // Step 3: Git client pushes new commits (git push)
        .route("/{repo}/git-receive-pack", post(handle_receive_pack));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("========================================================");
    println!("  ☕ CozyGit Server running on http://{}", addr);
    println!("  📁 Repositories directory: ./repos");
    println!("========================================================");
    println!("  Try running in another terminal:");
    println!("    git clone http://localhost:3000/my-code.git");
    println!("========================================================");

    axum::serve(listener, app).await.unwrap();
}

// ---------------------------------------------------------------------------------
// 1. Handler: advertises branches and commits to client (info/refs)
// ---------------------------------------------------------------------------------
async fn handle_info_refs(
    Path(repo): Path<String>,
    Query(query): Query<GitQuery>,
) -> impl IntoResponse {
    let repo_path = format!("./repos/{}", repo);

    // If repository doesn't exist yet, automatically create it on the fly!
    if !std::path::Path::new(&repo_path).exists() {
        println!("📦 Creating new empty repository: {}", repo);
        Command::new("git")
            .args(["init", "--bare", "--initial-branch=main", &repo_path])
            .output()
            .unwrap();

        // Allow pushing over plain HTTP without authentication for local tests
        Command::new("git")
            .args(["-C", &repo_path, "config", "http.receivepack", "true"])
            .output()
            .unwrap();
    }

    let service = query.service.unwrap_or_default();
    if service != "git-upload-pack" && service != "git-receive-pack" {
        return (StatusCode::FORBIDDEN, "Unsupported service").into_response();
    }

    println!("🔍 Git client requesting info for service: {} ({})", service, repo);

    // Run standard system git utility to advertise references
    let out = Command::new(&service)
        .args(["--stateless-rpc", "--advertise-refs", &repo_path])
        .output()
        .unwrap();

    // Construct service announcement according to Git Smart HTTP specification (packet-line)
    let packet = format!("# service={}\n", service);
    let len = format!("{:04x}", packet.len() + 4);

    let mut body = Vec::new();
    body.extend_from_slice(len.as_bytes());
    body.extend_from_slice(packet.as_bytes());
    body.extend_from_slice(b"0000"); // Git flush-pkt separator
    body.extend_from_slice(&out.stdout);

    let content_type = format!("application/x-{}-advertisement", service);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    (headers, body).into_response()
}

// ---------------------------------------------------------------------------------
// 2. Handler: streams pack objects to client during `git clone` / `git pull`
// ---------------------------------------------------------------------------------
async fn handle_upload_pack(Path(repo): Path<String>, body: Bytes) -> impl IntoResponse {
    let repo_path = format!("./repos/{}", repo);
    println!("⬇️  Client fetching data (git clone/pull): {}", repo);

    let mut child = Command::new("git-upload-pack")
        .args(["--stateless-rpc", &repo_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body);
    }

    let out = child.wait_with_output().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-git-upload-pack-result"),
    );

    (headers, out.stdout).into_response()
}

// ---------------------------------------------------------------------------------- 
// 3. Health check handler: /status
// ----------------------------------------------------------------------------------
async fn get_status_from_server() -> impl IntoResponse {
    "CozyGit is healthy and alive!\n"
}

// ---------------------------------------------------------------------------------
// 4. Handler: receives new commits during `git push`
// ---------------------------------------------------------------------------------
async fn handle_receive_pack(Path(repo): Path<String>, body: Bytes) -> impl IntoResponse {
    let repo_path = format!("./repos/{}", repo);
    println!("⬆️  Client pushing new code (git push): {}", repo);

    let mut child = Command::new("git-receive-pack")
        .args(["--stateless-rpc", &repo_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&body);
    }

    let out = child.wait_with_output().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-git-receive-pack-result"),
    );

    println!("✅ Commit successfully accepted by CozyGit!");

    (headers, out.stdout).into_response()
}
