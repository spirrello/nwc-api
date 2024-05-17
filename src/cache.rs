use deadpool_redis::{Config, Pool, Runtime};
pub type RedisPool = Pool;

#[tokio::main]
pub async fn create_redis_pool(redis_url: String) -> RedisPool {
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
