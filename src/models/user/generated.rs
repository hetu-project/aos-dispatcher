/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = PgConnection;

/// Struct representing a row in table `user`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    diesel::Identifiable,
)]
#[diesel(table_name=user, primary_key(id))]
pub struct User {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `count`
    pub count: i32,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `user` for [`User`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=user)]
pub struct CreateUser {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `count`
    pub count: i32,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `user` for [`User`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=user)]
pub struct UpdateUser {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `verify_id`
    pub verify_id: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
    /// Field representing column `count`
    pub count: Option<i32>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl User {
    /// Insert a new row into `user` with a given [`CreateUser`]
    pub fn create(db: &mut ConnectionType, item: &CreateUser) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::insert_into(user)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `user`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        user.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `user`, identified by the primary key with [`UpdateUser`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: String,
        item: &UpdateUser,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `user`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::user::dsl::*;

        diesel::delete(user.filter(id.eq(param_id))).execute(db)
    }
}
