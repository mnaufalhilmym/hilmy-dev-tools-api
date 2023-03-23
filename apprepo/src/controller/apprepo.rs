use std::str::FromStr;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tonic::{Request, Response, Result, Status};
use uuid::Uuid;

use crate::{
    model,
    proto::{self, apprepo::ApprepoService},
    schema,
};

use super::ApprepoController;

#[tonic::async_trait]
impl ApprepoService for ApprepoController {
    async fn get_apprepos(
        &self,
        _req: Request<proto::apprepo::GetAppreposReq>,
    ) -> Result<Response<proto::apprepo::Apprepos>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Get all apprepos
        let apprepos = schema::apprepo::table
            .load::<model::Apprepo>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::apprepo::Apprepos {
            apprepos: apprepos
                .iter()
                .map(|apprepo| proto::apprepo::Apprepo {
                    id: apprepo.id.to_string(),
                    name: apprepo.name.to_owned(),
                    icon: apprepo.icon.to_owned(),
                    link: apprepo.link.to_owned(),
                    created_at: apprepo.created_at.to_string(),
                    updated_at: apprepo.updated_at.to_string(),
                })
                .collect(),
        }))
    }

    async fn create_apprepo(
        &self,
        req: Request<proto::apprepo::CreateApprepoReq>,
    ) -> Result<Response<proto::apprepo::Apprepo>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Add an apprepo to database
        let apprepo = diesel::insert_into(schema::apprepo::table)
            .values((
                schema::apprepo::name.eq(&req.get_ref().name),
                schema::apprepo::icon.eq(&req.get_ref().icon),
                schema::apprepo::link.eq(&req.get_ref().link),
            ))
            .get_result::<model::Apprepo>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::apprepo::Apprepo {
            id: apprepo.id.to_string(),
            name: apprepo.name,
            icon: apprepo.icon,
            link: apprepo.link,
            created_at: apprepo.created_at.to_string(),
            updated_at: apprepo.updated_at.to_string(),
        }))
    }

    async fn update_apprepo(
        &self,
        req: Request<proto::apprepo::UpdateApprepoReq>,
    ) -> Result<Response<proto::apprepo::Apprepo>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Update the link
        let apprepo_id =
            Uuid::from_str(&req.get_ref().id).map_err(|e| Status::aborted(e.to_string()))?;
        let apprepo = diesel::update(schema::apprepo::table.find(&apprepo_id))
            .set((
                model::ApprepoChangeSet {
                    name: req.get_ref().name.to_owned(),
                    icon: req.get_ref().icon.to_owned(),
                    link: req.get_ref().link.to_owned(),
                },
                schema::apprepo::updated_at.eq(diesel::dsl::now),
            ))
            .get_result::<model::Apprepo>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::apprepo::Apprepo {
            id: apprepo.id.to_string(),
            name: apprepo.name,
            icon: apprepo.icon,
            link: apprepo.link,
            created_at: apprepo.created_at.to_string(),
            updated_at: apprepo.updated_at.to_string(),
        }))
    }

    async fn delete_apprepo(
        &self,
        req: Request<proto::apprepo::DeleteApprepoReq>,
    ) -> Result<Response<proto::apprepo::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Delete the apprepo
        let apprepo_id =
            Uuid::from_str(&req.get_ref().id).map_err(|e| Status::aborted(e.to_string()))?;
        diesel::delete(schema::apprepo::table.find(&apprepo_id))
            .execute(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::apprepo::OpRes { is_success: true }))
    }
}
