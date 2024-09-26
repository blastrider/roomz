use crate::schema::users;
use diesel::{Insertable, PgConnection, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

impl User {
    // Méthode pour créer un utilisateur
    pub fn create(conn: &mut PgConnection, new_user: User) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
    }

    // Méthode pour lister tous les utilisateurs
    pub fn list_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        users.load::<User>(conn)
    }
}
