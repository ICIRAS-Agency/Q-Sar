#[derive(Debug)]
pub enum HttpType {
    HTTP,
    HTTPS
}
impl From< &str > for HttpType {
    fn from( s: &str ) -> Self {
        match s {
            "http" => HttpType::HTTP,
            "https" => HttpType::HTTPS,
            _ => HttpType::HTTP
        }
    }
}