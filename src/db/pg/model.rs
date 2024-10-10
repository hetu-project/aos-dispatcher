use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn serialize_naive_datetime<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::operator)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Operator {
    pub id: String,
    pub name: String,
    pub address: String,
    pub start: String,
    pub end: String,
    pub operator_type: String,
    pub status: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Clone)]
#[diesel(table_name = crate::schema::job_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobRequest {
    pub id: String,
    pub job: Value,
    pub user: String,
    pub job_type: String,
    pub status: String,
    pub tag: String,
    pub clock: Value,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::job_result)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobResult {
    pub id: String,
    pub job_id: String,
    pub operator: String,
    pub result: Value,
    pub verify_id: String,
    pub vrf: Value,
    pub clock: Value,
    pub signature: String,
    pub job_type: String,
    pub tag: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::project)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: String,
    pub name: String,
    pub address: String,
    pub status: String,
    pub token: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub address: String,
    pub verify_id: String,
    pub status: String,
    pub count: i32,
    pub tag: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}
