use crate::schema::users;
use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
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

    // Méthode pour mettre à jour un utilisateur
    pub fn update(conn: &mut PgConnection, user_id: Uuid, updated_user: User) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(user_id)))
            .set((name.eq(updated_user.name), role.eq(updated_user.role)))
            .get_result(conn)
    }

    // Méthode pour supprimer un utilisateur
    pub fn delete(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }
}
