# Monitoring Agent HTTP

A cross-platform (Linux/Windows) HTTP monitoring agent written in Rust. It collects system metrics, checks services and HTTP ports, and exposes them via a REST API. Binaries are built and released automatically for both Linux and Windows.

## Features
- CPU, memory, load, and per-disk usage metrics (percentages)
- Service status checks (systemctl on Linux, sc on Windows)
- HTTP port checks
- REST API endpoints for each metric and check
- Configurable via `config.toml`
- Cross-compilation and GitHub Actions release pipeline

## API Endpoints
- `/` — Health check
- `/metrics` — All metrics and checks (plain text)
- `/cpu` — CPU usage (%)
- `/memory` — Memory usage (%)
- `/load` — 15min load average (%) only on linux
- `/disk` — Per-disk usage (%)
- `/service/{name}` — Service status
- `/port/{name}` — Port status

## Configuration
Create a `config.toml` in the root directory:

```toml
app_port = 8081
services = ["sshd", "nginx"]
[http_ports]
web = 80
api = 8080
```

- `app_port`: (optional) Port to run the agent on (default: 8081)
- `services`: List of service names to check
- `[http_ports]`: Named HTTP ports to check

## Building Locally

### Linux
```sh
cargo build --release
./target/release/monitoring-agent-http
```

### Windows (cross-compile from Linux)
```sh
rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
```

## GitHub Actions Release
On every tag push (e.g. `v0.1.0`), the workflow builds Linux and Windows binaries and attaches them to a GitHub release.

## Testing
Run all tests:
```sh
cargo test
```

## License
MIT
