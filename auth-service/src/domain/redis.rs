use std::ops::DerefMut;

use redis::Connection;

pub struct RedisConnection(pub Connection);

impl std::fmt::Debug for RedisConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RedisConnection").finish()
    }
}
impl AsRef<Connection> for RedisConnection {
    fn as_ref(&self) -> &Connection {
        return &self.0;
    }
}

impl AsMut<Connection> for RedisConnection {
    fn as_mut(&mut self) -> &mut Connection {
        return &mut self.0;
    }
}

pub fn make_redis_key(prefix: &str, suffix: &str) -> String {
    format!("{}_{}", prefix, suffix)
}
