use diesel::prelude::{SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub struct DataBase {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DataBase{
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        // 创建连接池
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("连接池创建失败.");
        DataBase {pool: pool.clone() }
    }
    /// Get a connection; 获得一个连接
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<SqliteConnection>>{
        self.pool.get().unwrap()
    }
}
