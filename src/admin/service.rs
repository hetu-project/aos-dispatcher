use async_trait::async_trait;
use diesel::{query_dsl::methods::SelectDsl, PgConnection, RunQueryDsl, SelectableHelper};

use crate::{db::pg::model::Project, schema};

use super::model::RegisterProjectReq;

// #[async_trait]
pub trait AdminService {
    async fn register_project(
        conn: &mut PgConnection,
        req: &RegisterProjectReq,
    ) -> anyhow::Result<Project>;
    async fn project_list(conn: &mut PgConnection) -> anyhow::Result<Vec<Project>>;
}

pub struct Admin;

impl AdminService for Admin {
    async fn register_project(
        conn: &mut PgConnection,
        req: &RegisterProjectReq,
    ) -> anyhow::Result<Project> {
        let project = Project {
            id: req.address.clone(),
            name: req.name.clone(),
            address: req.address.clone(),
            status: "".into(),
            created_at: chrono::Local::now().naive_local(),
        };
        let result = diesel::insert_into(schema::project::table)
            .values(&project)
            .returning(Project::as_returning())
            .get_result(conn)?;
        async {}.await;
        Ok(result)
    }

    async fn project_list(conn: &mut PgConnection) -> anyhow::Result<Vec<Project>> {
        let list = schema::project::table
            .select(Project::as_select())
            .load(conn)?;
        async {}.await;
        Ok(list)
    }
}
