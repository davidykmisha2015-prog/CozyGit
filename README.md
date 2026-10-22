<div align="center">
  <img src="assets/logo.svg" alt="CozyGit Logo" width="180"/>
  <h1>CozyGit</h1>
  <p><strong>☕ The Coziest Home for Your Code ❄️🍪</strong></p>
  <p><em>A lightweight, blazingly fast, and offline-first Git server written in pure Rust.</em></p>

  <p>
    <img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Rust">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License: Apache 2.0">
    <img src="https://img.shields.io/badge/Architecture-Offline--First-success.svg" alt="Offline-First">
    <img src="https://img.shields.io/badge/Binary%20Size-2.0%20MB-brightgreen.svg" alt="Size">
  </p>
</div>

---

## 📖 About CozyGit

**CozyGit** is an independent, minimalist, and open-source alternative to heavy cloud git platforms (GitHub, GitLab). 
It is built for developers who value privacy, speed, and complete independence from external internet connections.

### Key Highlights:
- ⚡ **Pure Rust Speed**: Standalone release binary is only **2.0 MB** and consumes under **10 MB of RAM** at idle.
- 🔌 **Offline-First (Air-Gapped Ready)**: Fully functional on local Wi-Fi, in basements, during blackouts, on trains, or in air-gapped security networks.
- 📦 **Native Git Smart HTTP**: 100% compliant with standard `git clone`, `git push`, and `git pull` commands out of the box.
- 🛠️ **Zero Configuration & Auto-Init**: Simply run `git clone http://localhost:3000/my-new-repo.git` — CozyGit creates and configures bare repositories on the fly.
- 🎨 **Warm Gruvbox Aesthetics**: Designed with the cozy comfort of winter coding, hot cocoa, and cookies.

---

## 🚀 Quickstart

### 1. Build and Run the Server
Ensure you have `git` and `rust` installed on your machine:

```bash
# Clone the CozyGit repository
git clone https://github.com/davidykmisha2015-prog/cozygit.git
cd cozygit

# Build and start the release server:
cargo run --release
```

The server will announce in the terminal:
```text
========================================================
  ☕ CozyGit Server running on http://127.0.0.1:3000
  📁 Repositories directory: ./repos
========================================================
```

---

### 2. Using CozyGit (Clone & Push)

In any other terminal window:

```bash
# 1. Clone a new repository (CozyGit initializes it automatically):
git clone http://localhost:3000/my-awesome-app.git

# 2. Navigate into the directory:
cd my-awesome-app

# 3. Create a file:
echo "# My Awesome App" > README.md

# 4. Commit the changes:
git add README.md
git commit -m "First commit to CozyGit!"

# 5. Push to your server:
git push origin master
```

---

## 🩺 Health Check & Diagnostics

CozyGit includes an endpoint for health checks, uptime monitors, and diagnostics:

```bash
curl http://localhost:3000/status
```
**Response:**
```text
CozyGit is healthy and alive!
```

---

## 🗺️ Roadmap

- [x] Full Git Smart HTTP support (`upload-pack` and `receive-pack`).
- [x] Automatic bare repository creation on the fly.
- [x] Built-in `/status` healthcheck endpoint.
- [ ] Lightweight Gruvbox Web UI for browsing commits, diffs, and trees.
- [ ] SSH key support and optional token authentication.
- [ ] Local mesh P2P peer synchronization over Wi-Fi.

---

## 📜 License

This project is licensed under the **Apache License 2.0** — commercial-friendly, patent-protected, and permanently free for everyone.
See the [LICENSE](LICENSE) file for details.

---

<div align="center">
  <p>Crafted with love for open source and cozy coding ☕🍪</p>
  <p>Author: <strong>David</strong> (<a href="https://github.com/davidykmisha2015-prog">@davidykmisha2015-prog</a>) • Chernihiv, Ukraine 🇺🇦</p>
</div>
