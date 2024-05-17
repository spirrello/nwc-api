use derive_builder::Builder;

#[derive(Clone, Builder)]
pub struct ServiceConfig {
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: u16,
    pub db_url: String,
    pub redis_url: String,
}

impl ServiceConfig {
    pub fn new() -> ServiceConfig {
        let db_host = std::env::var("DB_HOST").expect("DB_HOST not set");
        let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
        let db_user = std::env::var("DB_USER").expect("DB_USER not set");
        let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
        let db_port: u16 = std::env::var("DB_PORT")
            .expect("DB_PORT not set")
            .parse()
            .unwrap();
        let db_url = std::env::var("DB_URL").expect("DB_URL not set");
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");

        ServiceConfigBuilder::default()
            .db_user(db_user)
            .db_password(db_password)
            .db_name(db_name)
            .db_host(db_host)
            .db_port(db_port)
            .db_url(db_url)
            .redis_url(redis_url)
            .build()
            .unwrap()
    }
}
