use crate::error::Result;
use crate::model::ResourceStats;
use crate::monitor::ResourceCollector;

pub struct RamMonitor;

impl RamMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl ResourceCollector for RamMonitor {
    fn fetch_stats(&mut self) -> Result<ResourceStats> {
        let raw_data = "RAM 60.5";
        ResourceStats::try_from(raw_data)
    }
}