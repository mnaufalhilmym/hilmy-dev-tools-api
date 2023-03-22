use std::str::FromStr;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tonic::{Request, Response, Result, Status};
use uuid::Uuid;

use crate::{
    model,
    proto::{self, link::LinkService},
    schema,
};

use super::LinkController;

#[tonic::async_trait]
impl LinkService for LinkController {
    async fn get_links(
        &self,
        req: Request<proto::link::GetLinksReq>,
    ) -> Result<Response<proto::link::Links>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Get all links created by user id
        let created_by_id = Uuid::from_str(&req.get_ref().created_by_id).unwrap();
        let links = schema::link::table
            .filter(schema::link::created_by_id.eq(&created_by_id))
            .load::<model::Link>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::link::Links {
            links: links
                .iter()
                .map(|link| proto::link::Link {
                    id: link.id.to_string(),
                    title: link.title.to_owned(),
                    short_url: link.short_url.to_owned(),
                    long_url: link.long_url.to_owned(),
                    visits: link.visits,
                    created_at: link.created_at.to_string(),
                    updated_at: link.updated_at.to_string(),
                })
                .collect(),
        }))
    }

    async fn get_link(
        &self,
        req: Request<proto::link::GetLinkReq>,
    ) -> Result<Response<proto::link::Link>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Get a link with an id
        let link_id =
            Uuid::from_str(&req.get_ref().id).map_err(|e| Status::internal(e.to_string()))?;
        let link = schema::link::table
            .find(&link_id)
            .first::<model::Link>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::link::Link {
            id: link.id.to_string(),
            title: link.title.to_owned(),
            short_url: link.short_url.to_owned(),
            long_url: link.long_url.to_owned(),
            visits: link.visits,
            created_at: link.created_at.to_string(),
            updated_at: link.updated_at.to_string(),
        }))
    }

    async fn create_link(
        &self,
        req: Request<proto::link::CreateLinkReq>,
    ) -> Result<Response<proto::link::Link>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Add link to database
        let created_by_id = Uuid::from_str(&req.get_ref().created_by_id).unwrap();
        let link = diesel::insert_into(schema::link::table)
            .values((
                schema::link::title.eq(&req.get_ref().title),
                schema::link::short_url.eq(&req.get_ref().short_url),
                schema::link::long_url.eq(&req.get_ref().long_url),
                schema::link::created_by_id.eq(&created_by_id),
            ))
            .get_result::<model::Link>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::link::Link {
            id: link.id.to_string(),
            title: link.title.to_owned(),
            short_url: link.short_url.to_owned(),
            long_url: link.long_url.to_owned(),
            visits: link.visits,
            created_at: link.created_at.to_string(),
            updated_at: link.updated_at.to_string(),
        }))
    }

    async fn update_link(
        &self,
        req: Request<proto::link::UpdateLinkReq>,
    ) -> Result<Response<proto::link::Link>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Check if the link is created by the id
        let link_id =
            Uuid::from_str(&req.get_ref().id).map_err(|e| Status::internal(e.to_string()))?;
        let created_by_id = Uuid::from_str(&req.get_ref().created_by_id).unwrap();
        let is_link_exist_and_created_by_id = diesel::select(diesel::dsl::exists(
            schema::link::table
                .find(&link_id)
                .filter(schema::link::created_by_id.eq(&created_by_id)),
        ))
        .get_result::<bool>(db_conn)
        .map_err(|e| Status::internal(e.to_string()))?;
        if !is_link_exist_and_created_by_id {
            return Err(Status::aborted("You are not the one who created the link."));
        }

        // Update the link
        let link = diesel::update(schema::link::table)
            .set(model::LinkChangeSet {
                title: req.get_ref().title.to_owned(),
                long_url: req.get_ref().long_url.to_owned(),
                short_url: req.get_ref().short_url.to_owned(),
            })
            .get_result::<model::Link>(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(proto::link::Link {
            id: link.id.to_string(),
            title: link.title.to_owned(),
            short_url: link.short_url.to_owned(),
            long_url: link.long_url.to_owned(),
            visits: link.visits,
            created_at: link.created_at.to_string(),
            updated_at: link.updated_at.to_string(),
        }))
    }

    async fn delete_link(
        &self,
        req: Request<proto::link::DeleteLinkReq>,
    ) -> Result<Response<proto::link::OpRes>> {
        let db_conn =
            &mut tools_lib_db::pg::connection::get_connection(&self.app_mode, &self.db_pool)
                .map_err(|e| Status::internal(e.to_string()))?;

        // Check if the link is created by the id
        let link_id =
            Uuid::from_str(&req.get_ref().id).map_err(|e| Status::internal(e.to_string()))?;
        let created_by_id = Uuid::from_str(&req.get_ref().created_by_id).unwrap();
        let is_link_exist_and_created_by_id = diesel::select(diesel::dsl::exists(
            schema::link::table
                .find(&link_id)
                .filter(schema::link::created_by_id.eq(&created_by_id)),
        ))
        .get_result::<bool>(db_conn)
        .map_err(|e| Status::internal(e.to_string()))?;
        if !is_link_exist_and_created_by_id {
            return Err(Status::aborted("You are not the one who created the link."));
        }

        // Delete the link
        diesel::delete(schema::link::table.find(&link_id))
            .execute(db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(proto::link::OpRes { is_success: true }))
    }
}
