use crate::error::Result;
use crate::model::ResourceStats;
use crate::monitor::ResourceCollector;

pub struct CpuMonitor;

impl CpuMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl ResourceCollector for CpuMonitor {
    fn fetch_stats(&mut self) -> Result<ResourceStats> {
        let raw_data = "CPU 45.2";
        ResourceStats::try_from(raw_data)
    }
}