use self::t_user::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

// 定义实体
#[derive(Queryable, Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "t_user"]
pub struct User {
    pub id: i32,
    pub name: String,
}

// 表查询映射
table! {
    t_user (id) {
        id -> Integer,
        name -> Text,
    }
}

pub fn insert(conn: &PooledConnection<ConnectionManager<SqliteConnection>>, user: &User)
              -> Result<usize, diesel::result::Error> {
    diesel::insert_into(t_user)
        .values(user)
        .execute(conn)
}


/*

/// Query the database, the database type uses generics; 查询数据库, 数据库类型使用泛型

pub fn select<Conn: Connection + 'static>(
    conn: &PooledConnection<ConnectionManager<Conn>>,
    offset: i64,
    limit: i64,
) -> Result<Vec<User>, diesel::result::Error> {
    t_user.offset(offset).limit(limit).load::<User>(conn)
}

*/


//查询数据库
pub fn select(
    conn: &PooledConnection<ConnectionManager<SqliteConnection>>,
    offset: i64,
    limit: i64,
) -> Result<Vec<User>, diesel::result::Error> {
    t_user.offset(offset).limit(limit).load::<User>(conn)
}
