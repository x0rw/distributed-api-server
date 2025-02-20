#[derive(Debug)]
pub enum AuthType {
    Bearer,
    Basic,
    Digest,
    OAuth,
}
#[derive(Debug)]
pub struct Auth {
    auth_type: AuthType,
    value: String,
}
