pub trait ServiceDiscovery {
    fn register_service(
        &self,
        service_name: &str,
        service_id: &str,
        address: &str,
        port: u16,
        tags: Option<&Vec<String>>,
        health_check_url: Option<&str>,
        health_check_interval: Option<std::time::Duration>,
    ) -> std::io::Result<()>;
    fn deregister_service(&mut self, service_id: &str) -> std::io::Result<()>;
    fn discover_services(
        &self,
        service_name: &str,
        tags: Option<&Vec<String>>,
    ) -> std::io::Result<Vec<ServiceInstance>>;
}
#[derive(Debug, Clone)]
pub struct ServiceInstance {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
}
