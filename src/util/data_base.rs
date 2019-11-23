use diesel::prelude::{SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};

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
    //pub fn getpool
}
