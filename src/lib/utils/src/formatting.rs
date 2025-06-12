use std::time::Duration;

pub fn format_bytes(bytes: u64) -> String {
    let prefixes = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes as f64;
    let mut prefix_index = 0;
    while value >= 1024.0 && prefix_index < prefixes.len() - 1 {
        value /= 1024.0;
        prefix_index += 1;
    }
    if prefix_index == 0 {
        format!("{:.0} {}", value, prefixes[prefix_index])
    } else {
        format!("{:.2} {}", value, prefixes[prefix_index])
    }
}

fn format_time_small(duration: Duration) -> String {
    let nanos = duration.as_nanos() as f64;
    if nanos < 1_000.0 {
        if nanos % 1.0 == 0.0 {
            format!("{nanos:.0} ns")
        } else {
            format!("{nanos:.2} ns")
        }
    } else if nanos < 1_000_000.0 {
        if nanos % 1_000.0 == 0.0 {
            format!("{:.0} µs", nanos / 1_000.0)
        } else {
            format!("{:.2} µs", nanos / 1_000.0)
        }
    } else if nanos % 1_000_000.0 == 0.0 {
        format!("{:.0} ms", nanos / 1_000_000.0)
    } else {
        format!("{:.2} ms", nanos / 1_000_000.0)
    }
}

fn format_time_large(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!(
            "{}d {}h {}m {}s",
            days,
            hours % 24,
            minutes % 60,
            seconds % 60
        )
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{seconds}s")
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration.as_nanos() < 1_000_000_000 {
        format_time_small(duration)
    } else {
        format_time_large(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
        assert_eq!(format_bytes(1099511627776), "1.00 TB");
    }

    #[test]
    fn test_format_duration_subsecond() {
        assert_eq!(format_duration(Duration::from_nanos(500)), "500 ns");
        assert_eq!(format_duration(Duration::from_nanos(1500)), "1.50 µs");
        assert_eq!(format_duration(Duration::from_micros(150)), "150 µs");
        assert_eq!(format_duration(Duration::from_micros(2000)), "2 ms");
        assert_eq!(format_duration(Duration::from_micros(2320)), "2.32 ms");
        assert_eq!(format_duration(Duration::from_millis(2)), "2 ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1s");
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
        assert_eq!(format_duration(Duration::from_secs(125)), "2m 5s");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h 0m 0s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }
}
