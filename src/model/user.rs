use self::t_user::dsl::*;
use crate::DataBase;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

// 定义实体
#[derive(Queryable, Insertable, Debug)]
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

// orm
impl DataBase {
    //查询数据库
    pub fn user_select(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let conn: PooledConnection<ConnectionManager<SqliteConnection>> = self.pool.get()?;
        let result = t_user.offset(offset).limit(limit).load::<User>(&conn)?;
        //.filter(id.eq(100))
        Ok(result)
    }

    // 添加数据
    /*pub fn users_insert(&self, offset: i64, limit: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self.pool.get()?;
        //let result = t_user.values().load::<User>(&conn).is_ok()?;
        //.filter(id.eq(100))
        result
    }*/
}

//查询数据库
pub fn user_select2(
    conn: &PooledConnection<ConnectionManager<SqliteConnection>>,
    offset: i64,
    limit: i64,
) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let result = t_user.offset(offset).limit(limit).load::<User>(conn)?;
    //.filter(id.eq(100))
    Ok(result)
}
