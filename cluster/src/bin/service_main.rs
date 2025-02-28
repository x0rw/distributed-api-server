use cluster::service::Service;

fn main() {
    let clad = "127.0.0.1:1111".to_string();
    let service = Service::init("test_service", "#ser");
    let _discover = service.discover(clad);
}
