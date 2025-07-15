//! Service and port check utilities for monitoring agent.

use std::net::TcpStream;
use std::process::Command;

/// Checks if a named service is running (systemctl or sc).
pub fn check_service(service: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("sc").arg("query").arg(service).output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.contains("RUNNING")
            }
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("systemctl")
            .arg("is-active")
            .arg(service)
            .output();
        match output {
            Ok(out) => {
                out.status.success() && String::from_utf8_lossy(&out.stdout).trim() == "active"
            }
            Err(_) => false,
        }
    }
}

/// Checks if a TCP port is open on localhost.
pub fn check_http_port(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_http_port() {
        // 65535 is almost always closed
        assert!(!check_http_port(65535));
    }
}
