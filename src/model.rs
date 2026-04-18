use crate::error::MonitorError;

pub struct ResourceStats {
    pub label: String,
    pub value: f64,
}

impl TryFrom<&str> for ResourceStats {
    type Error = MonitorError;

    fn try_from(data: &str) -> std::result::Result<Self, Self::Error> {
        let parts: Vec<&str> = data.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(MonitorError::InvalidFormat);
        }

        let value = parts[1].parse::<f64>().map_err(|_| MonitorError::InvalidFormat)?;

        if value < 0.0 {
            return Err(MonitorError::InvalidFormat);
        }

        Ok(Self {
            label: parts[0].to_string(),
            value,
        })
    }
}