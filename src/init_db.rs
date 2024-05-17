use crate::settings::ServiceConfig;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
/// init_db runs migrations
pub async fn run_migrations(service_config: &ServiceConfig) {
    // let db_host = std::env::var("DB_HOST").expect("DB_HOST not set.");
    // let db_name = std::env::var("DB_NAME").expect("DB_NAME not set.");
    // let db_user = std::env::var("DB_USER").expect("DB_USER not set.");
    // let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set.");
    // let db_port: u16 = std::env::var("DB_PORT")
    //     .expect("DB_PORT not set.")
    //     .parse()
    //     .unwrap();

    let connection_config = format!(
        "host={} user={} password={} dbname={} port={}",
        service_config.db_host,
        service_config.db_user,
        service_config.db_password,
        service_config.db_name,
        service_config.db_port // db_host, db_user, db_password, db_name, db_port
    );
    let (mut client, connection) = tokio_postgres::connect(&connection_config, NoTls)
        .await
        .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .unwrap();
    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }
    println!("DB migrations finished!");
}
