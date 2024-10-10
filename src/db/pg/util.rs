use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::upsert::excluded;
use serde::Deserialize;

use crate::schema::{self, job_request, job_result, operator};

use super::model::{JobRequest, JobResult, Operator, User};

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

pub fn create_operator(
    conn: &mut PgConnection,
    op: &Operator,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(crate::schema::operator::table)
        .values(op)
        // .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn sync_operators_info(
    conn: &mut PgConnection,
    operators: &Vec<Operator>,
) -> Result<Vec<Operator>, diesel::result::Error> {
    diesel::insert_into(crate::schema::operator::table)
        .values(operators)
        // .on_conflict(target)
        .on_conflict(crate::schema::operator::id)
        .do_update()
        .set((
            crate::schema::operator::start.eq(excluded(crate::schema::operator::start)),
            crate::schema::operator::end.eq(excluded(crate::schema::operator::end)),
        ))
        .returning(Operator::as_returning())
        .get_results(conn)
    // .expect("Error saving new question")
}

pub fn create_job_request(
    conn: &mut PgConnection,
    q: &JobRequest,
) -> Result<JobRequest, diesel::result::Error> {
    diesel::insert_into(crate::schema::job_request::table)
        .values(q)
        .returning(JobRequest::as_returning())
        .get_result(conn)
    // .expect("Error saving new question")
}

pub fn update_job_request_status(
    conn: &mut PgConnection,
    q: &JobRequest,
) -> Result<JobRequest, diesel::result::Error> {
    diesel::update(crate::schema::job_request::table)
        .filter(job_request::id.eq(q.id.clone()))
        .set(job_request::status.eq("dispatched"))
        .returning(JobRequest::as_returning())
        .get_result(conn)
    // .expect("Error saving new question")
}

pub fn create_job_result(
    conn: &mut PgConnection,
    ans: &JobResult,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(crate::schema::job_result::table)
        .values(ans)
        .execute(conn)?;
    Ok(())
}

// pub fn get_job_request_by_id(conn: &mut PgConnection, q_id: &str) -> Result<Vec<JobRequest>, diesel::result::Error> {
//   job_request::table
//   .filter(job_request::id.eq(q_id))
// }

pub fn get_job_result_by_id(
    conn: &mut PgConnection,
    q_id: &str,
) -> Result<Option<JobResult>, diesel::result::Error> {
    job_result::table
        .select(JobResult::as_select())
        .filter(job_result::id.eq(q_id))
        .first::<JobResult>(conn)
        .optional()
}

pub fn get_job_results_by_job_id(
    conn: &mut PgConnection,
    q_id: &str,
) -> Result<Vec<JobResult>, diesel::result::Error> {
    job_result::table
        .select(JobResult::as_select())
        .filter(job_result::job_id.eq(q_id))
        .load(conn)
}

pub fn get_job_request_by_job_id(
    conn: &mut PgConnection,
    q_id: &str,
) -> Result<JobRequest, diesel::result::Error> {
    job_request::table
        .select(JobRequest::as_select())
        .filter(job_request::id.eq(q_id))
        .first(conn)
}

pub fn get_job_verify_by_user_id(
    conn: &mut PgConnection,
    id: &str,
) -> Result<Vec<JobResult>, diesel::result::Error> {
    job_result::table
        .left_join(job_request::table)
        .select(JobResult::as_select())
        .filter(job_request::user.eq(id))
        .order(job_result::created_at.desc())
        .limit(100)
        .load(conn)
}

pub fn query_new_job_request(
    conn: &mut PgConnection,
) -> Result<Vec<JobRequest>, diesel::result::Error> {
    let r = job_request::table
        .select(JobRequest::as_select())
        .order(job_request::created_at.desc())
        .filter(job_request::status.eq_any(["", "created"]))
        // .as_query()
        .load(conn);
    r
}

pub fn query_oldest_job_request_with_user(
    conn: &mut PgConnection,
    user: &str,
) -> Result<Vec<JobRequest>, diesel::result::Error> {
    let r = job_request::table
        .select(JobRequest::as_select())
        .filter(job_request::user.eq(user))
        .order(job_request::created_at.desc())
        .limit(10)
        // .as_query()
        .load(conn);
    r
}

pub fn query_operators(conn: &mut PgConnection) -> Result<Vec<Operator>, diesel::result::Error> {
    let r = operator::table
        .select(Operator::as_select())
        // .as_query()
        .load(conn);
    r
}

pub fn get_operator_by_id(
    conn: &mut PgConnection,
    q_id: &str,
) -> Result<Operator, diesel::result::Error> {
    operator::table
        .filter(operator::id.eq(q_id))
        .first::<Operator>(conn)
}

pub fn create_user(conn: &mut PgConnection, q: &User) -> Result<User, diesel::result::Error> {
    diesel::insert_into(crate::schema::user::table)
        .values(q)
        .on_conflict(schema::user::id)
        .do_update()
        .set((
            schema::user::tag.eq(&q.tag),
            schema::user::count.eq(&q.count),
        ))
        .returning(User::as_returning())
        .get_result(conn)
    // .expect("Error saving new question")
}

pub fn update_user(conn: &mut PgConnection, q: &User) -> Result<User, diesel::result::Error> {
    diesel::update(crate::schema::user::table)
        .filter(crate::schema::user::id.eq(q.id.clone()))
        .set(crate::schema::user::tag.eq(q.tag.clone()))
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, id: &str) -> Result<User, diesel::result::Error> {
    schema::user::table
        .select(User::as_select())
        .filter(schema::user::id.eq(id))
        .first::<User>(conn)
}
