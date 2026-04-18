pub mod cpu;
pub mod ram;

use crate::error::Result;
use crate::model::ResourceStats;

pub trait ResourceCollector {
    fn fetch_stats(&mut self) -> Result<ResourceStats>;
}