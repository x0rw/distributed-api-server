// service registry and discovery
// nodes register themselves into it

use crate::health::Health;
struct ServiceRegistry {
    service_name: String,
    address: String,
    health: Health,
}

