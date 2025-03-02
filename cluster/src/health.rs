use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HealthStatus {
    Unhealthy,
    Healthy,
    Degraded,
    Starting,
    OffService,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub status: HealthStatus,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            status: HealthStatus::OffService,
        }
    }
}
impl Health {
    fn setStatus(&mut self, status: HealthStatus) {
        self.status = status;
    }
}
