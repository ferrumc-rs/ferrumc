use sysinfo::{Pid, ProcessesToUpdate, System};

pub enum MemoryUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

pub struct MemoryUsage {
    sys: System,
    process_pid: Pid,
    max_rss_bytes: u64,
}

impl Default for MemoryUsage {
    fn default() -> Self {
        Self {
            sys: System::new(),
            process_pid: sysinfo::get_current_pid().ok().unwrap(),
            max_rss_bytes: 0,
        }
    }
}

impl MemoryUsage {
    fn get_usage_internal(&mut self) -> Option<u64> {
        self.sys
            .refresh_processes(ProcessesToUpdate::Some(&[self.process_pid]), true);

        let process = self.sys.process(self.process_pid)?;

        let rss_bytes = {
            let raw = process.memory();

            #[cfg(windows)]
            {
                raw
            }

            #[cfg(not(windows))]
            {
                raw * 1024
            }
        };

        self.max_rss_bytes = self.max_rss_bytes.max(rss_bytes);
        Some(rss_bytes)
    }

    fn convert(&self, bytes: u64, unit: &MemoryUnit) -> u64 {
        match unit {
            MemoryUnit::Bytes => bytes,
            MemoryUnit::Kilobytes => bytes / 1024,
            MemoryUnit::Megabytes => bytes / 1024 / 1024,
            MemoryUnit::Gigabytes => bytes / 1024 / 1024 / 1024,
        }
    }

    pub fn get_memory(&mut self, unit: MemoryUnit) -> (u64, u64) {
        // updates peak memory, so this has to go first.
        let usage = match self.get_usage_internal() {
            Some(bytes) => self.convert(bytes, &unit),
            None => 0,
        };
        let max = self.convert(self.max_rss_bytes, &unit);

        (usage, max)
    }
}
