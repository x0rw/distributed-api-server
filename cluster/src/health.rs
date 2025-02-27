pub enum HealthStatus {
    Unhealthy,
    Healthy,
    Degraded,
    Starting,
    OffService,
}
pub struct Health {
    status: HealthStatus,
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
