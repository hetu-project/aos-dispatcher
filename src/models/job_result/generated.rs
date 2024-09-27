/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::job_request::JobRequest;
use crate::schema::*;

pub type ConnectionType = PgConnection;

/// Struct representing a row in table `job_result`
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::QueryableByName,
    diesel::Associations,
    diesel::Identifiable,
)]
#[diesel(table_name=job_result, primary_key(id), belongs_to(JobRequest, foreign_key=job_id))]
pub struct JobResult {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `job_id`
    pub job_id: String,
    /// Field representing column `operator`
    pub operator: String,
    /// Field representing column `result`
    pub result: serde_json::Value,
    /// Field representing column `vrf`
    pub vrf: serde_json::Value,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `signature`
    pub signature: String,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `job_result` for [`JobResult`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=job_result)]
pub struct CreateJobResult {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `job_id`
    pub job_id: String,
    /// Field representing column `operator`
    pub operator: String,
    /// Field representing column `result`
    pub result: serde_json::Value,
    /// Field representing column `vrf`
    pub vrf: serde_json::Value,
    /// Field representing column `verify_id`
    pub verify_id: String,
    /// Field representing column `tag`
    pub tag: String,
    /// Field representing column `clock`
    pub clock: serde_json::Value,
    /// Field representing column `signature`
    pub signature: String,
    /// Field representing column `job_type`
    pub job_type: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `job_result` for [`JobResult`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=job_result)]
pub struct UpdateJobResult {
    /// Field representing column `job_id`
    pub job_id: Option<String>,
    /// Field representing column `operator`
    pub operator: Option<String>,
    /// Field representing column `result`
    pub result: Option<serde_json::Value>,
    /// Field representing column `vrf`
    pub vrf: Option<serde_json::Value>,
    /// Field representing column `verify_id`
    pub verify_id: Option<String>,
    /// Field representing column `tag`
    pub tag: Option<String>,
    /// Field representing column `clock`
    pub clock: Option<serde_json::Value>,
    /// Field representing column `signature`
    pub signature: Option<String>,
    /// Field representing column `job_type`
    pub job_type: Option<String>,
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

impl JobResult {
    /// Insert a new row into `job_result` with a given [`CreateJobResult`]
    pub fn create(db: &mut ConnectionType, item: &CreateJobResult) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        diesel::insert_into(job_result)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `job_result`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        job_result.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `job_result`, identified by the primary key with [`UpdateJobResult`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: String,
        item: &UpdateJobResult,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::job_result::dsl::*;

        diesel::update(job_result.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `job_result`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::job_result::dsl::*;

        diesel::delete(job_result.filter(id.eq(param_id))).execute(db)
    }
}
