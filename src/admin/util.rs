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
        let uuid = uuid::Uuid::new_v4();
        let token = uuid.to_string();
        let project = Project {
            id: format!("{}", req.address),
            name: req.name.clone(),
            address: req.address.clone(),
            token: format!("{}", token),
            status: format!("{}", "").into(),
            created_at: chrono::Local::now().naive_utc(),
        };
        let result = diesel::insert_into(schema::project::table)
            .values(&project)
            .returning(Project::as_returning())
            .get_result(conn)?;
        Ok(result)
    }

    async fn project_list(conn: &mut PgConnection) -> anyhow::Result<Vec<Project>> {
        let list = schema::project::table
            .select(Project::as_select())
            .load(conn)?;
        Ok(list)
    }
}
