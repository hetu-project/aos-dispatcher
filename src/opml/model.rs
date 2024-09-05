use diesel::query_builder::AsQuery;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel::associations::HasTable;
use crate::schema::opml_questions;
use crate::tee::model::{deserialize_naive_datetime, serialize_naive_datetime};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpmlAnswer {
    pub req_id: String,
    pub node_id: String,
    pub model: String,
    pub prompt: String,
    pub answer: String,
    pub state_root: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct OpmlAnswerResponse {
    pub code: u16,
    pub result: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::opml_answers)]
pub struct PgOPMLAnswer {
    pub req_id: String,
    pub node_id: String,
    pub model: String,
    pub prompt: String,
    pub answer: String,
    pub state_root: String,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpmlRequest {
    pub model: String,
    pub prompt: String,
    pub req_id: String,
    pub callback: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpmlResponse {
    pub code: u16,
    pub msg: String,
    pub data: OpmlResponseData,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct OpmlResponseData {
    pub node_id: String,
    pub req_id: String,
}

#[derive(Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::opml_questions)]
pub struct PgOpmlQuestion {
    pub req_id: String,
    pub model: String,
    pub prompt: String,
    pub callback: String,
    #[serde(serialize_with = "serialize_naive_datetime", deserialize_with = "deserialize_naive_datetime")]
    pub created_at: NaiveDateTime,
}

pub fn create_opml_question(conn: &mut PgConnection, id: String, req: &OpmlRequest) -> Result<(), diesel::result::Error> {
    let new_opml_question = PgOpmlQuestion {
        req_id: id,
        model: req.model.clone(),
        prompt: req.prompt.clone(),
        callback: req.callback.clone(),
        created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(crate::schema::opml_questions::table)
        .values(&new_opml_question)
        .execute(conn)?;

    Ok(())
}


pub fn get_opml_question(conn: &mut PgConnection) -> Result<Vec<PgOpmlQuestion>, diesel::result::Error> {
    let r = opml_questions::table
        .select(PgOpmlQuestion::as_select())
        // .as_query()
        .load(conn);
    r
}


// pub fn get_opml_answer_by_id(conn: &mut PgConnection, opml_req_id: &str) -> Result<NewOpmlAnswer, diesel::result::Error> {
//     opml_answers
//         .filter(req_id.eq(opml_req_id))
//         .first::<NewOpmlAnswer>(conn)
// }
