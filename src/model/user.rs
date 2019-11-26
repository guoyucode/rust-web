pub use self::t_user::dsl::*;

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

