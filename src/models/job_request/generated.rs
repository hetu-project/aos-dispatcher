/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = PgConnection;

/// Struct representing a row in table `job_request`
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
#[diesel(table_name=job_request, primary_key(id))]
pub struct JobRequest {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `user`
    pub user: String,
    /// Field representing column `job`
    pub job: serde_json::Value,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `job_request` for [`JobRequest`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=job_request)]
pub struct CreateJobRequest {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `user`
    pub user: String,
    /// Field representing column `job`
    pub job: serde_json::Value,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `job_request` for [`JobRequest`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=job_request)]
pub struct UpdateJobRequest {
    /// Field representing column `user`
    pub user: Option<String>,
    /// Field representing column `job`
    pub job: Option<serde_json::Value>,
    /// Field representing column `clock`
    pub clock: Option<serde_json::Value>,
    /// Field representing column `job_type`
    pub job_type: Option<String>,
    /// Field representing column `status`
    pub status: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
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

impl JobRequest {
    /// Insert a new row into `job_request` with a given [`CreateJobRequest`]
    pub fn create(db: &mut ConnectionType, item: &CreateJobRequest) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        diesel::insert_into(job_request)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `job_request`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        job_request.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `job_request`, identified by the primary key with [`UpdateJobRequest`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: String,
        item: &UpdateJobRequest,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::job_request::dsl::*;

        diesel::update(job_request.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `job_request`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::job_request::dsl::*;

        diesel::delete(job_request.filter(id.eq(param_id))).execute(db)
    }
}
