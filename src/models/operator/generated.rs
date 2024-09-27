/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = PgConnection;

/// Struct representing a row in table `operator`
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
#[diesel(table_name=operator, primary_key(id))]
pub struct Operator {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `start`
    pub start: String,
    /// Field representing column `end`
    pub end: String,
    /// Field representing column `operator_type`
    pub operator_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `operator` for [`Operator`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=operator)]
pub struct CreateOperator {
    /// Field representing column `id`
    pub id: String,
    /// Field representing column `name`
    pub name: String,
    /// Field representing column `address`
    pub address: String,
    /// Field representing column `start`
    pub start: String,
    /// Field representing column `end`
    pub end: String,
    /// Field representing column `operator_type`
    pub operator_type: String,
    /// Field representing column `status`
    pub status: String,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
}

/// Update Struct for a row in table `operator` for [`Operator`]
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default,
)]
#[diesel(table_name=operator)]
pub struct UpdateOperator {
    /// Field representing column `name`
    pub name: Option<String>,
    /// Field representing column `address`
    pub address: Option<String>,
    /// Field representing column `start`
    pub start: Option<String>,
    /// Field representing column `end`
    pub end: Option<String>,
    /// Field representing column `operator_type`
    pub operator_type: Option<String>,
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

impl Operator {
    /// Insert a new row into `operator` with a given [`CreateOperator`]
    pub fn create(db: &mut ConnectionType, item: &CreateOperator) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        diesel::insert_into(operator)
            .values(item)
            .get_result::<Self>(db)
    }

    /// Get a row from `operator`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        operator.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `operator`, identified by the primary key with [`UpdateOperator`]
    pub fn update(
        db: &mut ConnectionType,
        param_id: String,
        item: &UpdateOperator,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::operator::dsl::*;

        diesel::update(operator.filter(id.eq(param_id)))
            .set(item)
            .get_result(db)
    }

    /// Delete a row in `operator`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: String) -> diesel::QueryResult<usize> {
        use crate::schema::operator::dsl::*;

        diesel::delete(operator.filter(id.eq(param_id))).execute(db)
    }
}
