#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::thread;
    use std::time::Duration;

    // Helper to get Consul address from env or use default
    fn consul_address() -> String {
        std::env::var("CONSUL_HTTP_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8500".to_string())
    }

    // Helper to generate unique service IDs
    fn unique_service_id(prefix: &str) -> String {
        format!("{}-{}", prefix, rand::random::<u32>())
    }

    #[test]
    #[serial]
    fn test_service_registration_and_discovery() {
        let consul_addr = consul_address();
        let mut discovery = ConsulServiceDiscovery::new(&consul_addr).unwrap();

        // Register a service
        let service_id = unique_service_id("test-service");
        let register_result = discovery.register_service(
            "test-service",
            &service_id,
            "127.0.0.1",
            8080,
            Some(&vec!["integration-test".to_string(), "rust".to_string()]),
            Some("http://127.0.0.1:8080/health"),
            Some(Duration::from_secs(10)),
        );
        assert!(register_result.is_ok(), "Service registration failed");

        // Small delay to allow Consul to update its state
        thread::sleep(Duration::from_millis(500));

        // Discover without tags
        let discovered = discovery.discover_services("test-service", None);
        assert!(discovered.is_ok(), "Service discovery failed");
        let services = discovered.unwrap();
        assert!(!services.is_empty(), "No services found");
        assert!(services.iter().any(|s| s.id == service_id));

        // Discover with tag filter
        let discovered_with_tag =
            discovery.discover_services("test-service", Some(&vec!["rust".to_string()]));
        assert!(discovered_with_tag.is_ok());
        let tagged_services = discovered_with_tag.unwrap();
        assert!(tagged_services.iter().any(|s| s.id == service_id));

        // Cleanup
        discovery.deregister_service(&service_id).unwrap();
    }

    #[test]
    #[serial]
    fn test_service_deregistration() {
        let consul_addr = consul_address();
        let mut discovery = ConsulServiceDiscovery::new(&consul_addr).unwrap();

        // Register first
        let service_id = unique_service_id("test-deregister");
        discovery
            .register_service(
                "test-deregister",
                &service_id,
                "127.0.0.1",
                8081,
                None,
                None,
                None,
            )
            .unwrap();

        // Small delay
        thread::sleep(Duration::from_millis(500));

        // Verify registration worked
        let pre_deregister = discovery.discover_services("test-deregister", None);
        assert!(pre_deregister.unwrap().iter().any(|s| s.id == service_id));

        // Deregister
        let deregister_result = discovery.deregister_service(&service_id);
        assert!(deregister_result.is_ok(), "Deregistration failed");

        // Small delay
        thread::sleep(Duration::from_millis(500));

        // Verify service is gone
        let post_deregister = discovery.discover_services("test-deregister", None);
        assert!(!post_deregister.unwrap().iter().any(|s| s.id == service_id));
    }

    #[test]
    #[serial]
    fn test_health_check_registration() {
        let consul_addr = consul_address();
        let discovery = ConsulServiceDiscovery::new(&consul_addr).unwrap();

        let service_id = unique_service_id("health-check-service");
        let result = discovery.register_service(
            "health-check-service",
            &service_id,
            "127.0.0.1",
            8082,
            None,
            Some("http://127.0.0.1:8082/health"),
            Some(Duration::from_secs(5)),
        );

        assert!(result.is_ok(), "Health check registration failed");

        // Cleanup
        discovery.deregister_service(&service_id).unwrap();
    }

    #[test]
    #[serial]
    fn test_tag_filtering() {
        let consul_addr = consul_address();
        let mut discovery = ConsulServiceDiscovery::new(&consul_addr).unwrap();

        // Register services with different tags
        let service1_id = unique_service_id("tag-service-1");
        discovery
            .register_service(
                "tag-service",
                &service1_id,
                "127.0.0.1",
                8083,
                Some(&vec!["tag1".to_string(), "common".to_string()]),
                None,
                None,
            )
            .unwrap();

        let service2_id = unique_service_id("tag-service-2");
        discovery
            .register_service(
                "tag-service",
                &service2_id,
                "127.0.0.1",
                8084,
                Some(&vec!["tag2".to_string(), "common".to_string()]),
                None,
                None,
            )
            .unwrap();

        // Small delay
        thread::sleep(Duration::from_millis(500));

        // Test tag filtering
        let tag1_results = discovery
            .discover_services("tag-service", Some(&vec!["tag1".to_string()]))
            .unwrap();
        assert_eq!(tag1_results.len(), 1);
        assert_eq!(tag1_results[0].id, service1_id);

        let common_results = discovery
            .discover_services("tag-service", Some(&vec!["common".to_string()]))
            .unwrap();
        assert_eq!(common_results.len(), 2);

        // Cleanup
        discovery.deregister_service(&service1_id).unwrap();
        discovery.deregister_service(&service2_id).unwrap();
    }
}
