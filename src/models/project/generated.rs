/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = PgConnection;

/// Struct representing a row in table `project`
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
#[diesel(table_name=project, primary_key(id))]
pub struct Project {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `token`
    pub token: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `project` for [`Project`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=project)]
pub struct CreateProject {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `token`
    pub token: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `project` for [`Project`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=project)]
pub struct UpdateProject {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `token`
    pub token: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
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

impl Project {
    /// Insert a new row into `project` with a given [`CreateProject`]
    pub fn create(db: &mut ConnectionType, item: &CreateProject) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        diesel::insert_into(project)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `project`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        project.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `project`, identified by the primary key with [`UpdateProject`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: String,
        item: &UpdateProject,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::project::dsl::*;

        diesel::update(project.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `project`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::project::dsl::*;

        diesel::delete(project.filter(id.eq(param_id))).execute(db)
    }
}
