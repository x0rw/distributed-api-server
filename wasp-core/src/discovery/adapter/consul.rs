use super::super::service_discovery::ServiceDiscovery;
use consul::agent::Agent;
use consul::catalog::Catalog;
use consul::check::Check;
use consul::health::Health;
use consul::service::Service;
use consul::{Client, Config};
use std::io::{Error, ErrorKind};
use std::time::Duration;

pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
}

pub struct ConsulServiceDiscovery {
    client: Client,
}

impl ConsulServiceDiscovery {
    pub fn new(consul_address: &str) -> std::io::Result<Self> {
        let mut config = Config::new();
        config.address = consul_address.to_string();
        let client = Client::new(config);

        // Verify connection
        if let Err(e) = client.agent().services() {
            return Err(Error::new(ErrorKind::ConnectionRefused, e.to_string()));
        }

        Ok(ConsulServiceDiscovery { client })
    }
}

impl ServiceDiscovery for ConsulServiceDiscovery {
    fn register_service(
        &self,
        service_name: &str,
        service_id: &str,
        address: &str,
        port: u16,
        tags: Option<&Vec<String>>,
        health_check_url: Option<&str>,
        health_check_interval: Option<Duration>,
    ) -> std::io::Result<()> {
        let mut service = Service::new(service_name, service_id, address, port);

        if let Some(tags) = tags {
            service = service.set_tags(tags.clone());
        }

        if let (Some(check_url), Some(interval)) = (health_check_url, health_check_interval) {
            let check = Check::http(check_url, interval).set_notes("Service health check");
            service = service.set_check(check);
        }

        self.client
            .agent()
            .service_register(&service)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
    }

    fn deregister_service(&mut self, service_id: &str) -> std::io::Result<()> {
        self.client
            .agent()
            .service_deregister(service_id)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    fn discover_services(
        &self,
        service_name: &str,
        tags: Option<&Vec<String>>,
    ) -> std::io::Result<Vec<ServiceInstance>> {
        let services = self
            .client
            .health()
            .service(service_name, None, true, None)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

        let mut instances = Vec::new();

        for service_entry in services {
            let service = service_entry.service.ok_or_else(|| {
                Error::new(ErrorKind::InvalidData, "Service data missing in response")
            })?;

            // Filter by tags if provided
            if let Some(required_tags) = tags {
                if let Some(service_tags) = &service.tags {
                    if !required_tags.iter().all(|t| service_tags.contains(t)) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            instances.push(ServiceInstance {
                id: service.id.clone(),
                name: service.service.clone(),
                address: service.address.clone().unwrap_or_default(),
                port: service.port.unwrap_or(0),
                tags: service.tags.unwrap_or_default(),
            });
        }

        Ok(instances)
    }

    fn discover_service(
        &self,
        service_name: &str,
        tags: Option<&Vec<String>>,
    ) -> std::io::Result<Vec<ServiceInstance>> {
        self.discover_services(service_name, tags)
    }
}
