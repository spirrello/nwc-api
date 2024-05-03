use deadpool_redis::{Config, Pool, Runtime};
use std::env;
pub type RedisPool = Pool;

#[tokio::main]
pub async fn create_redis_pool() -> RedisPool {
    let cfg = Config::from_url(env::var("REDIS_URL").expect("REDIS_URL not set."));
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
