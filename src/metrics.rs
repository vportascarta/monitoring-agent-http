//! System metrics collection for monitoring agent.

use sysinfo::{Disks, System};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub cpu: f32,
    pub mem_percent: f32,
    pub load1: f64,
    pub load5: f64,
    pub load15: f64,
    pub disk_stats: Vec<(String, f32)>,
}

/// Collects system metrics (CPU, memory, load, disks).
pub fn get_metrics() -> Metrics {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu = sys.global_cpu_usage();
    let mem_used = sys.used_memory() as f32;
    let mem_total = sys.total_memory() as f32;
    let load = System::load_average();
    let num_cpus = sys.cpus().len();
    let disks = Disks::new_with_refreshed_list();
    let mut disk_stats = Vec::new();

    #[cfg(target_os = "windows")]
    {
        for disk in disks.iter() {
            let mount = disk.mount_point().to_str().unwrap_or("");
            let total = disk.total_space() as f32;
            let used = (disk.total_space() - disk.available_space()) as f32;
            let percent = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
            disk_stats.push((mount.to_owned(), percent));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        for disk in disks.iter() {
            let name = disk.name().to_str().unwrap_or("");
            let mount = disk.mount_point().to_str().unwrap_or("");
            if name.starts_with("/dev") {
                let total = disk.total_space() as f32;
                let used = (disk.total_space() - disk.available_space()) as f32;
                let percent = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
                disk_stats.push((format!("{} - {}", name, mount), percent));
            }
        }
    }
    Metrics {
        cpu,
        mem_percent: if mem_total > 0.0 { (mem_used / mem_total) * 100.0 } else { 0.0 },
        load1: (load.one / num_cpus as f64) * 100.0,
        load5: (load.five / num_cpus as f64) * 100.0,
        load15: (load.fifteen / num_cpus as f64) * 100.0,
        disk_stats,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_fields() {
        let metrics = get_metrics();
        // Just check that fields exist and are in range
        assert!(metrics.cpu >= 0.0);
        assert!(metrics.mem_percent >= 0.0);
        assert!(metrics.load1 >= 0.0);
        assert!(metrics.load5 >= 0.0);
        assert!(metrics.load15 >= 0.0);
    }
}
